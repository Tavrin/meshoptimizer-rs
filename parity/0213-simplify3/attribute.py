"""Resumable instruction sampling and annotation of both release arms."""
from pathlib import Path
import hashlib
import json
import os
import subprocess

ART = Path(os.environ['MESHOPT_SCRATCH'])
PERF = os.environ['MESHOPT_PERF']
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'
assert subprocess.check_output([PERF, '--version'], text=True).startswith('perf version 6.17')
OUT = ART / os.environ.get('MESHOPT_PROFILE_LABEL', 'attribution')
OUT.mkdir(exist_ok=True)
records = OUT / 'records.jsonl'
rows = [json.loads(s) for s in records.read_text().splitlines()] if records.exists() else []
keys = {(r['case'], r['arm']) for r in rows}
for cell in json.loads((ART/'stages.json').read_text()):
    for arm in ['base', 'M7']:
        if (cell['case'], arm) in keys:
            continue
        prefix = OUT / f"{cell['case']}-{arm}"
        leftovers = list(OUT.glob(prefix.name+'.*'))
        if leftovers:
            import tempfile
            retained = Path(tempfile.mkdtemp(prefix=prefix.name+'-',dir=OUT))
            for path in leftovers:
                path.rename(retained/path.name)
        binary = ART / f'driver-{arm}'
        row = dict(case=cell['case'], family=cell['family'], arm=arm,
                   admission=os.environ['PARITY_RUNNER_ACTIVE'],
                   binary_sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
        for mode in ['stat', 'record']:
            target = prefix.with_suffix('.perf' if mode == 'stat' else '.data')
            assert not target.exists()
            cr,cw = os.pipe()
            ar,aw = os.pipe()
            cmd = [PERF,mode,'-e','instructions:u','-o',str(target)]
            cmd += ['-x',';'] if mode == 'stat' else ['-c',os.environ.get('MESHOPT_PROFILE_PERIOD','1000000')]
            cmd += ['-D','-1','--control',f'fd:{cr},{aw}','--','taskset','-c','9',str(binary),
                    'simplify',cell['input'],str(prefix.with_suffix('.output'))]
            try:
                result = subprocess.run(cmd, env=dict(os.environ,PROBE_CTL=str(cw),PROBE_ACK=str(ar)),
                    input='1\nstop\n',pass_fds=(cr,cw,ar,aw),capture_output=True,text=True,timeout=300)
            finally:
                for fd in [cr,cw,ar,aw]:
                    os.close(fd)
            prefix.with_suffix('.'+mode+'.stderr').write_text(result.stderr)
            row[mode+'_exit'] = result.returncode
            row['sampling_period'] = int(os.environ.get('MESHOPT_PROFILE_PERIOD','1000000'))
            assert result.returncode == 0 and result.stdout == 'READY\nCOUNTED\n', (cmd,result.stderr,result.stdout)
            if mode == 'stat':
                fields = next(s for s in target.read_text().splitlines() if ';instructions:u;' in s).split(';')
                row['instructions'] = int(fields[0])
                assert float(fields[4]) == 100.0
            else:
                dump = subprocess.run([PERF, 'script', '-D', '-i', str(target)], capture_output=True, text=True, timeout=180)
                assert dump.returncode == 0, dump.stderr
                throttles = [line for line in dump.stdout.splitlines() if 'PERF_RECORD_THROTTLE' in line or 'PERF_RECORD_LOST' in line]
                prefix.with_suffix('.sampling-check').write_text('\n'.join(throttles)+'\n')
                assert not throttles, ('sampling throttled or lost', prefix, len(throttles))
                for action,args in [('report',['--no-children','--sort','symbol','--percent-limit','0','-F','overhead,period,symbol']),
                                    ('lines',['--no-children','--inline','--sort','symbol,srcline','--percent-limit','0','-F','overhead,period,symbol,srcline','--field-separator','\t']),
                                    ('annotate',['--percent-type','global-period','--source','--print-line','--show-total-period'])]:
                    cmd = [PERF,'report' if action == 'lines' else action,'--stdio','-i',str(target),*args]
                    with prefix.with_suffix('.'+action).open('x') as stream:
                        r = subprocess.run(cmd, stdout=stream,stderr=subprocess.PIPE,text=True,timeout=180)
                    prefix.with_suffix('.'+action+'.stderr').write_text(r.stderr)
                    row[action+'_exit'] = r.returncode
                    assert r.returncode == 0, (action,r.stderr)
        with records.open('a') as stream:
            stream.write(json.dumps(row)+'\n')
        print(cell['case'],arm,'complete',flush=True)
(OUT/'exit.json').write_text(json.dumps(dict(exit=0,records=36))+'\n')
