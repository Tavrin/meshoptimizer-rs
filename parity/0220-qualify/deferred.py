"""Run only the deferred 0.2.20 qualification items on one commit.

Usage: python3 parity/0220-qualify/deferred.py <commit> [item ...]

Items, in order (default: all of them):
  v2-exact      weight-zero v2 bytes against the baseline release, grids 12/24/48, both feature modes
  determinism   cross-variant determinism, including v2 at normal-error weights 0 and 0.25
  bench         virtual-geometry benchmark rows, then the two-weight shading rows
  all-features  `cargo test --release --all-features` (tests and doctests)

The commit is extracted with `git archive` (no checkout, no change to the worktree) and every item runs
against that copy. Each item is submitted separately through the caller's admission wrapper; a hold or
refusal (exit 75, 130 or 143) stops further submissions. Everything a run writes goes under
$QUALIFY_LOGS/deferred-<commit>/; nothing is written into the repository.
"""
import io, json, os, subprocess, sys, tarfile
from pathlib import Path

# item -> admission reservation in GB
ITEMS = {'v2-exact': 4, 'determinism': 4, 'bench': 4, 'all-features': 9}
STOP = (75, 130, 143)
HERE = Path(__file__).resolve().parent


def item(name):
    """Inner step: runs inside one admission with QUALIFY_SOURCE/RECEIPTS/LOGS pointing at the run directory."""
    import run as q
    q.T.mkdir(parents=True, exist_ok=True)
    (q.T / 'bin').mkdir(exist_ok=True)
    q.OUT.mkdir(parents=True, exist_ok=True)
    if name == 'v2-exact':
        import v2_exact
        return 0 if v2_exact.main() else 1
    if name == 'determinism':
        try:
            q.new_apis_determinism()
            return 0
        except (AssertionError, SystemExit) as e:
            print('determinism failed:', repr(e), flush=True)
            return 1
    if name == 'bench':
        return subprocess.run(['python3', str(HERE / 'bench.py')]).returncode
    if name == 'all-features':
        return subprocess.run(['python3', str(HERE / 'gate.py'), 'release-all-features']).returncode
    raise SystemExit(f'unknown item {name}')


def extract(sha, dest):
    """Extract `sha` into `dest` once; the harness repository is only read."""
    if (dest / 'Cargo.toml').exists():
        return
    archive = subprocess.check_output(['git', 'archive', sha], cwd=HERE.parents[1])
    dest.mkdir(parents=True)
    tarfile.open(fileobj=io.BytesIO(archive)).extractall(dest, filter='data')


def held():
    steer = os.environ.get('QUALIFY_STEER')
    if not steer or not Path(steer).exists():
        return False
    state = json.loads(Path(steer).read_text())
    return bool(state.get('holds') or state.get('lane_holds'))


def results(receipts):
    """Small per-item facts read back from the receipts (absent when an item did not get that far)."""
    out = {}
    for mode in ['all', 'none']:
        p = receipts / f'v2-exact-{mode}.json'
        if p.exists():
            d = json.loads(p.read_text())
            out[f'v2-exact-{mode}'] = dict(equal=d['equal'], total=d['total'], complete=d['complete'], baseline=d['baseline'])
    p = receipts / 'determinism.json'
    if p.exists():
        d = json.loads(p.read_text())
        cases = d['variants'].get('native-all', {}).get('cases', [])
        weights = {}
        for case in ['v2-normal-weight-zero', 'v2-normal-weight-quarter']:
            if case in cases:
                i = cases.index(case)
                hashes = {v: r['hashes'][i] if i < len(r['hashes']) else None for v, r in d['variants'].items()}
                weights[case] = dict(variants=len(hashes), all_equal=len(set(hashes.values())) == 1 and None not in hashes.values())
        out['determinism'] = dict(all_equal=d['all_equal'], variants=len(d['variants']), weights=weights)
    p = receipts / 'bench' / 'failures.json'
    if p.exists():
        out['bench'] = dict(failed_meshes=json.loads(p.read_text()))
    p = receipts / 'shading' / 'core.json'
    if p.exists():
        out['shading'] = dict(rows=len(json.loads(p.read_text())))
    return out


def main(argv):
    if len(argv) < 1 or argv[0].startswith('-'):
        raise SystemExit(__doc__)
    names = argv[1:] or list(ITEMS)
    unknown = [n for n in names if n not in ITEMS]
    if unknown:
        raise SystemExit(f'unknown item(s) {unknown}; choose from {list(ITEMS)}')
    for var in ['QUALIFY_LOGS', 'CARGO_TARGET_DIR', 'QUALIFY_ADMISSION']:
        if not os.environ.get(var):
            raise SystemExit(f'set {var} (see parity/0220-qualify/README.md)')
    sha = subprocess.check_output(['git', 'rev-parse', '--verify', argv[0] + '^{commit}'], cwd=HERE.parents[1], text=True).strip()
    run_dir = Path(os.environ['QUALIFY_LOGS']).resolve() / ('deferred-' + sha[:12])
    source, receipts = run_dir / 'source', run_dir / 'receipts'
    extract(sha, source)
    receipts.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, QUALIFY_SOURCE=str(source), QUALIFY_RECEIPTS=str(receipts), QUALIFY_LOGS=str(run_dir), QUALIFY_COMMIT=sha,
               MOSS_HEAVY_GPU='0')
    summary_path = run_dir / 'summary.json'
    summary = json.loads(summary_path.read_text()) if summary_path.exists() else dict(commit=sha, items={})
    code = 0
    for name in names:
        if held():
            print('STOP: hold on the admission queue; no new submission', flush=True)
            code = 75
            break
        print('SUBMIT', name, ITEMS[name], 'GB', flush=True)
        with (run_dir / f'{name}-admission.log').open('w') as f:
            p = subprocess.run([os.environ['QUALIFY_ADMISSION'], str(ITEMS[name]), 'python3', str(HERE / 'deferred.py'), '--item', name],
                               stdout=f, stderr=subprocess.STDOUT, env=env)
        summary['items'][name] = dict(exit=p.returncode, reservation_gb=ITEMS[name])
        summary['results'] = results(receipts)
        summary_path.write_text(json.dumps(summary, indent=2, sort_keys=True) + '\n')
        print('COMPLETE', name, p.returncode, flush=True)
        code = code or p.returncode
        if p.returncode in STOP:
            print('STOPPED: admission refused or held', flush=True)
            break
    print(json.dumps(summary, indent=2, sort_keys=True))
    return code


if __name__ == '__main__':
    if len(sys.argv) == 3 and sys.argv[1] == '--item':
        sys.exit(item(sys.argv[2]))
    sys.exit(main(sys.argv[1:]))
