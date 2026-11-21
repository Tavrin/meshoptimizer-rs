import os,json,subprocess
from pathlib import Path
r=Path(os.environ["VG_CONTINUATION"]);e=Path(os.environ["VG_EVIDENCE"])
cores=sorted(os.sched_getaffinity(0))
def run(name,cmd):
    with (r/(name+".log")).open("w") as f:
        rc=subprocess.run(list(map(str,cmd)),stdout=f,stderr=subprocess.STDOUT).returncode
    with (r/"profile-exits.jsonl").open("a") as f:f.write(json.dumps(dict(name=name,exit=rc,command=list(map(str,cmd))))+"\n")
    return rc
run("candidate-ibs-dragon-1T",["perf","record","-a","-C",cores[0],"-e","ibs_op//u","-c",100003,"-o",r/"candidate-ibs.perf","--","taskset","-c",cores[0],r/"candidate-vg-bench","build","ours-v2",e/"dragon.vgm","--threads",1])
for arm,binary in [("previous",e/"candidate-vg-bench"),("candidate",r/"candidate-vg-bench")]:
    for threads,cpus in [(1,str(cores[0])),(8,",".join(map(str,cores[:8])))]:
        name=f"{arm}-cycles-dragon-{threads}T"
        data=r/(name+".perf")
        if run(name,["taskset","-c",cpus,"perf","record","-e","cycles:u","-F",997,"-o",data,"--",binary,"build","ours-v2",e/"dragon.vgm","--threads",threads])==0:
            run(name+"-report",["perf","report","-i",data,"--stdio","--no-children","--percent-limit",0.5])
