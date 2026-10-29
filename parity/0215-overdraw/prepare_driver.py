"""Adapt the peer consumer in scratch: count instructions only, no wall timing."""
from pathlib import Path
import sys

p = Path(sys.argv[1])
s = p.read_text()
for old, new in [('let start=Instant::now();', ''),
                 ('let elapsed=start.elapsed().as_secs_f64();', ''),
                 ('println!("COUNTED {elapsed:.9}")', 'println!("COUNTED")')]:
    assert s.count(old) == 1, old
    s = s.replace(old, new)
p.write_text(s)
