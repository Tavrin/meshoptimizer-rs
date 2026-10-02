#!/usr/bin/env python3
"""Retain gprofng profiles of the slowest measured million-triangle case in each geometry family."""
import json
import os
import struct
import subprocess
import time
import zipfile
import runner as r


def main():
    reference, target, results = r.paths()
    benchmark = json.loads((results/'benchmark.json').read_text())
    before = r.snapshot(reference)
    if benchmark['identities']['source_sha256'] != before:
        raise ValueError('stale benchmark sources')
    binary = target/'release/meshopt-driver'
    if r.sha(binary) != benchmark['identities']['executable_sha256']['rust']:
        raise ValueError('stale benchmark binary')
    artifact_dir = r.artifacts_path()
    directory = artifact_dir/'profiles'
    directory.mkdir(exist_ok=True)
    if r.sha(artifact_dir/'benchmark-buffers.zip') != benchmark['artifacts']['benchmark-buffers.zip']:
        raise ValueError('changed benchmark archive')
    record = {'source_sha256': before, 'executable_sha256': r.sha(binary), 'runs': [],
              'method': 'gprofng clock sampling; 10 to 100 timed calls targeting 30 seconds plus warmup; sampling attribution is not an impossibility proof'}
    with zipfile.ZipFile(artifact_dir/'benchmark-buffers.zip') as archive:
        for family in r.FAMILIES.values():
            name, case = max(((n,w) for n,w in benchmark['workloads'].items() if w['family']==family and w['size']=='million'),
                             key=lambda nw:nw[1]['rust_cpp_ratio'])
            data = bytearray(archive.read(case['input']['member']))
            samples = max(10, min(100, int(30 / case['stats']['rust']['median_seconds'])))
            struct.pack_into('<I', data, 24, samples)
            inp = directory/(family+'.input'); inp.write_bytes(data)
            experiment = directory/(family+'.er')
            command = ['gprofng','collect','app','-o',str(experiment),str(binary)]
            start = time.time()
            run = subprocess.run(command,input=data,capture_output=True,env=r.ENV,timeout=300)
            output = directory/(family+'.output'); output.write_bytes(run.stdout)
            (directory/(family+'.stderr')).write_bytes(run.stderr)
            if run.returncode: raise ValueError(f'profile failed: {family}: {run.stderr!r}')
            # The collector prefixes stdout with its experiment creation notice.
            raw = run.stdout[run.stdout.index(b'MR01'):]
            values, times = r.response(raw[:-8],samples=samples)
            expected = archive.read(case['outputs'][0]['member'])
            if values != r.response(expected[:-8],samples=struct.unpack_from('<I',expected,12)[0])[0]: raise ValueError('profile output mismatch')
            reports = {}
            for report, option in [('functions','-functions'),('lines','-lines')]:
                result = r.command(['gprofng','display','text','-limit','40',option,experiment],capture_output=True)
                path = directory/(family+'-'+report+'.txt');path.write_bytes(result.stdout)
                reports[path.name] = r.sha(path)
            record['runs'].append({'case':name,'command':command,'exit':run.returncode,
                                   'elapsed_seconds':time.time()-start,'timed_seconds':times,
                                   'load_end':os.getloadavg(),'input_sha256':r.sha(inp),
                                   'output_sha256':r.sha(output),'reports':reports})
    record['identities_unchanged'] = before==r.snapshot(reference) and r.sha(binary)==record['executable_sha256']
    (results/'profiles.json').write_text(json.dumps(record,indent=2)+'\n')
    if not record['identities_unchanged']: raise ValueError('changed profile identity')


if __name__=='__main__':
    main()
