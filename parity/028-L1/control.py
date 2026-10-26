#!/usr/bin/env python3
"""Admitted sequential builds and instruction-only diagnosis for SPEC-028 L1."""

# Public path placeholders are supplied by the caller's environment.
def _public_path(value):
    import os as _os
    for _name, _env in {'repo': 'REPO', 'scratch': 'SCRATCH', 'target-dir': 'CARGO_TARGET_DIR', 'worktree': 'WORKTREE', 'artifacts': 'MESHOPT_ARTIFACTS', 'refs': 'REFS', 'local': 'LOCAL', 'job-runner': 'JOB_RUNNER'}.items():
        _marker = '<' + _name + '>'
        if _marker in value:
            value = value.replace(_marker, _os.environ[_env])
    return _os.path.expanduser(value)

from pathlib import Path
import os, subprocess, sys, shutil, json, hashlib, select, signal, time
ROOT=Path(__file__).resolve().parents[2]; LANE=ROOT/'parity/028-L1'
ART=Path(_public_path('<scratch>/meshopt-028-L1'))
TARGET=Path(_public_path('<target-dir>'))
PERF='/usr/lib/linux-tools/6.17.0-22-generic/perf'
OPS=['partition','meshlets-scan','meshlets','meshlets-spatial','meshlets-flex','spatial-triangles']
CASES=['tiny-smooth','medium-smooth','medium-seams','medium-sparse','million-smooth']
def sha(p):return hashlib.sha256(p.read_bytes()).hexdigest()
def save(p,x):p.write_text(json.dumps(x,indent=2)+'\n')
def build(epoch,arms):
    assert os.environ.get('MOSS_HEAVY_ACTIVE')
    assert Path(os.environ['CARGO_TARGET_DIR'])==TARGET
    prior=ART/f'build-{epoch}.json'
    receipts=json.loads(prior.read_text())if prior.exists()else[]
    for arm in arms:
        manifest=(ART/'original-adapter/Cargo.toml') if epoch.startswith('original') else LANE/'Cargo.toml'
        if epoch=='profile':manifest=ART/'profile-adapter/Cargo.toml'
        if arm=='base':
            dst=ART/'baseline/parity/028-L1';shutil.copytree(LANE/'src',dst/'src',dirs_exist_ok=True)
            for f in ['Cargo.toml','Cargo.lock']:shutil.copy2(LANE/f,dst/f)
            manifest=dst/'Cargo.toml'
        features={'ours':'ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod','base':'ours,meshoptimizer-rs/experimental,meshoptimizer-rs/clusterlod','M7':'peer7','X':'optimesh'}[arm]
        env=dict(os.environ,RUSTC_WRAPPER='',RUSTC_WORKSPACE_WRAPPER='',RUSTFLAGS='',CARGO_ENCODED_RUSTFLAGS='',CARGO_INCREMENTAL='0',CARGO_BUILD_JOBS='1',CXXFLAGS='-DMESHOPTIMIZER_NO_SIMD')
        cmd=['cargo','build','--manifest-path',str(manifest),'--locked','--offline','--profile','moss','--features',features,'--bin','cmp-driver']
        sources=list((ROOT/'src').rglob('*.rs'))+list((LANE/'src').glob('*.rs'))
        before={str(p.relative_to(ROOT)):sha(p)for p in sources}
        log=ART/f'build-{epoch}-{arm}.log';assert not log.exists()
        with log.open('w')as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT)
        assert before=={str(p.relative_to(ROOT)):sha(p)for p in sources},'source changed during build'
        row={'env':{k:env.get(k,'')for k in ['CXXFLAGS','RUSTFLAGS','CARGO_ENCODED_RUSTFLAGS','CARGO_BUILD_JOBS','CARGO_PROFILE_MOSS_STRIP']},'arm':arm,'exit':r.returncode,'command':cmd,'admission':os.environ['MOSS_HEAVY_ACTIVE'],'source':{str(p.relative_to(ROOT)):sha(p)for p in list((ROOT/'src').rglob('*.rs'))+list((LANE/'src').glob('*.rs'))}}
        if r.returncode:print(log.read_text());raise SystemExit(r.returncode)
        dst=ART/f'driver-{epoch}-{arm}';shutil.copy2(TARGET/'moss/cmp-driver',dst);row['binary_sha256']=sha(dst)
        receipts.append(row);save(ART/f'build-{epoch}.json',receipts)
        print('built',epoch,arm,flush=True)
class Driver:
    def __init__(self,epoch,arm,op,case,cpu=2):
        self.output=ART/'outputs'/f'{epoch}-{arm}-{op}-{case}.bin'
        self.p=subprocess.Popen(['taskset','-c',str(cpu),str(ART/f'driver-{epoch}-{arm}'),op,str(ART/'inputs'/f'{case}.bin'),str(self.output)],stdin=subprocess.PIPE,stdout=subprocess.PIPE,stderr=subprocess.PIPE,text=True)
        assert self.read()=='READY'
    def read(self):
        assert select.select([self.p.stdout],[],[],180)[0],'timeout'
        s=self.p.stdout.readline().strip()
        assert s,self.p.stderr.read()
        return s
    def sample(self,n):self.p.stdin.write(str(n)+'\n');self.p.stdin.flush();return float(self.read())
    def close(self):
        self.p.stdin.write('stop\n');self.p.stdin.flush();self.p.communicate(timeout=180);assert self.p.returncode==0

def counts(epoch,arms):
    assert os.environ.get('MOSS_HEAVY_MEASUREMENT')=='1' and os.environ.get('MOSS_HEAVY_ACTIVE')
    rows=[];out=ART/f'counts-{epoch}.json';assert not out.exists()
    for op in OPS:
        for case in CASES:
            for arm in arms:
                d=Driver(epoch,arm,op,case)
                n=3 if case=='million-smooth' else 30 if case!='tiny-smooth' else 3000
                d.sample(1) # Warm code; discard returned wall value, not a wall sample.
                ctl_r,ctl_w=os.pipe();ack_r,ack_w=os.pipe()
                stat=ART/f'count-{epoch}-{arm}-{op}-{case}.stat'
                err=stat.with_suffix('.stderr')
                with err.open('w')as f:
                    p=subprocess.Popen([PERF,'stat','-e','instructions:u,cycles:u','-x',';','-o',str(stat),'-p',str(d.p.pid),'--delay=-1',f'--control=fd:{ctl_r},{ack_w}'],pass_fds=(ctl_r,ack_w),stdout=f,stderr=f)
                    os.close(ctl_r);os.close(ack_w)
                    def command(s):
                        os.write(ctl_w,(s+'\n').encode())
                        assert select.select([ack_r],[],[],15)[0],err.read_text()
                        assert os.read(ack_r,128).startswith(b'ack')
                    command('enable');d.sample(n);command('disable');p.send_signal(signal.SIGINT);p.wait(timeout=15)
                os.close(ctl_w);os.close(ack_r);d.close()
                lines=[s.split(';')for s in stat.read_text().splitlines()if 'instructions:u'in s]
                count=int(lines[0][0].replace(',',''))
                cycle_lines=[s.split(';')for s in stat.read_text().splitlines()if 'cycles:u'in s]
                cycles=int(cycle_lines[0][0].replace(',',''))if cycle_lines else None
                rows.append({'op':op,'case':case,'arm':arm,'iterations':n,'instructions':count,'cycles':cycles,'cycles_per_call':cycles/n if cycles else None,'per_call':count/n,'stat':str(stat),'perf_exit':p.returncode})
                save(out,rows)
            print('counts',epoch,op,case,flush=True)
def profile(epoch):
    assert os.environ.get('MOSS_HEAVY_MEASUREMENT')=='1'
    for op in OPS:
        d=Driver(epoch,'ours',op,'medium-smooth')
        dst=ART/f'profile-{op}.data'
        with (ART/f'profile-{op}.log').open('x')as f:
            p=subprocess.Popen([PERF,'record','-e','instructions:u','-c','100003','-o',str(dst),'-p',str(d.p.pid)],stdout=f,stderr=f)
            time.sleep(0.2);d.sample(150);p.send_signal(signal.SIGINT);p.wait(timeout=15)
        d.close()
        with (ART/f'profile-{op}.txt').open('x')as f:subprocess.run([PERF,'report','--stdio','--no-children','-i',str(dst)],stdout=f,stderr=subprocess.STDOUT,check=True)
        print('profile',op,flush=True)
if __name__=='__main__':
    if sys.argv[1]=='build':build(sys.argv[2],sys.argv[3:])
    elif sys.argv[1]=='counts':counts(sys.argv[2],sys.argv[3:])
    elif sys.argv[1]=='profile':profile(sys.argv[2])
