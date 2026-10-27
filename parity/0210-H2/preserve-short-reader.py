"""Retain completed qualification and preserve the original short-record path."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import shutil,subprocess,json
R=Path(__file__).resolve().parents[2];S=Path(_public_path('<scratch>/meshopt-0210-H2'));D=S/'round1'
assert not D.exists();D.mkdir()
for name in ['samples','fuzz-artifacts','fuzz-results']:
 p=S/name
 if p.exists():p.rename(D/name)
for p in list(S.iterdir()):
 if p.is_file() and (p.suffix in ['.log','.json','.tsv'] or p.name.startswith('driver-') or '-exact-' in p.name):shutil.copy2(p,D/p.name)
shutil.copytree(R/'src',D/'src')
for name in ['Cargo.toml','Cargo.lock']:shutil.copy2(R/name,D/name)
(S/'gates.tsv').write_text('retained-initial-candidate-build\t101\nretained-pilot-2-regression\t3\n')
p=R/'src/input.rs';s=p.read_text();assert 'fn scale_full_records' not in s
s=s.replace('pub(crate) fn scale_records(', 'pub(crate) fn scale_full_records(',1)
t=subprocess.check_output(['git','show','v0.2.9:src/input.rs'],text=True)
a=t.index('    /// Fixed-stride cooker records');b=t.index('    /// Original packed coordinates',a)
original=t[a:b]
a=s.index('    /// Fixed-stride cooker records')
s=s[:a]+original+s[a:]
s=s.replace('''    /// Fixed-stride cooker records for scale's independent validation/reduction.
    /// Keep every other layout on its existing reader, including mapped views.
    pub(crate) fn scale_full_records''','''    /// Fully padded cooker records for a bounds-check-free inner iterator.
    /// Keep the original reader for short records to preserve validation order.
    pub(crate) fn scale_full_records''')
p.write_text(s)
p=R/'src/simplify.rs';s=p.read_text();a=s.index('fn scale_view(');b=s.index('\n    if let Some(records) = positions.scale_records()',a)
s=s[:b]+'''
    if let Some(records) = positions.scale_full_records() {
        return scale_records(records, positions.is_empty());
    }'''+s[b:];p.write_text(s)
subprocess.run(['rustfmt','--edition','2021',str(R/'src/input.rs'),str(p)],check=True)
(S/'reader-decision.json').write_text(json.dumps(dict(reason='Preserve the original short-record validation-return position; isolate the new full-record iterator in a private helper.',public_api_change=False,retained_round1='round1/'),indent=2)+'\n')
