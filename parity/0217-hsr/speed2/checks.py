"""Repeat existing full gates with a separate receipt directory."""
from support import *
assert not (RECEIPT/'checks/exits.tsv').exists(), 'Refusing to overwrite retained receipts'
script=(ROOT/'parity/0217-hsr/eval/checks.py').read_text()
script=script.replace("receipt=lane/'eval' / 'checks'", "receipt=lane/'speed2' / 'checks'")
script=script.replace("scratch=target/'proof'", "scratch=target/'speed2-proof'")
exec(compile(script,str(ROOT/'parity/0217-hsr/eval/checks.py'),'exec'))
