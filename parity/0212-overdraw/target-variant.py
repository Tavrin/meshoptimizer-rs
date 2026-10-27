"""Omit only the target guard already proved unreachable by prepaid admission."""
from pathlib import Path
import hashlib,json,os,shutil,subprocess
A=Path(os.environ['MESHOPT_OVERDRAW_SCRATCH']);d=A/'variant-I';assert not d.exists()
shutil.copytree(A/'variant-H',d)
p=d/'src/overdraw.rs';s=p.read_text()
s=s.replace('trait ClusterMeter {','''trait ClusterMeter {
    fn target(&self, value: f32) -> Result<f32, Error>;''',1)
s=s.replace('impl ClusterMeter for Work {','''impl ClusterMeter for Work {
    #[inline(always)]
    fn target(&self, value: f32) -> Result<f32, Error> { finite(value) }''',1)
s=s.replace('impl ClusterMeter for ClusterPrepaid {','''impl ClusterMeter for ClusterPrepaid {
    #[inline(always)]
    fn target(&self, value: f32) -> Result<f32, Error> { Ok(value) }''',1)
assert s.count('finite(threshold *')==1
s=s.replace('finite(threshold *','work.target(threshold *',1)
p.write_text(s)
shutil.copytree(A/'harness-H',A/'harness-I')
p=A/'harness-I/Cargo.toml';p.write_text(p.read_text().replace('variant-H','variant-I'))
(A/'I.patch').write_text(subprocess.run(['diff','-ru',str(A/'base/src'),str(d/'src')],capture_output=True,text=True).stdout)
(A/'target-sources.json').write_text(json.dumps({str(p):hashlib.sha256(p.read_bytes()).hexdigest() for p in (d/'src').rglob('*.rs')},indent=2)+'\n')
