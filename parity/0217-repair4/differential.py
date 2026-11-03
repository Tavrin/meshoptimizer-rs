"""Existing protocol exact C++ checks, including simplification."""
import sys
from pathlib import Path
sys.path.insert(0,str(Path(__file__).resolve().parents[1]))
import runner
count=0
for name,p,i,_ in runner.fixtures():
 for op in [1,2,4,5,6]:
  for variant in range(8):
   data=runner.message(op,p,i,variant=variant)
   a=runner.execute(sys.argv[1],data);b=runner.execute(sys.argv[2],data)
   assert a==b,(name,op,variant)
   count+=1
print('PASS',count,'byte-exact C++/Rust protocol comparisons')
