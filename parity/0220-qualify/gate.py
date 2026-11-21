"""Run a single required gate within caller-owned admission."""
import run as q
import sys
commands = {
 'fmt': ['cargo','fmt','--check'],
 'clippy': ['cargo','clippy','--all-targets','--all-features','--','-D','warnings'],
 'release': ['cargo','test','--release'],
 'release-all-features': ['cargo','test','--offline','--release','--all-features'],
 'doc': ['cargo','doc','--no-deps','--all-features'],
 'public-text': ['bash','parity/tools/check_public_text.sh'],
}
name=sys.argv[1]
gate_env={'RUSTDOCFLAGS':'-D warnings'} if name=='doc' else {'RUST_TEST_THREADS':'2'} if name in ['release','release-all-features'] else None
# The public-text guard inspects the harness worktree (it needs git); every other gate runs in the source under test.
q.run(name, commands[name], env=gate_env, check=name!='fmt', cwd=q.HARNESS if name=='public-text' else None)
if name=='fmt':
 import subprocess
 sys.exit(subprocess.run(['python3',str(q.A/'bundle.py')]).returncode)
