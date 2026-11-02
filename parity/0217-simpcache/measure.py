"""measure.py <out.jsonl> <ops,comma> <drivers,comma> [cases,comma] [reps]

Exact instructions:u per resident call, counted by perf stat with fd control
around the counted loop of the parity/peers consumer. Drivers are executables
in $MEASURE_DRIVERS (for example cpp-scalar, base-scalar, fin-scalar), built
from parity/peers against the arm's source. The first driver is the ratio
reference. Every cell asserts that all drivers wrote identical output bytes.

Environment: MEASURE_DRIVERS, MEASURE_INPUTS (the eight peer-format
fixtures), MEASURE_OUT (scratch), MEASURE_PERF (perf executable), CPU.
"""
import hashlib
import json
import math
import os
import statistics
import subprocess
import sys
from pathlib import Path

DRIVERS = Path(os.environ['MEASURE_DRIVERS'])
INPUTS = Path(os.environ['MEASURE_INPUTS'])
OUT = Path(os.environ['MEASURE_OUT'])
PERF = os.environ['MEASURE_PERF']
CPU = os.environ.get('CPU', '9')
CASES = ['tiny-smooth', 'medium-smooth', 'medium-seams', 'medium-sparse',
         'million-smooth', 'real-214e', 'real-a082', 'real-b6fd']


def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()


def invoke(drv, op, case, n):
    cr, cw = os.pipe()
    ar, aw = os.pipe()
    OUT.mkdir(parents=True, exist_ok=True)
    out = OUT / f'{drv}-{op}-{case}.out'
    pf = OUT / f'{drv}-{op}-{case}.perf'
    cmd = [PERF, 'stat', '-e', 'instructions:u', '-o', str(pf), '-x', ';', '-D', '-1',
           '--control', f'fd:{cr},{aw}', '--', 'taskset', '-c', CPU, str(DRIVERS / drv), op,
           str(INPUTS / (case + '.bin')), str(out)]
    try:
        r = subprocess.run(cmd, input=f'{n}\nstop\n',
                           env=dict(os.environ, PROBE_CTL=str(cw), PROBE_ACK=str(ar), LC_ALL='C'),
                           pass_fds=(cr, cw, ar, aw), capture_output=True, text=True)
    finally:
        for fd in [cr, cw, ar, aw]:
            os.close(fd)
    assert r.returncode == 0, (cmd, r.stderr[-2000:])
    f = next(line.split(';') for line in pf.read_text().splitlines() if ';instructions:u;' in line)
    assert float(f[4]) == 100.0, f
    return int(f[0]) / n, sha(out)


def main():
    dst = Path(sys.argv[1])
    ops = sys.argv[2].split(',')
    drvs = sys.argv[3].split(',')
    cases = sys.argv[4].split(',') if len(sys.argv) > 4 and sys.argv[4] else CASES
    reps = int(sys.argv[5]) if len(sys.argv) > 5 else 1
    rows = []
    for op in ops:
        for case in cases:
            n = 1000 if case == 'tiny-smooth' else 3 if case.startswith('medium') else 1
            for rep in range(reps):
                for d in (drvs if rep % 2 == 0 else drvs[::-1]):
                    ipc, h = invoke(d, op, case, n)
                    rows.append(dict(op=op, case=case, drv=d, rep=rep, ins=ipc, sha=h))
                    with dst.open('a') as fh:
                        fh.write(json.dumps(rows[-1]) + '\n')
            cell = [r for r in rows if r['op'] == op and r['case'] == case]
            assert len({r['sha'] for r in cell}) == 1, (op, case, 'OUTPUT MISMATCH')
            print(op, case, ' '.join(
                f"{d}={statistics.median(r['ins'] for r in cell if r['drv'] == d):.0f}" for d in drvs), flush=True)

    def gm(v):
        return math.exp(statistics.mean(math.log(x) for x in v))
    ref = drvs[0]
    for op in ops:
        med = {(c, d): statistics.median(r['ins'] for r in rows if r['op'] == op and r['case'] == c and r['drv'] == d)
               for c in cases for d in drvs}
        for d in drvs[1:]:
            real = [med[(c, d)] / med[(c, ref)] for c in cases if c.startswith('real')]
            corp = [med[(c, d)] / med[(c, ref)] for c in cases if not c.startswith('real')]
            print(f'GM {op} {d}/{ref}:' + (f' real {gm(real):.4f}' if real else '') +
                  (f' corpus {gm(corp):.4f}' if corp else '') + ' cases ' +
                  ' '.join(f'{c}:{med[(c, d)] / med[(c, ref)]:.3f}' for c in cases))


main()
