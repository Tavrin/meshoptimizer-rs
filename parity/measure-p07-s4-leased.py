#!/usr/bin/env python3
"""Run only registered scalar-baseline borderline supplements under the lease."""
import json
import sys
import time
from pathlib import Path

sys.path.insert(0,str(Path(__file__).resolve().parent/'simd'))
import qualify as q
from p07_lease import admission

began=time.monotonic()
before=admission()
q.admission=admission
original=q.ROOT/'parity/measure-simd-s4.py'
source=original.read_text()
exec(compile(source,str(original),'exec'),{'__file__':str(original),'__name__':'__main__'})
after=admission()
q.save(q.ART/'s4-lease-context.json',{'before':before,'after':after,'wall_seconds':time.monotonic()-began,
    'wrapper_sha256':q.m.sha(__file__),'admission_sha256':q.m.sha(q.ROOT/'parity/p07_lease.py'),
    'original_sha256':q.m.sha(original)})
assert time.monotonic()-began < 840
