#!/usr/bin/env python3
"""One checkpointed P07 performance pass, with the registered paired protocol."""
import inspect
import json
import sys
import time
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent / "simd"))
import qualify as q

path = q.ART / "performance.json"
previous = json.loads(path.read_text()) if path.exists() else None
if previous:
    assert not previous.get("complete", False), "complete matrix must not be repeated"
    assert previous["sources"] == q.sources()
    assert previous["binaries"] == {p.name: q.m.sha(p) for p in q.BIN.iterdir()}
done = {(r["api"], r["case"]) for r in previous["rows"]} if previous else set()
controller = {
    "source": "parity/measure-p07-perf.py",
    "sha256": q.m.sha(__file__),
    "original_source": "parity/simd/qualify.py",
    "original_sha256": q.m.sha(q.ROOT / "parity/simd/qualify.py"),
    "resumed_rows": len(done),
    "poll_seconds": 2,
}

# Reuse the numerical protocol verbatim. Only admission polling, checkpoint
# adoption and retention of an unfinished row differ from the original runner.
source = inspect.getsource(q.bench)
source = source.replace(
    "if path.exists() and not diagnostic:raise ValueError('final matrix already exists; do not repeat')",
    'assert not previous or not previous.get("complete", False)',
)
source = source.replace("time.sleep(15)", "time.sleep(2)")
source = source.replace(
    "    for api in ['allocating','caller-buffer']:",
    "    if previous:record['rows']=previous['rows'];raw=record['rows'];record['admissions']=previous['admissions']\n"
    "    record['controller_segments']=(previous.get('controller_segments',[]) if previous else [])+[controller]\n"
    "    for api in ['allocating','caller-buffer']:",
)
source = source.replace(
    "        for name,b in cases:",
    "        for name,b in cases:\n            if (api,name) in done:continue",
)
source = source.replace(
    "            while len(row['raw_seconds']['rust'])<",
    "            record['current_row']=row\n            while len(row['raw_seconds']['rust'])<",
)
source = source.replace(
    "            raw.append(row);save(path,record)",
    "            record.pop('current_row',None);raw.append(row);save(path,record)",
)
assert source.count("in done:continue") == 1
assert source.count("record['current_row']=row") == 1
executed = q.ART / "native-executed-controller.py"
executed.write_text(source)
controller["executed_controller_sha256"] = q.m.sha(executed)
namespace = dict(q.__dict__, previous=previous, done=done, controller=controller)
exec(compile(source, str(executed), "exec"), namespace)
namespace["bench"](q.corpus())
assert q.m.sha(__file__) == controller["sha256"]
