
# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import hashlib,json,subprocess,datetime,tomllib
s=Path(__file__).parent;repo=Path(_public_path('<repo>'))
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
ids={}
for a,rev in [('B','afe803b'),('Bprime','afe803b'),('C','f40f5af'),('Bp','afe803b'),('Cp','f40f5af')]:
 source=s/f'src-{a}';profile=tomllib.loads((source/'parity/Cargo.toml').read_text())['profile']['release'];assert profile['lto']=='fat' and profile['codegen-units']==1;full=subprocess.check_output(['git','-C',str(repo),'rev-parse',rev],text=True).strip();manifest={str(p.relative_to(source)):sha(p) for p in sorted((source/'src').rglob('*.rs'))}
 for n,h in manifest.items():assert hashlib.sha256(subprocess.check_output(['git','-C',str(repo),'show',f'{full}:{n}'])).hexdigest()==h
 driver={str(p.relative_to(source/'parity/src')):sha(p) for p in sorted((source/'parity/src').rglob('*.rs'))};expected={str(p.relative_to(repo/'parity/src')):sha(p) for p in sorted((repo/'parity/src').rglob('*.rs'))};assert driver==expected
 flags='-C llvm-args=-align-all-functions=6' if a in ['Bp','Cp'] else ''
 target=Path(_public_path('<target-dir>'))/f'meshopt-rcv2-{a}'
 fingerprints=list((target/'release/.fingerprint').glob('meshoptimizer-rs-*/lib-meshoptimizer_rs.json'));assert any(json.loads(p.read_text())['rustflags']==flags.split() for p in fingerprints)
 ids[a]={'revision':full,'binary':str(s/f'driver-{a}'),'sha256':sha(s/f'driver-{a}'),'src_manifest':manifest,'driver_manifest':driver,'archive_sha256':sha(s/f'source-{a}.tar'),'profile':{'lto':'fat','codegen_units':1,'rustflags':flags,'encoded_rustflags':'unset'},'build_log':str(s/(f'build-{a}-qualified.log' if flags else f'build-{a}.log'))}
assert ids['B']['archive_sha256']==ids['Bprime']['archive_sha256']
assert ids['B']['src_manifest']==ids['Bprime']['src_manifest']==ids['Bp']['src_manifest']
assert ids['C']['src_manifest']==ids['Cp']['src_manifest']
(s/'identities.json').write_text(json.dumps(ids,indent=2)+'\n')
assert json.loads((s/'alignment-check.json').read_text())['passed']
assert json.loads((s/'alignment-parity-summary.json').read_text())['passed']
assert (s/'driver-Cp').read_bytes()==(s/'parity-artifacts/run/binaries/rust').read_bytes()
assert json.loads((s/'real-Cp/hash-checks.json').read_text())
codes={p.name:sha(p) for p in sorted(s.iterdir()) if p.is_file() and p.suffix in ['.py','.sh']}
(s/'protocol-code-manifest.json').write_text(json.dumps(codes,indent=2)+'\n')
frozen=s/'protocol-frozen';frozen.mkdir()
for name in codes:(frozen/name).write_bytes((s/name).read_bytes())
reg=(s/'REGISTRATION.md').read_text()+'\n\n## Qualification frozen before step 1\n\nUTC: '+datetime.datetime.now(datetime.timezone.utc).isoformat()+_public_path('''\n\nSelected first option: `-C llvm-args=-align-all-functions=6`. Both arms built; phase 0.1 scalar-strict parity and all three real-case exact hash checks passed on Cp. `cmp` verified the parity qualification executable equals the retained Cp driver. The scratch-only runner preserves RUSTFLAGS and removes the empty encoded-flags override. Its inherited descriptive rust_profile sentence still says flags are empty; the retained Cargo fingerprints, explicit build flags and symbol checks establish the actual alignment flag; no algorithm or driver changes. Real-case checker is the retained r2 exact checker with archive dependency/output paths adapted. Initial unqualified perturbation binaries and build incidents retained in build-preflight-incident.json. The real-case checker dependency build hit its 7 GiB cap and <job-runner> queued a retry at 11 GiB, which refused to overwrite the immutable completed parity receipt. The unfinished real-checker build was resumed alone at 11 GiB; the original OOM log is retained; no measurement occurred.\n\nRaw kernel symbol table checks (every required address mod 64 = 0):\n\n```text\n''')+(s/'alignment-qualified.txt').read_text()+'''```\n\nFull raw nm outputs: Bp-symbols.txt and Cp-symbols.txt. Build/profile/source identities: identities.json. Qualification logs: parity-qualification.log, real-qualification.log, real-Cp/hash-checks.json.\n\nController uses unchanged performance.Paired and parity/src. ABBA is 12 alternating paired ratios (AB/BA), identical to v1. Bootstrap uses NumPy PCG64 seed 20261025 and 10000 within-case resamples. A single logical measurement session covers main, fresh A/A and replication. A train PAUSE releases admission at a completed-case boundary; resumption keeps that session and never repeats a completed case. Any waiting train job causes PAUSE regardless of queue reason. Infrastructure crash/timeout permits one complete session restart only. Layout profiling is diagnostic and cannot feed F, T, or the comparison statistics. No prior diagnosis/repro timings feed decisions.\n'''
reg+='\nProtocol script manifest SHA-256: `'+sha(s/'protocol-code-manifest.json')+'`. Exact scripts retained in protocol-frozen/.\n'
(s/'REGISTRATION.md').write_text(reg);(repo/'parity/release-check-v2/REGISTRATION.md').write_text(reg)
(s/'registration-frozen.json').write_text(json.dumps({'utc':datetime.datetime.now(datetime.timezone.utc).isoformat(),'sha256':sha(s/'REGISTRATION.md')},indent=2)+'\n')
