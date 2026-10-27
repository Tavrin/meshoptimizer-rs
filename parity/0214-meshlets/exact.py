#!/usr/bin/env python3
"""Compare the two witnesses at every work limit for small meshlet cases."""
import hashlib
import json
import os
from pathlib import Path
import re
import struct
import subprocess

assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ['PARITY_RUNNER_GPU'] == '0'
root = Path(__file__).resolve().parents[2]
art = Path(os.environ['MESHLETS_ART'])
def message(op, points, indices, mv, min, mt, weight, split, repeats, target=4):
    header = struct.pack('<4s6I2f3I', b'MO03', op, len(points), len(indices),
                         mv, min, mt, weight, split, target, 0, repeats)
    return (header + b''.join(struct.pack('<3f', *p) for p in points)
            + b''.join(struct.pack('<I', i) for i in indices)
            + bytes(4 * len(points)))

class Client:
    def __init__(self, path):
        self.proc = subprocess.Popen([str(path)], stdin=subprocess.PIPE, stdout=subprocess.PIPE)
    def run(self, data):
        self.proc.stdin.write(struct.pack('<I', len(data)) + data)
        self.proc.stdin.flush()
        head = self.proc.stdout.read(4)
        assert len(head) == 4
        size = struct.unpack('<I', head)[0]
        value = self.proc.stdout.read(size)
        assert len(value) == size
        return value
    def close(self):
        self.proc.stdin.close()
        assert self.proc.wait() == 0

clients = {arm: Client(art / ('witness-' + arm)) for arm in ('base', 'candidate')}
records = []
meshes = [([], []), ([(0.,0.,0.),(1.,0.,0.),(0.,1.,0.)], [0,1,2]),
          ([(0.,0.,0.),(1.,0.,0.),(0.,1.,0.),(1.,1.,0.)], [0,1,2,2,1,3]),
          ([(0.,0.,0.),(1.,0.,0.),(0.,1.,0.)], [0,0,1,0,1,2,2,1,0])]
for op in (1, 3, 4):
 for mesh_index, (points, indices) in enumerate(meshes):
  for layout in range(4):
   for into in (False, True):
    for weight, split in ((0., 0.), (0.5, 1.)):
     data = message(op | (0x10000 if into else 0), points, indices,
                        mv=4, min=1, mt=2, weight=weight, split=split, repeats=layout)
     initial = clients['base'].run(data)
     assert initial == clients['candidate'].run(data)
     match = re.search(rb'work: (\d+)', initial)
     assert match, initial
     work = int(match.group(1))
     for limit in range(work + 2):
      request = data + struct.pack('<QQB', limit, 512 * 1024 * 1024, 0)
      values = {arm: client.run(request) for arm, client in clients.items()}
      if values['base'] != values['candidate']:
       (art / 'exact-divergence.input').write_bytes(request)
       for arm, value in values.items(): (art / ('exact-divergence-' + arm)).write_bytes(value)
       raise AssertionError((op, mesh_index, layout, into, weight, split, limit))
     records.append(dict(op=op, mesh=mesh_index, layout=layout, into=into, weight=weight,
                         split=split, successful_work=work, comparisons=work+2,
                         request_sha256=hashlib.sha256(data).hexdigest()))
     with (art/'exact.jsonl').open('a') as f: f.write(json.dumps(records[-1])+'\n')
# Caller destinations below capacity and malformed topology/settings exercise
# error precedence and contents at every limit up to the ordinary success.
for op in (1, 3, 4):
 for mesh_index, (points, indices) in enumerate(meshes[1:], 1):
  data = message(op | 0x10000, points, indices, mv=4, min=1, mt=2,
                 weight=0., split=0., repeats=0)
  success = clients['base'].run(data)
  work = int(re.search(rb'work: (\d+)', success).group(1))
  for short in (1, 2, 3):
   for limit in range(work + 2):
    request = data + struct.pack('<QQB', limit, 512 * 1024 * 1024, short)
    a = clients['base'].run(request); b = clients['candidate'].run(request)
    assert a == b, (op, mesh_index, short, limit)
   row = dict(op=op, mesh=mesh_index, short=short, comparisons=work+2)
   records.append(row)
   with (art/'exact.jsonl').open('a') as f: f.write(json.dumps(row)+'\n')
for op in (20, 21, 22):
 for mesh_index, (points, indices) in enumerate(meshes):
  for flags in (0, 1, 2, 4, 8, 16, 32, 63, 64, 128, 256, 512, 1024, 2048, 4096, 8192, 16384, 32768):
   data = message(op, points, indices, mv=4, min=1, mt=2, weight=0., split=0., repeats=0, target=flags)
   initial = clients['base'].run(data)
   assert initial == clients['candidate'].run(data)
   match = re.search(rb'work: (\d+)', initial)
   assert match, initial
   work = int(match.group(1))
   for limit in range(work + 2):
    request = data + struct.pack('<QQB', limit, 512 * 1024 * 1024, 0)
    values = {arm: client.run(request) for arm, client in clients.items()}
    if values['base'] != values['candidate']:
     (art / 'exact-divergence.input').write_bytes(request)
     for arm, value in values.items(): (art / ('exact-divergence-' + arm)).write_bytes(value)
     raise AssertionError((op, mesh_index, flags, limit))
   row = dict(op=op, mesh=mesh_index, flags=flags, successful_work=work, comparisons=work+2,
              request_sha256=hashlib.sha256(data).hexdigest())
   records.append(row)
   with (art/'exact.jsonl').open('a') as f: f.write(json.dumps(row)+'\n')
# Coincident triangles create a maximally unsplittable KD leaf, exercising the
# existing covered-leaf work bound and its short-budget fallback.
points = [(0.,0.,0.), (0.,0.,0.), (0.,0.,0.)]
for op in (1, 3, 4):
 for into in (False, True):
  data = message(op | (0x10000 if into else 0), points, [0,1,2] * 32,
                 mv=4, min=1, mt=2, weight=0., split=0., repeats=0)
  initial = clients['base'].run(data)
  assert initial == clients['candidate'].run(data)
  work = int(re.search(rb'work: (\d+)', initial).group(1))
  for limit in range(work + 2):
   request = data + struct.pack('<QQB', limit, 512 * 1024 * 1024, 0)
   assert clients['base'].run(request) == clients['candidate'].run(request), (op, into, limit)
  row = dict(op=op, mesh='unsplittable', into=into, comparisons=work+2)
  records.append(row)
  with (art/'exact.jsonl').open('a') as f: f.write(json.dumps(row)+'\n')
for client in clients.values(): client.close()
print('completed', len(records), 'cases', sum(x['comparisons'] for x in records), 'limits')
