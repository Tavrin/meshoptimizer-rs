"""Lane supervisor. Invoke under CPU-only admission; paths come from the environment.

Usage: python3 parity/0215-update-spatial/run.py prepare|build|checks|exact|measure
Required: MESHOPT_SCRATCH, CARGO_TARGET_DIR; measurements also use PEERS_PERF.
"""
import hashlib
import json
import os
import resource
from pathlib import Path
import shutil
import statistics
import math
import subprocess
import sys

ROOT = Path(__file__).resolve().parents[2]
LANE = Path(__file__).resolve().parent
S = Path(os.environ['MESHOPT_SCRATCH'])
ART = S / os.environ.get('LANE_CAMPAIGN', 'campaign1')
TARGET = Path(os.environ['CARGO_TARGET_DIR'])
assert os.environ.get('PARITY_RUNNER_ACTIVE')
assert os.environ.get('PARITY_RUNNER_GPU') == '0'
os.environ.update(CARGO_INCREMENTAL='0',
                  CARGO_BUILD_JOBS='1', LC_ALL='C', PEERS_ART=str(ART))
S.mkdir(parents=True, exist_ok=True)
(S / 'receipts').mkdir(exist_ok=True)


def sha(p):
    with p.open('rb') as f:
        return hashlib.file_digest(f, 'sha256').hexdigest()


def run(name, cmd, **kwargs):
    path = S / 'receipts' / (name + '.log')
    original = name
    attempt = 1
    while path.exists():
        attempt += 1
        name = original + '-attempt' + str(attempt)
        path = S / 'receipts' / (name + '.log')
    with path.open('x') as f:
        result = subprocess.run(cmd, cwd=ROOT, stdout=f, stderr=subprocess.STDOUT, **kwargs)
    (S / 'receipts' / (name + '.json')).write_text(json.dumps(
        dict(command=cmd, exit=result.returncode, log_sha256=sha(path))) + '\n')
    print(name, result.returncode, flush=True)
    if result.returncode:
        print(path.read_text()[-3000:], flush=True)
        raise SystemExit(result.returncode)


def prepare():
    ART.mkdir(exist_ok=True)
    if ART.name != 'campaign1' and not (S/'rejected-campaign1').exists():
        archive = S/'rejected-campaign1'
        archive.mkdir()
        for name in ['frozen-base', 'frozen-candidate', 'proof']:
            shutil.copytree(S/name, archive/name)
        shutil.copy2(S/'prepared-binding.json', archive/'prepared-binding.json')
    for name in ['base', 'candidate']:
        tree = S / ('frozen-' + name)
        tree.mkdir(exist_ok=True)
        if name == 'base':
            archive = subprocess.check_output(['git', 'archive', 'v0.2.14'], cwd=ROOT)
            subprocess.run(['tar', 'xf', '-', '-C', str(tree)], input=archive, check=True)
        else:
            for part in ['src', 'docs', 'parity/peers']:
                shutil.copytree(ROOT / part, tree / part, dirs_exist_ok=True)
            for part in ['Cargo.toml', 'Cargo.lock', 'README.md', 'LICENSE']:
                shutil.copy2(ROOT / part, tree / part)
        driver = tree / 'parity/peers/src/main.rs'
        for helper in ['prepare_driver.py', 'additional_driver.py']:
            subprocess.run([sys.executable, str(ROOT / 'parity/0216-qualify' / helper), str(driver)], check=True)
    shutil.copytree(S / 'art/inputs', ART / 'inputs', dirs_exist_ok=True)
    shutil.copy2(S / 'art/inputs.json', ART / 'inputs.json')
    proof_base = S / 'proof/base'
    shutil.copytree(S / 'frozen-base', proof_base, dirs_exist_ok=True)
    manifest = proof_base / 'Cargo.toml'
    manifest.write_text(manifest.read_text().replace('name = "meshoptimizer-rs"', 'name = "meshoptimizer-baseline"', 1))
    proof = S / 'proof'
    for kind, source in [('meshlet', 'targeted-meshlet-warm-draft.rs'),
                         ('quadrics', 'targeted-quadrics-warm-draft.rs')]:
        text = (ROOT / 'parity/0216-qualify' / source).read_text()
        if kind == 'meshlet':
            text = text.replace('for variant in 0..5', 'for variant in 4..8')
            text = text.replace('settings, 1, 0.5,',
                                'settings, if $variant == 5 { $setting.1 } else { 1 }, if $variant == 6 { 0. } else if $variant == 7 { 2. } else { 0.5 },')
            text = text.replace('let mut checks=[0u64;5]', 'let mut checks=[0u64;8]')
            text = text.replace('    let settings = [', '    cases.push((vec![[-0.,0.,-0.];72],indices.clone()));\n    let settings = [')
        else:
            text = text.replace('for family in 0..3', 'for family in 2..3')
            text = text.replace('let masks = [0u32, 1, 2, 4, 16, 32, 3, 5, 17, 33, 6, 18, 34, 20, 36, 48];',
                                'let masks: Vec<u32> = (0..512).collect();')
            text = text.replace('end <= 384', 'end <= 12288')
            text = text.replace('cell / 48', 'cell / 1536').replace('cell / 16 % 3', 'cell / 512 % 3').replace('cell % 16', 'cell % 512')
        (proof / (kind + '.rs')).write_text(text)
        out = proof / kind
        out.mkdir(exist_ok=True)
        features = ', '.join('"' + arm + '/' + feat + '"' for arm in ['baseline', 'candidate']
                             for feat in ['std', 'simd', 'parallel', 'experimental', 'parity-internals', 'clusterlod'])
        (out / 'Cargo.toml').write_text(f'''[package]
name = "lane-{kind}"
version = "0.0.0"
edition = "2021"
[features]
all = [{features}]
[dependencies]
baseline = {{ package = "meshoptimizer-baseline", path = "../base", default-features = false }}
candidate = {{ package = "meshoptimizer-rs", path = "../../frozen-candidate", default-features = false }}
[[bin]]
name = "lane-{kind}"
path = "../{kind}.rs"
[profile.release]
lto = "thin"
codegen-units = 1
''')
    binding = {str(p.relative_to(S)): sha(p) for tree in [S/'frozen-base', S/'frozen-candidate', proof]
               for p in tree.rglob('*') if p.is_file()}
    (S / 'prepared-binding.json').write_text(json.dumps(binding, indent=2) + '\n')


def build():
    binaries = {}
    for arm, feature, name in [('base', 'ours', 'base'), ('candidate', 'ours', 'candidate'),
                                ('cpp', 'cpp', 'candidate'), ('rust', 'rust', 'candidate')]:
        run('build-' + arm, ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path',
            str(S / ('frozen-' + name) / 'parity/peers/Cargo.toml'), '--no-default-features', '--features', feature])
        shutil.copy2(TARGET/'release/meshopt-peers', ART / ('driver-' + arm))
        binaries[arm] = sha(ART / ('driver-' + arm))
    sources = {str(p.relative_to(S)): sha(p) for name in ['base', 'candidate']
               for p in (S/('frozen-'+name)).rglob('*') if p.is_file()}
    (ART/'build-binding.json').write_text(json.dumps(dict(binaries=binaries, sources=sources), indent=2)+'\n')


def exact():
    for kind in ['meshlet', 'quadrics']:
        manifest = str(S / 'proof' / kind / 'Cargo.toml')
        run('lock-'+kind, ['cargo', 'generate-lockfile', '--offline', '--manifest-path', manifest])
        for mode in ['all', 'none']:
            args = ['--features', 'all'] if mode == 'all' else []
            run('exact-build-'+kind+'-'+mode, ['cargo', 'build', '--offline', '--locked', '--release', '--manifest-path', manifest, *args])
            binary = S / 'proof' / f'{kind}-{mode}'
            shutil.copy2(TARGET / 'release' / ('lane-'+kind), binary)
            run('exact-'+kind+'-'+mode, [str(binary), *(['0', '12288'] if kind == 'quadrics' else [])])


def checks():
    for name, cmd in [
        ('test-all', ['cargo', 'test', '--locked', '--all-features']),
        ('test-none', ['cargo', 'test', '--locked', '--no-default-features']),
        ('clippy', ['cargo', 'clippy', '--locked', '--all-features', '--all-targets', '--', '-D', 'warnings']),
        ('fmt', ['cargo', 'fmt', '--check']),
        ('boundary', ['bash', 'parity/check-unsafe-boundary.sh']),
        ('public', ['bash', 'parity/tools/check_public_text.sh'])]:
        run(name, cmd)


def measure():
    source = (ROOT / 'parity/0216-qualify/measure-initial.py').read_text()
    start = source.index('ops = ')
    end = source.index('\narms = ', start)
    source = source[:start] + "ops = ['simplify-update', 'meshlets-spatial']" + source[end:]
    runner = ART / 'measure.py'
    if runner.exists():
        assert runner.read_text() == source
    else:
        runner.write_text(source)
    subprocess.run([sys.executable, str(runner)], check=True)


def fuzz():
    for kind, tail in [('meshlet', 'fuzz-spatial.rs'), ('quadrics', 'fuzz-update.rs')]:
        source = (S/'proof'/f'{kind}.rs').read_text().split('fn main() {')[0]
        (S/'proof'/f'fuzz-{kind}.rs').write_text(source + (LANE/tail).read_text())
        original = S/'proof'/kind/'Cargo.toml'
        directory = S/'proof'/('fuzz-'+kind)
        directory.mkdir(exist_ok=True)
        manifest = directory/'Cargo.toml'
        manifest.write_text(original.read_text().replace(f'lane-{kind}', f'fuzz-{kind}').replace(f'../{kind}.rs', f'../fuzz-{kind}.rs'))
        run('fuzz-lock-'+kind, ['cargo', 'generate-lockfile', '--offline', '--manifest-path', str(manifest)])
        run('fuzz-build-'+kind, ['cargo', 'build', '--offline', '--locked', '--release', '--features', 'all', '--manifest-path', str(manifest)])
        binary = S/'proof'/f'fuzz-{kind}-binary'
        shutil.copy2(TARGET/'release'/f'fuzz-{kind}', binary)
        journal = S/'proof'/f'fuzz-{kind}.jsonl'
        rows = [json.loads(x) for x in journal.read_text().splitlines()] if journal.exists() else []
        assert all(r['exit'] == 0 and r['binary'] == sha(binary) for r in rows)
        total = sum(r['cpu_seconds'] for r in rows)
        while total < 300:
            seed = 21540000 + len(rows)
            before = resource.getrusage(resource.RUSAGE_CHILDREN)
            child = subprocess.run([str(binary), str(seed)], capture_output=True)
            after = resource.getrusage(resource.RUSAGE_CHILDREN)
            seconds = after.ru_utime + after.ru_stime - before.ru_utime - before.ru_stime
            row = dict(seed=seed, cases=100, exit=child.returncode, cpu_seconds=seconds, binary=sha(binary))
            with journal.open('a') as f:
                f.write(json.dumps(row)+'\n')
                f.flush()
                os.fsync(f.fileno())
            if child.returncode:
                (S/'proof'/f'fuzz-{kind}-failure.log').write_bytes(child.stderr)
                raise SystemExit(child.returncode)
            rows.append(row)
            total += seconds
        print('fuzz', kind, len(rows)*100, total, 'PASS', flush=True)


def cpp():
    os.environ['MESHOPT_QUALIFY_TARGET'] = str(TARGET)
    path = S/'proof/cpp-supervisor.sh'
    path.write_text((ROOT/'parity/0216-qualify/run-protocol-modes.sh').read_text().replace('$scratch/candidate', '$scratch/frozen-candidate'))
    run('cpp-update', ['bash', str(path)])
    os.environ['PEERS_ART'] = str(S/'proof/cpp-meshlet')
    Path(os.environ['PEERS_ART']).mkdir(exist_ok=True)
    run('cpp-spatial', [sys.executable, str(ROOT/'parity/0216-qualify/meshlet-protocol-modes.py')])


def summarize():
    rows = [json.loads(x) for x in (ART/'counts.jsonl').read_text().splitlines()]
    assert len(rows) == 192
    cells = []
    for case in json.loads((ART/'inputs.json').read_text()):
        for op in ['simplify-update', 'meshlets-spatial']:
            values = [r for r in rows if r['case'] == case['case'] and r['op'] == op]
            assert len(values) == 12
            assert len({r['output_sha256'] for r in values}) == 1, (case['case'], op)
            med = {arm: statistics.median(r['instructions_per_call'] for r in values if r['arm'] == arm)
                   for arm in ['base', 'candidate', 'cpp', 'rust']}
            assert med['candidate']/med['base'] <= 1.01
            cells.append(dict(case=case['case'], op=op, instructions=med,
                              candidate_base=med['candidate']/med['base'],
                              candidate_optimesh=med['candidate']/med['rust'],
                              candidate_cpp=med['candidate']/med['cpp']))
    verdicts = {}
    for op in ['simplify-update', 'meshlets-spatial']:
        real = [r for r in cells if r['op'] == op and r['case'].startswith('real-')]
        ratios = {key: math.exp(sum(math.log(r[key]) for r in real)/3)
                  for key in ['candidate_base', 'candidate_optimesh', 'candidate_cpp']}
        ratios['verdict'] = 'SHIPS' if ratios['candidate_base'] <= .97 and ratios['candidate_optimesh'] <= .99 else 'IMPROVES' if ratios['candidate_base'] <= .97 else 'NO-GAIN'
        verdicts[op] = ratios
    (LANE/'measurements.json').write_text(json.dumps(dict(samples=len(rows), cells=cells, real_geomeans=verdicts), indent=2)+'\n')
    shutil.copy2(ART/'counts.jsonl', LANE/'counts.jsonl')
    shutil.copy2(ART/'inputs.json', LANE/'inputs.json')
    shutil.copy2(ART/'build-binding.json', LANE/'build-binding.json')
    if ART.name != 'campaign1':
        shutil.copy2(S/'campaign1/counts.jsonl', LANE/'rejected-counts.jsonl')
        shutil.copy2(S/'campaign1/regression.json', LANE/'regression.json')
        shutil.copy2(S/'campaign1/build-binding.json', LANE/'rejected-build-binding.json')
    receipts = {}
    for p in sorted((S/'receipts').glob('*.json')):
        data = json.loads(p.read_text())
        command = [x.replace(str(ROOT), '<repo>').replace(str(S), '<scratch>').replace(str(TARGET), '<target>') for x in data['command']]
        receipts[p.stem] = dict(exit=data['exit'], command=command, log_sha256=data['log_sha256'])
    for kind in ['meshlet', 'quadrics']:
        path = S/'proof'/f'fuzz-{kind}.jsonl'
        if path.exists():
            fuzzrows = [json.loads(x) for x in path.read_text().splitlines()]
            receipts['fuzz-'+kind] = dict(exit=0, cases=sum(r['cases'] for r in fuzzrows),
                cpu_seconds=sum(r['cpu_seconds'] for r in fuzzrows), journal_sha256=sha(path),
                binary=fuzzrows[0]['binary'])
    (LANE/'receipts.json').write_text(json.dumps(receipts, indent=2)+'\n')
    proof_binding = {str(p.relative_to(S)): sha(p) for p in (S/'proof').rglob('*')
                     if p.is_file() and (p.suffix == '.rs' or os.access(p, os.X_OK))}
    (LANE/'proof-binding.json').write_text(json.dumps(proof_binding, indent=2)+'\n')
    print(json.dumps(verdicts, indent=2))


dict(prepare=prepare, build=build, checks=checks, exact=exact, measure=measure,
     fuzz=fuzz, cpp=cpp, summarize=summarize)[sys.argv[1]]()
