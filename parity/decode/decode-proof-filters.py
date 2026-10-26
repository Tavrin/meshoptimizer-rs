#!/usr/bin/env python3
"""Retain native exhaustive proof; close allocating filter reachability gap.

The first driver used v1 vertex encoding for EXT views, which correctly reject
v1. Direct filter calls reached the changed code. This completion script uses
v0 EXT input, asserts bulk entry separately for each API in counter builds,
and retains all earlier logs. It performs no timings or library edits.
"""
import importlib.util
import json
import os
from pathlib import Path
import shutil

spec = importlib.util.spec_from_file_location('proof', Path(__file__).with_name('decode-proof.py'))
p = importlib.util.module_from_spec(spec)
spec.loader.exec_module(p)
assert os.environ.get('MOSS_HEAVY_ACTIVE')
assert os.environ['CARGO_TARGET_DIR'] == str(p.TARGET)
assert (p.ART/'native-exact.json').exists(), 'original exhaustive vertex check must finish first'
assert not (p.ART/'filter-final-summary.json').exists(), 'never overwrite completion receipts'
shutil.copy2(__file__,p.ART/'decode-proof-filters.py')
shutil.copy2(Path(__file__).with_name('decode-proof.py'),p.ART/'decode-proof-final-driver.py')
packages=[p.package(p.ART/name,'filter-final-'+name) for name in ['baseline','candidate','probe']]
rows=[]
native=[p.build(pkg,'filter-final-'+name+'-native',release=True,probe=name=='probe') for pkg,name in zip(packages,['baseline','candidate','probe'])]
rows.append(p.compare(*native[:2],'filter-final-native',suite='filter'))
rows.append(p.compare(native[0],native[2],'filter-final-reach-native',probe=True,suite='filter'))
for release in [False,True]:
    mode='release' if release else 'debug'
    binaries=[p.build(pkg,'filter-final-'+name+'-wasm-'+mode,wasm=True,release=release,probe=name=='probe') for pkg,name in zip(packages,['baseline','candidate','probe'])]
    rows.append(p.compare(*binaries[:2],'filter-final-wasm-'+mode,wasm=True))
    rows.append(p.compare(binaries[0],binaries[2],'filter-final-reach-wasm-'+mode,wasm=True,probe=True))
assert all(row['cases']==3740 and row['api_records']==14240 for row in rows)
(p.ART/'filter-final-summary.json').write_text(json.dumps(dict(verdict='PASS',comparisons=rows),indent=2)+'\n')
