"""Reuse the independent oracle and fixed 80-scene corpus without overwriting receipts."""
from support import *
assert not (RECEIPT/'oracle.json').exists(), 'Refusing to overwrite retained receipts'
script=(ROOT/'parity/0217-hsr/eval/oracle.py').read_text()
script=script.replace("receipt=lane/'eval'", "receipt=lane/'speed2'")
script=script.replace("scratch=target/'speed'", "scratch=target/'speed2-oracle'")
script=script.replace("'HEAD:src/hidden.rs'", "'f9859cc:src/hidden.rs'")
script=script.replace("else receipt/'bench.rs'", "else lane/'eval/bench.rs'")
script=script.split('# Same complete merged scenes/densities')[0]
exec(compile(script,str(ROOT/'parity/0217-hsr/eval/oracle.py'),'exec'))
