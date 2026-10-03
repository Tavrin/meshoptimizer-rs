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
import quiet


class Paired:
    def __init__(self, r, binary, path):
        self.process = subprocess.Popen(['taskset', '-c', str(r.benchmark_cpu), str(binary), str(path)], stdin=subprocess.PIPE,
                                        stdout=subprocess.PIPE, stderr=subprocess.PIPE, env=r.ENV)
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


def paired_stats(samples):
    ratios = [a/b for a,b in zip(samples['rust'], samples['cpp'], strict=True)]
    median = statistics.median(ratios)
    return {'median': median, 'min': min(ratios), 'max': max(ratios),
            'quartiles': statistics.quantiles(ratios, n=4),
            'stdev': statistics.stdev(ratios),
            'coefficient_of_variation': statistics.stdev(ratios)/statistics.mean(ratios),
            'median_absolute_deviation': statistics.median(abs(x-median) for x in ratios),
            'raw_ratios': ratios}


def run(r, args):
    reference, target, results = r.paths()
    binaries, identities = r.build(reference, target, wasm=False)
    core = quiet.physical_core()
    r.benchmark_cpu = core['cpu']
    record = {'schema': 4, 'command': 'benchmark', 'identities': identities,
              'started_unix': time.time(), 'load_start': os.getloadavg(), 'threads': 1,
              'cpu_affinity': core['cpu'], 'physical_core': core,
              'load_policy': 'coordinator correction: no quiet admission or load-based repeats; least-busy physical core measured with /proc/stat; retain load before/after every pair',
              'ratio_metric': 'median of same-core interleaved Rust/C++ paired ratios; family geometric mean of these case medians',
              'samples': 20, 'warmups_per_backend_attempt': 1, 'tiny_calls_per_sample': 64,
              'paired_protocol': 'two resident drivers: R ready after warmup, R triggers one sample returning T plus f64, S returns final MR01 response; backend order alternates each pair',
              'timing': 'validation, required copies, output allocation (allocating API), scratch and algorithm; excludes serialization, I/O, input generation and process startup',
              'memory': 'peak requested output plus scratch; caller output excluded on both sides; Rust retained workspace capacity included; C++ allocator hook during untimed warmup',
              'cpp_reference': 'scalar-strict; these upstream modules have no explicit SIMD paths',
              'workloads': {}, 'passed': False, 'completed': False}
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
                    for attempt in range(1):
                        start_load = quiet.load()
                        sample_loads = []
                        samples = {'rust': [], 'cpp': []}
                        invalid_times = []
                        backend_order = []
                        with contextlib.ExitStack() as cleanup:
                            processes = {}
                            for backend in ['rust', 'cpp']:
                                processes[backend] = Paired(r, binaries[backend], path)
                                cleanup.callback(processes[backend].close)
                            pair = 0
                            target_samples = 20
                            while pair < target_samples:
                                sample_loads.append({'pair': pair, 'before': quiet.load()})
                                for backend in (['rust', 'cpp'] if pair % 2 == 0 else ['cpp', 'rust']):
                                    for retry in range(4):
                                        seconds = processes[backend].sample()
                                        backend_order.append(backend)
                                        if math.isfinite(seconds) and seconds > 0: break
                                        invalid_times.append({'backend': backend, 'pair': pair, 'raw_seconds': [seconds]})
                                    else: raise ValueError(f'nonpositive clock intervals after four attempts: {name}: {invalid_times!r}')
                                    samples[backend].append(seconds)
                                pair += 1
                                sample_loads[-1]['after'] = quiet.load()
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
                        end_load = quiet.load()
                        attempts.append({'load_start': start_load, 'load_end': end_load, 'stats': stats(samples),
                                         'high_load': max(start_load[0], end_load[0]) > 4,
                                         'sample_loads': sample_loads,
                                         'invalid_clock_samples': invalid_times, 'sample_backend_order': backend_order})
                    path.unlink()
                    st = attempts[-1]['stats']
                    dispersion = paired_stats(samples)
                    time_ratio = dispersion['median']
                    memory_ratio = memory['rust']/memory['cpp'] if memory['cpp'] else (1. if memory['rust']==0 else None)
                    work = {'family': r.FAMILIES[op], 'size': size, 'shape': shape, 'triangles': triangles,
                            'vertices': len(p)//3, 'api': api, 'attributes': ac, 'target_ratio': ratio,
                            'stats': st, 'attempts': attempts, 'rust_cpp_ratio': time_ratio,
                            'memory_bytes': memory, 'memory_ratio': memory_ratio,
                            'input': r.retain(archive, name+'.input', data), 'outputs': output_records,
                            'paired_ratio_stats': dispersion,
                            'ratio_of_medians': st['rust']['median_seconds']/st['cpp']['median_seconds'],
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
    record.update(completed=True, identities_unchanged=True,
                  passed=all(f['passed'] for f in record['families'].values()), finished_unix=time.time(),
                  load_end=os.getloadavg(), artifacts={artifact.name:r.sha(artifact)})
    (results/'benchmark.json').write_text(json.dumps(record, indent=2)+'\n')
    (results/'benchmark.partial.json').unlink()
    write_report(r, record, results)
    if args.enforce and not record['passed']: raise SystemExit('RFC performance gate failed')


def write_report(r, record, results):
    if record.get('schema') == 3 and not record['completed']:
        checks = [c for w in record['quiet_waits'] for c in w['checks']]
        lines = ['# Measured geometry performance', '',
                 'Lane 3b quiet measurement: **BLOCKED**. No complete qualified matrix.', '',
                 record['blocked_reason']+'.',
                 f'Pinned CPU: {record["cpu_affinity"]}; physical core: {record["physical_core"]["core_id"]}; siblings: {record["physical_core"]["thread_siblings"]}.',
                 f'Admission: /proc/loadavg, one-minute load strictly below 1.5; checks every 30 seconds for at most 3600 seconds.',
                 f'Recorded {len(checks)} load checks; minimum {min(c["load_average"][0] for c in checks):.2f}; maximum {max(c["load_average"][0] for c in checks):.2f}.',
                 f'Total admission wait: {sum(w["waited_seconds"] for w in record["quiet_waits"]):.1f} seconds.', '',
                 'Before/after geometric means and maxima are unavailable under the required quiet conditions. The RFC bar remains unqualified.',
                 'No safe-Rust performance ceiling has been established. Historical high-load ratios are retained separately in `benchmark-lane3-high-load.json`.',
                 'Full waits, load averages and source/binary identities are in `results/benchmark.json`.']
        report = '\n'.join(lines)+'\n'
        (results/'MEASURED_PERFORMANCE.md').write_text(report)
        if results == r.ROOT/'parity/results': (r.ROOT/'parity/MEASURED_PERFORMANCE.md').write_text(report)
        return
    lines = ['# Measured geometry performance', '',
             'Single-thread RFC 6.1 matrix. One warm-up and at least twenty alternating same-core paired samples per case.' if record.get('schema') == 4 else
             'Single-thread RFC 6.1 matrix. One warm-up and at least ten alternating paired samples per attempt.',
             'Coordinator correction: measure immediately on a core selected from three one-second /proc/stat intervals. Ratios are medians of paired Rust/C++ samples; load does not gate or repeat measurements.' if record.get('schema') == 4 else
             'Lane 3b admits each attempt below one-minute load 1.5, polling /proc/loadavg every 30 seconds for up to 60 minutes. Nonquiet attempts repeat once; all attempts are retained.' if record.get('schema') == 3 else
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
              '| Case | Rust ms | C++ ms | Paired time ratio | Ratio IQR | Ratio CV | Memory ratio |' if record.get('schema') == 4 else
              '| Case | Rust ms | C++ ms | Time ratio | Memory ratio |',
              '|---|---:|---:|---:|---|---:|---:|' if record.get('schema') == 4 else '|---|---:|---:|---:|---:|']
    if record.get('schema') == 4:
        lines[4:4] = [f'Pinned CPU: {record["cpu_affinity"]}; physical core {record["physical_core"]["core_id"]}; siblings {record["physical_core"]["thread_siblings"]}; selected logical CPU busy {record["physical_core"]["busy_percent"]:.2f}%.',
                      'Raw paired ratios, IQR, standard deviation, median absolute deviation, coefficient of variation and per-pair loads are retained in `results/benchmark.json`.', '']
    if record.get('schema') == 3:
        lines[4:4] = [f'Pinned CPU: {record["cpu_affinity"]}; physical core {record["physical_core"]["core_id"]}; siblings {record["physical_core"]["thread_siblings"]}.',
                      f'Quiet qualification: {record["quiet_qualified"]}; admission waits {len(record["quiet_waits"])}; total waited {sum(w["waited_seconds"] for w in record["quiet_waits"]):.1f} seconds.', '']
    for name,w in record['workloads'].items():
        if record.get('schema') == 4:
            d = w['paired_ratio_stats']
            lines.append(f'| {name} | {w["stats"]["rust"]["median_seconds"]*1000:.6f} | {w["stats"]["cpp"]["median_seconds"]*1000:.6f} | {w["rust_cpp_ratio"]:.3f} | {d["quartiles"][0]:.3f}–{d["quartiles"][2]:.3f} | {d["coefficient_of_variation"]:.3f} | {w["memory_ratio"]:.3f} |')
        else:
            lines.append(f'| {name} | {w["stats"]["rust"]["median_seconds"]*1000:.6f} | {w["stats"]["cpp"]["median_seconds"]*1000:.6f} | {w["rust_cpp_ratio"]:.3f} | {w["memory_ratio"]:.3f} |')
    (results/'MEASURED_PERFORMANCE.md').write_text('\n'.join(lines)+'\n')
    if results == r.ROOT/'parity/results': (r.ROOT/'parity/MEASURED_PERFORMANCE.md').write_text('\n'.join(lines)+'\n')
