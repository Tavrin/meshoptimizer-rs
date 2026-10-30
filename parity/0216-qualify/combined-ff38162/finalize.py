"""Report, verify and clean the completed campaign under CPU admission.

Size: at most 65 CPU seconds, report assembly plus the 60-second final check.
No measurement or proof is repeated.
"""
import os
import runpy
import sys
import run as q

assert os.environ.get('MOSS_HEAVY_ACTIVE')
assert os.environ.get('MOSS_HEAVY_GPU') == '0'
q.run('assemble-report', [sys.executable, q.A / 'report.py'], estimate=5)
sys.argv = [str(q.A / 'finish.py'), '--cleanup']
runpy.run_path(str(q.A / 'finish.py'), run_name='__main__')
