#!/usr/bin/env python3
"""Instruction/cycle diagnostics only. No wall values are retained or used."""
import os,subprocess,select,signal,json,math,statistics
from control import ART,Driver,PERF,CASES,save,sha
assert os.environ.get('PARITY_RUNNER_ACTIVE') and os.environ.get('PARITY_RUNNER_MEASUREMENT')=='1'
def count(d,n,stem):
    cr,cw=os.pipe();ar,aw=os.pipe();stat=ART/(stem+'.stat')
    with (ART/(stem+'.stderr')).open('x')as err:
        p=subprocess.Popen([PERF,'stat','-e','instructions:u,cycles:u','-x',';','-o',str(stat),'-p',str(d.p.pid),'--delay=-1',f'--control=fd:{cr},{aw}'],pass_fds=(cr,aw),stdout=err,stderr=err)
        os.close(cr);os.close(aw)
        def ctl(command):
            os.write(cw,(command+'\n').encode());assert select.select([ar],[],[],15)[0];assert os.read(ar,128).startswith(b'ack')
        ctl('enable');d.sample(n);ctl('disable');p.send_signal(signal.SIGINT);p.wait(timeout=15)
    os.close(cw);os.close(ar)
    values={}
    for line in stat.read_text().splitlines():
        x=line.split(';')
        if len(x)>2 and x[2]in['cycles:u','instructions:u']:values[x[2]]=int(x[0])/n
    assert len(values)==2
    return values
rows=[]
for case in CASES:
    pool=[Driver('cyclebase','ours','meshlets-spatial',case),Driver('cyclebase','ours','meshlets-spatial',case),Driver('trial6','ours','meshlets-spatial',case)]
    n=5000 if case=='tiny-smooth' else 1 if case=='million-smooth' else 16
    for d in pool:d.sample(n)
    try:
        for mode,index in [('AA',1),('AB',2)]:
            pairs=[]
            for i in range(5):
                if i%2==0:a=count(pool[0],n,f'counter-{case}-{mode}-{i}-A');b=count(pool[index],n,f'counter-{case}-{mode}-{i}-B')
                else:b=count(pool[index],n,f'counter-{case}-{mode}-{i}-B');a=count(pool[0],n,f'counter-{case}-{mode}-{i}-A')
                pairs.append({'A':a,'B':b})
            gm=lambda event:math.exp(statistics.mean(math.log(p['B'][event]/p['A'][event])for p in pairs))
            rows.append({'case':case,'mode':mode,'pairs':pairs,'instructions_ratio':gm('instructions:u'),'cycles_ratio':gm('cycles:u'),'old_binary':sha(ART/'driver-cyclebase-ours'),'new_binary':sha(ART/'driver-trial6-ours')});save(ART/'counter-pairs.json',rows)
        print('counter pairs',case,flush=True)
    finally:
        for d in pool:d.close()
