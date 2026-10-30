"""Final staged evidence review; CPU admission required, size <= 30 CPU s."""
import json
import os
from pathlib import Path
import resource
import subprocess

assert os.environ.get('MOSS_HEAVY_ACTIVE')
assert os.environ.get('MOSS_HEAVY_GPU') == '0'
a = Path(__file__).resolve().parent
results = {'upper_cpu_seconds_estimate': 30, 'checks': {}}
commands = {
    'full_staged_whitespace': ['git', 'diff', '--cached', '--check'],
    'staged_except_raw_logs': ['git', 'diff', '--cached', '--check', '--', '.',
                               ':!parity/0216-qualify/combined-ff38162/logs/*.log'],
    'public_text': ['bash', 'parity/tools/check_public_text.sh'],
    'source_unchanged': ['git', 'diff', '--exit-code', 'ff38162', '--', 'src', 'Cargo.toml', 'Cargo.lock'],
}
for name, command in commands.items():
    p = subprocess.run(command, text=True, capture_output=True)
    results['checks'][name] = {'exit': p.returncode, 'stdout': p.stdout, 'stderr': p.stderr}
usage = resource.getrusage(resource.RUSAGE_CHILDREN)
own = resource.getrusage(resource.RUSAGE_SELF)
results['cpu_seconds'] = usage.ru_utime + usage.ru_stime + own.ru_utime + own.ru_stime
(a / 'staged-review.json').write_text(json.dumps(results, indent=2) + '\n')
raw = results['checks']['full_staged_whitespace']
assert raw['exit'] == 2
assert raw['stdout'].count('new blank line at EOF.') == 6
assert all('/logs/' in line and 'new blank line at EOF.' in line
           for line in raw['stdout'].splitlines())
assert all(x['exit'] == 0 for k, x in results['checks'].items() if k != 'full_staged_whitespace')
assert results['cpu_seconds'] <= 30
print('PASS: staged source/helpers/docs and public text; six raw log EOF blanks retained')
