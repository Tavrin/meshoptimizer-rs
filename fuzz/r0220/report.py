"""Print seed-only line coverage using the compiler's matching llvm-cov."""
import json
import os
from pathlib import Path
import subprocess

root = Path(__file__).resolve().parents[2]
sysroot = Path(subprocess.check_output(['rustc', '+nightly', '--print', 'sysroot'], text=True).strip())
llvm = sysroot / 'lib/rustlib/x86_64-unknown-linux-gnu/bin/llvm-cov'
for target, sources in [('attr_page', ['attributes.rs', 'page.rs']), ('v2_build', ['v2.rs']), ('dag_format_v2', ['dag_format_v2.rs', 'far_field.rs'])]:
    binary = Path(os.environ['CARGO_TARGET_DIR']) / 'cov/x86_64-unknown-linux-gnu/release' / target
    profile = root / 'fuzz/r0220/coverage' / target / 'coverage.profdata'
    args = [str(llvm), 'report', str(binary), '-instr-profile=' + str(profile)]
    args += [str(root / 'src/clusterlod' / name) for name in sources]
    print(target, flush=True)
    subprocess.run(args, check=True)
    exported = json.loads(subprocess.check_output([str(llvm), 'export', str(binary), '-instr-profile=' + str(profile), '-summary-only']))
    for file in exported['data'][0]['files']:
        if Path(file['filename']).name in sources and '/src/clusterlod/' in file['filename']:
            lines = file['summary']['lines']
            print(json.dumps(dict(target=target, source=Path(file['filename']).name, **lines)), flush=True)
