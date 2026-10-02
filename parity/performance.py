"""RFC 6.1 geometry matrix; imported by runner.py, no external packages."""
import array
import contextlib
import select
import subprocess
import json
import math
import os
import statistics
import struct
import time
import zipfile


class Paired:
    def __init__(self, r, binary, path):
        self.process = subprocess.Popen([str(binary), str(path)], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=r.ENV)
        os.sched_setaffinity(self.process.pid, {max(os.sched_getaffinity(0))})
        self.count = 0
        try:
            if self.read(1) != b'R': raise ValueError('driver did not finish warm-up')
        except BaseException:
            self.close()
            raise

    def read(self, count):
        if not select.select([self.process.stdout], [], [], 180)[0]:
            raise ValueError('paired driver timeout')
        data = self.process.stdout.read(count)
        if len(data) != count:
            raise ValueError('short paired response: '+self.process.stderr.read().decode(errors='replace'))
        return data

    def sample(self):
        if self.count >= 99: raise ValueError('paired retry budget exhausted')
        self.process.stdin.write(b'R'); self.process.stdin.flush()
        self.count += 1
        data = self.read(9)
        if data[:1] != b'T': raise ValueError(f'invalid sample frame: {data.hex()}')
        return struct.unpack('<d', data[1:])[0]

    def finish(self):
        out, err = self.process.communicate(input=b'S', timeout=180)
        if self.process.returncode: raise ValueError(f'paired driver failed: {err!r}')
        return out

    def close(self):
        if self.process.poll() is None:
            self.process.kill(); self.process.wait()
        for stream in [self.process.stdin, self.process.stdout, self.process.stderr]: stream.close()


def geometry(r, triangles, shape):
    if shape in {'seam-heavy', 'disconnected'}:
        # Independent grid patches. Seam patches share boundary positions;
        # disconnected patches are translated apart. Interiors remain collapsible.
        p, ib = array.array('f'), array.array('I')
        remaining = triangles
        patch = 0
        while remaining:
            n = min(8 if triangles == 32 else 128, remaining)
            pp, ii = r.grid(n)
            side = math.ceil(math.sqrt(n / 2))
            ox = (patch % 128) * (side if shape == 'seam-heavy' else side + 4)
            oy = (patch // 128) * (side if shape == 'seam-heavy' else side + 4)
            base = len(p) // 3
            for v in range(len(pp)//3):
                x, y = pp[v*3] + ox, pp[v*3+1] + oy
                p.extend((x, y, (x*x+y*y)*0.00001))
            ib.extend(base + i for i in ii)
            remaining -= n
            patch += 1
    else:
        p, ib = r.grid(triangles)
        for v in range(len(p)//3):
            x, y = p[v*3], p[v*3+1]
            p[v*3+2] = (x*x+y*y)*0.00001
        if shape == 'sparse':
            # Half the supplied records unused, with referenced IDs interspersed.
            pp = array.array('f')
            for v in range(len(p)//3):
                pp.extend(p[v*3:v*3+3])
                pp.extend((0., 0., 0.))
            p = pp
            ib = array.array('I', (i*2 for i in ib))
    return p, ib


def workloads():
    for size, triangles in [('tiny', 32), ('medium', 8192), ('million', 1000000)]:
        for shape in ['smooth', 'seam-heavy', 'disconnected', 'sparse']:
            yield size, triangles, shape


def configurations():
    yield 1, None, 0
    yield 2, None, 0
    yield 6, None, 0
    for ratio in [.5, .25, .125]:
        yield 4, ratio, 0
    for ac, ratio in [(1, .5), (8, .25), (12, .125)]:
        yield 5, ratio, ac


def stats(samples):
    return {name: {'median_seconds': statistics.median(ts), 'min_seconds': min(ts),
                   'max_seconds': max(ts), 'stdev_seconds': statistics.stdev(ts),
                   'quartiles_seconds': statistics.quantiles(ts, n=4), 'raw_seconds': ts}
            for name, ts in samples.items()}


def run(r, args):
    reference, target, results = r.paths()
    binaries, identities = r.build(reference, target, wasm=False)
    record = {'schema': 2, 'command': 'benchmark', 'identities': identities,
              'started_unix': time.time(), 'load_start': os.getloadavg(), 'threads': 1,
              'cpu_affinity': max(os.sched_getaffinity(0)),
              'samples': 10, 'warmups_per_backend_attempt': 1, 'tiny_calls_per_sample': 64,
              'paired_protocol': 'two resident drivers: R ready after warmup, R triggers one sample returning T plus f64, S returns final MR01 response; backend order alternates each pair',
              'timing': 'validation, required copies, output allocation (allocating API), scratch and algorithm; excludes serialization, I/O, input generation and process startup',
              'memory': 'peak requested output plus scratch; caller output excluded on both sides; Rust retained workspace capacity included; C++ allocator hook during untimed warmup',
              'cpp_reference': 'scalar-strict; these upstream modules have no explicit SIMD paths',
              'workloads': {}, 'passed': False}
    artifact = r.artifacts_path() / 'benchmark-buffers.zip'
    with zipfile.ZipFile(artifact, 'w', zipfile.ZIP_DEFLATED, compresslevel=1) as archive:
        for size, triangles, shape in workloads():
            p, ib = geometry(r, triangles, shape)
            cached = None
            for op, ratio, ac in configurations():
                topology = ib
                if op == 2:
                    if cached is None:
                        raw, _ = r.response(r.execute(binaries['rust'], r.message(1, p, ib)), len(ib))
                        cached = array.array('I'); cached.frombytes(raw)
                        if r.sys.byteorder != 'little': cached.byteswap()
                    topology = cached
                for api in (['allocation-free'] if op == 6 else ['allocating', 'caller-buffer']):
                    name = f'{r.FAMILIES[op]}/{size}/{shape}/{api}/a{ac}/r{ratio}'
                    data = bytearray(r.message(op, p, topology, mode=2 if api == 'caller-buffer' else 1,
                                               samples=100, variant={0:0, 1:1, 8:4, 12:5}[ac]))
                    if ratio is not None:
                        struct.pack_into('<IfI', data, 28+len(p)*4+len(topology)*4,
                                         int(len(topology)*ratio), max(1-ratio,.05), 32 if op==5 else 0)
                    data = bytes(data)
                    attempts = []
                    output_records = []
                    expected = None
                    memory = {}
                    path = target/'paired-benchmark.input'
                    path.write_bytes(data)
                    for attempt in range(2):
                        start_load = os.getloadavg()
                        samples = {'rust': [], 'cpp': []}
                        invalid_times = []
                        backend_order = []
                        with contextlib.ExitStack() as cleanup:
                            processes = {}
                            for backend in ['rust', 'cpp']:
                                processes[backend] = Paired(r, binaries[backend], path)
                                cleanup.callback(processes[backend].close)
                            pair = 0
                            target_samples = 10
                            while pair < target_samples:
                                for backend in (['rust', 'cpp'] if pair % 2 == 0 else ['cpp', 'rust']):
                                    for retry in range(4):
                                        seconds = processes[backend].sample()
                                        backend_order.append(backend)
                                        if math.isfinite(seconds) and seconds > 0: break
                                        invalid_times.append({'backend': backend, 'pair': pair, 'raw_seconds': [seconds]})
                                    else: raise ValueError(f'nonpositive clock intervals after four attempts: {name}: {invalid_times!r}')
                                    samples[backend].append(seconds)
                                pair += 1
                                if pair == target_samples and pair < 30:
                                    ratios = sorted(a/b for a,b in zip(samples['rust'], samples['cpp']))
                                    lo, hi = ratios[len(ratios)//4], ratios[3*len(ratios)//4]
                                    if any(lo <= bar <= hi for bar in [1.25, 1.50]): target_samples += 10
                            for backend in ['rust', 'cpp']:
                                raw = processes[backend].finish()
                                mem, = struct.unpack('<Q', raw[-8:])
                                out, times = r.response(raw[:-8], len(topology) if op <= 2 else None, processes[backend].count)
                                if expected is None: expected = out
                                if out != expected: raise ValueError(f'benchmark output mismatch: {name}')
                                positive = [t for t in times if math.isfinite(t) and t>0]
                                if positive != samples[backend]: raise ValueError('paired sample stream mismatch')
                                memory[backend] = mem
                                output_records.append(r.retain(archive, name+f'/{attempt}-{backend}.output', raw))
                        end_load = os.getloadavg()
                        attempts.append({'load_start': start_load, 'load_end': end_load, 'stats': stats(samples),
                                         'high_load': max(start_load[0], end_load[0]) > 4,
                                         'invalid_clock_samples': invalid_times, 'sample_backend_order': backend_order})
                        if not attempts[-1]['high_load']: break
                    path.unlink()
                    st = attempts[-1]['stats']
                    time_ratio = st['rust']['median_seconds']/st['cpp']['median_seconds']
                    memory_ratio = memory['rust']/memory['cpp'] if memory['cpp'] else (1. if memory['rust']==0 else None)
                    work = {'family': r.FAMILIES[op], 'size': size, 'shape': shape, 'triangles': triangles,
                            'vertices': len(p)//3, 'api': api, 'attributes': ac, 'target_ratio': ratio,
                            'stats': st, 'attempts': attempts, 'rust_cpp_ratio': time_ratio,
                            'memory_bytes': memory, 'memory_ratio': memory_ratio,
                            'input': r.retain(archive, name+'.input', data), 'outputs': output_records,
                            'passed': time_ratio <= 1.5 and memory_ratio is not None and memory_ratio <= 1.25}
                    record['workloads'][name] = work
                    print(f'{name}: time {time_ratio:.3f}, memory {memory_ratio}, load {attempts[-1]["load_end"][0]:.2f}', flush=True)
                    # Keep recoverable partial results; never mark an incomplete run passing.
                    (results/'benchmark.partial.json').write_text(json.dumps(record, indent=2)+'\n')
    r.check_unchanged(reference, binaries, identities)
    record['families'] = {}
    for family in r.FAMILIES.values():
        cases = [w for w in record['workloads'].values() if w['family']==family]
        gm = math.exp(statistics.mean(math.log(w['rust_cpp_ratio']) for w in cases))
        maximum = max(w['rust_cpp_ratio'] for w in cases)
        memory = max(w['memory_ratio'] if w['memory_ratio'] is not None else math.inf for w in cases)
        record['families'][family] = {'cases': len(cases), 'geometric_mean': gm, 'maximum': maximum,
                                      'maximum_memory_ratio': memory, 'passed': gm<=1.25 and maximum<=1.5 and memory<=1.25}
    record.update(passed=all(f['passed'] for f in record['families'].values()), finished_unix=time.time(),
                  load_end=os.getloadavg(), artifacts={artifact.name:r.sha(artifact)})
    (results/'benchmark.json').write_text(json.dumps(record, indent=2)+'\n')
    (results/'benchmark.partial.json').unlink()
    write_report(r, record, results)
    if args.enforce and not record['passed']: raise SystemExit('RFC performance gate failed')


def write_report(r, record, results):
    lines = ['# Measured geometry performance', '',
             'Single-thread RFC 6.1 matrix. One warm-up and at least ten alternating paired samples per attempt.',
             'High-load cases are repeated once; both attempts and all raw samples are retained. Persistent load above 4 limits timing confidence.', '',
             '| Family | Cases | Geometric mean | Maximum | Maximum memory ratio | Verdict |',
             '|---|---:|---:|---:|---:|---|']
    for family, f in record['families'].items():
        line = f'| {family} | {f["cases"]} | {f["geometric_mean"]:.3f} | {f["maximum"]:.3f} | {f["maximum_memory_ratio"]:.3f} | {"PASS" if f["passed"] else "FAIL"} |'
        lines.append(line); print(line, flush=True)
    lines += ['', f'Overall: **{"PASS" if record["passed"] else "FAIL"}**. Required: geometric mean ≤1.25, every case ≤1.50, memory ≤1.25.',
              f'Load average start: {record["load_start"]}; end: {record["load_end"]}.', '',
              'Scale has one allocation-free API, so it is measured once per geometry. Caller-buffer measurements reuse output and Workspace after warm-up; allocating measurements use fresh scratch.',
              'Seam-heavy meshes have duplicate patch boundaries and discontinuous attributes; disconnected meshes have separated patches; sparse meshes reference half the supplied vertices.',
              'Attribute counts 1, 8 and 12 are paired with target ratios 0.5, 0.25 and 0.125; plain simplification covers all three ratios on every geometry.',
              'Raw samples, dispersion, per-attempt load, identities and per-case memory are in `results/benchmark.json`.', '',
              '| Case | Rust ms | C++ ms | Time ratio | Memory ratio |', '|---|---:|---:|---:|---:|']
    for name,w in record['workloads'].items():
        lines.append(f'| {name} | {w["stats"]["rust"]["median_seconds"]*1000:.6f} | {w["stats"]["cpp"]["median_seconds"]*1000:.6f} | {w["rust_cpp_ratio"]:.3f} | {w["memory_ratio"]:.3f} |')
    (results/'MEASURED_PERFORMANCE.md').write_text('\n'.join(lines)+'\n')
    if results == r.ROOT/'parity/results': (r.ROOT/'parity/MEASURED_PERFORMANCE.md').write_text('\n'.join(lines)+'\n')
