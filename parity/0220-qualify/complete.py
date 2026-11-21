"""Run the 9 GB all-features test/doctest, then record and clean up."""
import os,subprocess,sys
from pathlib import Path
A=Path(__file__).resolve().parent
assert os.environ.get('MOSS_HEAVY_ACTIVE') and os.environ.get('MOSS_HEAVY_GPU')=='0'
test=subprocess.run(['python3',str(A/'gate.py'),'release-all-features'])
public=subprocess.run(['python3',str(A/'gate.py'),'public-text'])
finish=subprocess.run(['python3',str(A/'finish.py')])
sys.exit(test.returncode or public.returncode or finish.returncode)
