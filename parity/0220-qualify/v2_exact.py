"""Compare weight-zero v2 bytes and direct build fields against the baseline release.

The driver is generated from determinism.rs into the disposable target, never into this directory.
Every source rewrite must match an exact number of times, so a drifted determinism.rs fails here with
a clear message instead of producing a driver that does not compile. Changed output is disclosed,
never normalized: the receipt lists each grid/case with its equality and first differing byte.
"""
import run as q

GRIDS = [12, 24, 48]
CASES = [0, 1, 2, 4]


def rewrite(text, old, new, count):
    found = text.count(old)
    assert found == count, f'determinism.rs drifted: expected {count} of {old!r}, found {found}'
    return text.replace(old, new)


def adapter(src):
    """determinism.rs with a grid-size parameter: `file(case)` reads the grid from bits 8.. of `case`."""
    s = rewrite(src, '#[no_mangle]\n', '', 2)
    s = rewrite(s, 'pub extern "C" fn run_case', 'pub fn run_case', 1)
    s = rewrite(s, '\nfn file(case: u32) -> Vec<u8> {\n', '\npub fn file(case: u32) -> Vec<u8> {\n    let n = (case >> 8) as usize;\n    let case = case & 255;\n', 1)
    s = rewrite(s, 'fn fixture() ->', 'fn fixture(n: usize) ->', 1)
    s = rewrite(s, '    let n = 12usize;\n', '', 1)
    # file() builds at the requested grid; the decode benchmark keeps its original 12-grid fixture.
    s = rewrite(s, 'let (p, ix, uv) = fixture();', 'let (p, ix, uv) = fixture(n);', 1)
    s = rewrite(s, 'let (p, ix, _) = fixture();', 'let (p, ix, _) = fixture(12);', 1)
    return s


DRIVER = '''#[path="before-v2.rs"] mod before;
#[path="after-v2.rs"] mod after;
fn main() {
    for n in %s { for case in %s {
        let i=(n << 8) | case;
        let a=before::file(i); let b=after::file(i);
        let first=a.iter().zip(&b).position(|(a,b)|a!=b);
        println!("grid={} case={} baseline_bytes={} candidate_bytes={} equal={} first_difference={:?} baseline_hash={:016x} candidate_hash={:016x}",n,case,a.len(),b.len(),a==b,first,before::run_case(i),after::run_case(i));
    }}
}
''' % ('[' + ','.join(f'{g}u32' for g in GRIDS) + ']', '[' + ','.join(f'{c}u32' for c in CASES) + ']')


def generate():
    s = adapter((q.A / 'determinism.rs').read_text())
    src = q.T / 'v2-exact-src'
    src.mkdir(parents=True, exist_ok=True)
    for arm, alias in [('before', 'baseline'), ('after', 'candidate')]:
        text = rewrite(s, 'use meshoptimizer_rs', f'use {alias}', 1)
        if arm == 'before':
            # The baseline predates the opt-in normal-error term; weight zero is its only behaviour.
            text = rewrite(text, '    m.normal_error_weight = weight;', '    let _ = weight;', 1)
        (src / (arm + '-v2.rs')).write_text(text)
    p = src / 'v2_exact.rs'
    p.write_text(DRIVER)
    return p


def parse(log):
    rows = []
    for line in log.read_text().splitlines():
        if line.startswith('grid='):
            row = dict(kv.split('=', 1) for kv in line.split())
            rows.append(dict(grid=int(row['grid']), case=int(row['case']), baseline_bytes=int(row['baseline_bytes']),
                             candidate_bytes=int(row['candidate_bytes']), equal=row['equal'] == 'true',
                             first_difference=row['first_difference'], baseline_hash=row['baseline_hash'],
                             candidate_hash=row['candidate_hash']))
    return rows


def main(build_only=False):
    q.baseline()
    p = generate()
    ok = True
    for mode, features in [('all', q.ALL), ('none', ['clusterlod'])]:
        d = q.T / ('v2-exact-' + mode)
        q.manifest(d, {'v2-exact': p}, features)
        q.run('v2-exact-build-' + mode, ['cargo', 'build', '--offline', '--release', '--manifest-path', d / 'Cargo.toml'])
        b = q.T / 'bin' / ('v2-exact-' + mode)
        q.shutil.copy2(q.T / 'release/v2-exact', b)
        q.write(q.OUT / ('v2-binding-' + mode + '.json'), dict(binary=q.sha(b), manifest=q.clean((d / 'Cargo.toml').read_text())))
        if build_only:
            continue
        rows = parse(q.run('v2-exact-run-' + mode, [b]))
        complete = len(rows) == len(GRIDS) * len(CASES)
        equal = sum(r['equal'] for r in rows)
        q.write(q.OUT / ('v2-exact-' + mode + '.json'), dict(baseline=q.BASE_TAG, rows=rows, complete=complete, equal=equal, total=len(rows)))
        print(f'v2-exact {mode}: {equal}/{len(rows)} grid/case pairs byte-identical to {q.BASE_TAG}', flush=True)
        ok = ok and complete and equal == len(rows)
    return ok


if __name__ == '__main__':
    import sys
    sys.exit(0 if main(build_only='--build-only' in sys.argv) else 1)
