"""Extend the established consumer to the 19-operation release guard."""
import hashlib,os,subprocess,sys
from pathlib import Path
R=Path.cwd();d=Path(sys.argv[1]);seed=Path(os.environ['GUARD_DRIVER'])
assert hashlib.sha256(seed.read_bytes()).hexdigest()=='e3ce993b023f9bebfbd6e6b3b22904091e1356e49444e59f6ae85125dfbbd6d9'
p=d/'src/main.rs';p.write_bytes(seed.read_bytes())
subprocess.run(['python3',str(R/'parity/0215-contract-meshlets/prepare_driver.py'),str(p)],check=True)
# Reuse the release guard's exact remap adapter (including first-reference remap setup).
s=(R/'parity/0215-basics/prepare_driver.py').read_text().replace("'   Self::", "'Self::")
sys.argv=[sys.argv[0],str(d)]
exec(compile(s,'prepare_driver.py','exec'),{'__name__':'__main__'})
s=p.read_text().replace('control("enable");for _','control("enable");let start=Instant::now();for _').replace('control("disable");println!("COUNTED")','let elapsed=start.elapsed().as_secs_f64();control("disable");println!("COUNTED {elapsed:.9}")');p.write_text(s)
# The original adapter disables SIMD for scalar release qualification. Restore
# actual default std/SIMD features for this registered default-build guard.
m=d/'Cargo.toml';s=m.read_text().replace('default-features = false, features = ["std", "clusterlod"]','features = ["clusterlod"]');m.write_text(s)
