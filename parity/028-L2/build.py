#!/usr/bin/env python3
"""Build retained scalar comparators; run only within 4 GiB CPU admission."""
import hashlib, json, os, pathlib, shutil, subprocess, sys
ROOT = pathlib.Path(__file__).resolve().parents[2]
ART = pathlib.Path('/mnt/linux-extra/moss-scratch/meshopt-028-L2')
TARGET = pathlib.Path('/mnt/linux-extra/moss-cargo-targets/meshopt-L2')
BASE = 'e5168983067a5decfc5bf15605c7dae7113dba49'
assert os.environ.get('MOSS_HEAVY_ACTIVE')
assert pathlib.Path(os.environ['CARGO_TARGET_DIR']) == TARGET
if not (ART / 'baseline').exists():
    (ART / 'baseline').mkdir()
    archive = subprocess.check_output(['git', 'archive', BASE])
    subprocess.run(['tar', '-x', '-C', str(ART / 'baseline')], input=archive, check=True)
if not (ART / 'inputs').exists():
    shutil.copytree('/mnt/linux-extra/moss-scratch/meshopt-peer-screen/campaign-20261025T150615Z/inputs', ART / 'inputs')
    (ART / 'inputs-sha256.json').write_text(json.dumps({p.name:hashlib.sha256(p.read_bytes()).hexdigest() for p in (ART / 'inputs').glob('*.bin')}, indent=2)+'\n')
for arm in sys.argv[1:]:
    harness = ART / ('harness-' + arm)
    if not harness.exists():
        shutil.copytree(ART / 'parity/compare', harness)
    source = (ART / 'parity/compare/src/main.rs').read_text()
    if 'strip_prefix("count ")' not in source:
        source = source.replace('        let iterations: usize = line.parse().unwrap();', '''        if let Some(n) = line.strip_prefix("count ") {
            for _ in 0..n.parse::<usize>().unwrap() {
                black_box(run(black_box(op), black_box(&x), &mut w));
            }
            println!("COUNTED");
            io::stdout().flush().unwrap();
            continue;
        }
        let iterations: usize = line.parse().unwrap();''')
    if 'fn chain(a:' not in source:
        source += '\n' + (ROOT / 'parity/028-L2/stage-support.rs').read_text()
        source = source.replace('    let op = &a[1];', '    if a[1].starts_with("chain") { chain(&a); return; }\n    let op = &a[1];')
        # Exported real scale adapters incorrectly hardcode packed stride.
        source = source.replace('f::meshopt_simplifyScale(p, n, 12)', 'f::meshopt_simplifyScale(p, n, stride)')
        xp = harness / 'src/peer_x.rs'
        xp.write_text(xp.read_text().replace('s::simplify_scale(positions,n,12)', 's::simplify_scale(positions,n,stride)'))
    # Real L2 calls never consume synthetic attributes or codec transport.
    # Omit that untimed preparation equally in every arm so paired resident
    # real drivers fit the lane's 4 GiB envelope.
    source = source.replace('        let floats: Vec<f32> = (0..n)',
                            '        let floats: Vec<f32> = (0..if replay.is_some() { 0 } else { n })')
    source = source.replace('        let raw = p\n            .iter()',
                            '        let raw = p[..if replay.is_some() { 0 } else { n }]\n            .iter()')
    # The four L2 adapters already borrow positions and indices. No input
    # preparation is required in any of O/M7/X. Keep their allocation APIs.
    (harness / 'src/main.rs').write_text(source)
    manifest = (ART / 'parity/compare/Cargo.toml').read_text()
    lib = ART / 'baseline' if arm == 'base' else ROOT
    manifest = manifest.replace('path = "../.."', 'path = ' + json.dumps(str(lib)))
    (harness / 'Cargo.toml').write_text(manifest)
    feature = {'base':'ours', 'candidate':'ours', 'M7':'peer7', 'X':'optimesh'}[arm]
    if feature == 'ours': feature += ',meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod'
    env = dict(os.environ, RUSTC_WRAPPER='', RUSTC_WORKSPACE_WRAPPER='', RUSTFLAGS='', CARGO_ENCODED_RUSTFLAGS='', CARGO_INCREMENTAL='0', CARGO_BUILD_JOBS='1', CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD')
    # Exported lock has the older workspace version; resolve offline once.
    subprocess.run(['cargo','generate-lockfile','--offline','--manifest-path',str(harness/'Cargo.toml')], env=env, check=True)
    index = len(list(ART.glob(f'build-{arm}-*.json')))
    stem = ART / f'build-{arm}-{index}'
    command = ['cargo','build','--offline','--locked','--manifest-path',str(harness/'Cargo.toml'),'--profile','moss','--features',feature,'--bin','cmp-driver']
    with stem.with_suffix('.log').open('w') as log:
        r = subprocess.run(command, env=env, stdout=log, stderr=subprocess.STDOUT)
    receipt = dict(command=command, exit=r.returncode, admission=os.environ['MOSS_HEAVY_ACTIVE'], source={str(p.relative_to(lib)):hashlib.sha256(p.read_bytes()).hexdigest() for p in (lib/'src').rglob('*.rs')}, harness={str(p.relative_to(harness)):hashlib.sha256(p.read_bytes()).hexdigest() for p in [harness/'Cargo.toml', harness/'Cargo.lock', *sorted((harness/'src').glob('*.rs'))]})
    if r.returncode == 0:
        binary = ART / f'driver-{arm}-{index}'
        shutil.copy2(TARGET/'moss/cmp-driver', binary)
        receipt.update(binary=str(binary), sha256=hashlib.sha256(binary.read_bytes()).hexdigest())
    stem.with_suffix('.json').write_text(json.dumps(receipt, indent=2)+'\n')
    if r.returncode:
        print(stem.with_suffix('.log').read_text()); sys.exit(r.returncode)
    print(binary, flush=True)
