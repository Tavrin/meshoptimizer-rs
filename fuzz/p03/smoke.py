#!/usr/bin/env python3
"""Build cargo-fuzz targets and run one 300-second coverage smoke per function."""
import concurrent.futures,hashlib,json,os,platform,re,subprocess,time
from pathlib import Path
ROOT=Path(__file__).resolve().parents[2];TARGET=Path(os.environ['CARGO_TARGET_DIR']);ART=Path(os.environ.get('MESHOPT_ARTIFACTS','/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p03'));ART.mkdir(parents=True,exist_ok=True)
TOOL=Path('/mnt/linux-extra/moss-cargo-targets/meshopt-artifacts/p02/tools/bin/cargo-fuzz')
ENV={**os.environ,'CARGO_HOME':str(TARGET/'cargo-home'),'CARGO_NET_OFFLINE':'true','RUSTC_BOOTSTRAP':'1','RUSTC_WRAPPER':'','RUSTC_WORKSPACE_WRAPPER':'','RUSTFLAGS':'','CARGO_ENCODED_RUSTFLAGS':''}
ENV.pop("CARGO_ENCODED_RUSTFLAGS",None)
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def sources():return {str(p.relative_to(ROOT)):sha(p) for base in [ROOT/'Cargo.toml',ROOT/'Cargo.lock',ROOT/'src',ROOT/'fuzz/p03'] for p in ([base] if base.is_file() else sorted(base.rglob('*'))) if p.is_file() and p.suffix in {'.rs','.py','.lock','.toml'}}
def main():
 cpu_record=ART/'benchmark/cpu.json'
 if cpu_record.exists():
  siblings=set(json.loads(cpu_record.read_text())['siblings']);os.sched_setaffinity(0,os.sched_getaffinity(0)-siblings)
 before=sources();subprocess.run([str(TOOL),'fuzz','build','--fuzz-dir',str(ROOT/'fuzz/p03'),'--target-dir',str(TARGET),'--sanitizer','none'],check=True,env=ENV,cwd=ROOT)
 if before!=sources():raise RuntimeError('source changed while building')
 names=[p.stem for p in sorted((ROOT/'fuzz/p03/fuzz_targets').glob('*.rs'))];folder=ART/'fuzz';folder.mkdir(exist_ok=True)
 binaries={n:TARGET/'x86_64-unknown-linux-gnu/release'/n for n in names};hashes={n:sha(p) for n,p in binaries.items()}
 def run(name):
  corpus=folder/name/'corpus';corpus.mkdir(parents=True,exist_ok=True);failure=folder/name/'failures';failure.mkdir(exist_ok=True);(corpus/'seed').write_bytes(bytes([20,20,0,0,0,63,63,16,64,32])+bytes(range(256))*4)
  usage=folder/name/'usage.json';log=folder/name/'run.log';start=time.time()
  with log.open('wb') as stream:
   cmd=['/usr/bin/time','-o',str(usage),'-f','{"user_seconds":%U,"system_seconds":%S,"elapsed_seconds":%e,"max_rss_kib":%M}',str(TOOL),'fuzz','run','--fuzz-dir',str(ROOT/'fuzz/p03'),'--target-dir',str(TARGET),'--sanitizer','none',name,str(corpus),'--','-max_total_time=300','-max_len=4096','-timeout=10','-rss_limit_mb=512','-seed=20261004','-print_final_stats=1',f'-artifact_prefix={failure}/']
   process=subprocess.run(cmd,env=ENV,cwd=ROOT,stdout=stream,stderr=subprocess.STDOUT)
  text=log.read_text();match=re.search(r'stat::number_of_executed_units:\s*(\d+)',text)
  record={'target':name,'exit':process.returncode,'seconds':time.time()-start,'command':cmd,'statistics':json.loads(usage.read_text()),'executions':int(match[1]) if match else None,'instrumented':bool(re.search(r'Loaded \d+ modules? .*inline 8-bit counters',text)),'coverage_lines':re.findall(r'.*cov:.*ft:.*',text)[-3:],'log_sha256':sha(log),'corpus':{p.name:sha(p) for p in corpus.iterdir() if p.is_file()}}
  print(name,process.returncode,record['executions'],flush=True);return record
 with concurrent.futures.ThreadPoolExecutor(max_workers=15) as pool:runs=list(pool.map(run,names))
 record={'schema':'meshopt-p03-fuzz/1','profile':'cargo-fuzz 0.13.2; libfuzzer-sys 0.4.13; coverage and trace-compares; sanitizer none; stable compiler with RUSTC_BOOTSTRAP=1','sources':before,'executables':hashes,'cargo_fuzz_sha256':sha(TOOL),'rustc':subprocess.check_output(['rustc','-Vv'],text=True),'hardware':{k:v for k,v in platform.uname()._asdict().items() if k!='node'},'runs':runs,'unchanged':before==sources() and hashes=={n:sha(p) for n,p in binaries.items()}}
 (folder/'record.json').write_text(json.dumps(record,indent=2)+'\n')
 if not record['unchanged'] or any(r['exit']!=0 or not r['instrumented'] or not r['executions'] or r['statistics']['elapsed_seconds']<300 for r in runs):raise SystemExit('fuzz gate failed')
if __name__=='__main__':main()
