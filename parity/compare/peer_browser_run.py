#!/usr/bin/env python3
"""Reuse REPORT-3's frozen streams, Rust/JS arm wrappers and headless CDP runner."""
import pathlib,os,sys,json,subprocess,shutil,hashlib
from peer_screen import boundary,event,ART,ROOT
assert os.environ.get('MOSS_HEAVY_ACTIVE')
OLD=pathlib.Path('/mnt/linux-extra/moss-scratch/meshopt-web-decode/speed')
phase=sys.argv[1]
for profile in (sys.argv[2:] or ['defaults','moss']):
 root=ART/f'browser-{profile}';(root/'data').mkdir(parents=True,exist_ok=True);(root/'out').mkdir(exist_ok=True);(root/'references').mkdir(exist_ok=True)
 if not (root/'data/streams.json').exists():
  streams=json.loads((OLD/'data/streams.json').read_text());binary=(OLD/'data/streams.bin').read_bytes();assert len(streams)==42
  for k,s in enumerate(streams):
   boundary();raw=binary[s['offset']:s['offset']+s['encoded_len']];src=root/'references'/f'{k}.encoded';dst=root/'references'/f'{k}.bin';src.write_bytes(raw)
   subprocess.run([str(ART/'peer-stream-ref'),str(s['count']),str(s['stride']),s['mode'],str(src),str(dst)],check=True)
   s['reference_file']=f'references/{k}.bin';s['reference_sha256']=hashlib.sha256(dst.read_bytes()).hexdigest();s['encoded_sha256']=hashlib.sha256(raw).hexdigest()
  (root/'data/streams.json').write_text(json.dumps(streams,indent=2));(root/'data/streams.bin').write_bytes(binary)
  for arm in ['O','X']:shutil.copy2(ART/f'peer-{profile}-{arm}.wasm',root/'out'/f'{arm}.wasm')
  shutil.copy2(OLD/'out/upstream-simd.wasm',root/'out/JS.wasm')
  (root/'data/arms.json').write_text(json.dumps([{'name':a,'kind':'js'if a=='JS'else'rust','file':f'out/{a}.wasm'}for a in ['O','X','JS']]))
  shutil.copy2(ROOT/'parity/compare/peer_browser.mjs',root/'harness.mjs')
  page=(OLD/'page.html').read_text().replace('const bytesOnly = !!mode.bytes_only;','const bytesOnly = !!mode.bytes_only;\nconst existing=await(await fetch("./data/resume.json")).json();').replace('{ warmup: 5, repeats: 15, bytesOnly }','{ bytesOnly, existing }');(root/'page.html').write_text(page)
  drive=(OLD/'drive.mjs').read_text().replace('const root = "/mnt/linux-extra/moss-scratch/meshopt-web-decode/speed";',f'const root = {json.dumps(str(root))};')
  drive=drive.replace('const http = spawn("python3", ["-m", "http.server", String(httpPort), "--bind", "127.0.0.1"],',f'const http = spawn("python3", [{json.dumps(str(ROOT/"parity/compare/peer_http.py"))}, root],')
  drive=drive.replace('"-c", "2"','"-c", process.env.PEER_CPU').replace('v !== "match"','v !== "identical"')
  drive=drive.replace('const tab = await getJson("http://127.0.0.1:" + port + "/json/new?" + url, "PUT");','const tab = await getJson("http://127.0.0.1:" + port + "/json/new?about:blank", "PUT");')
  drive=drive.replace('    ws.addEventListener("message", (event) => {','    let loadResolve; const loaded=new Promise(resolve=>loadResolve=resolve);\n    ws.addEventListener("message", (event) => {')
  drive=drive.replace('        const msg = JSON.parse(event.data);','        const msg = JSON.parse(event.data);\n        if(msg.method==="Page.loadEventFired")loadResolve();')
  drive=drive.replace('    await send("Runtime.enable", {});','    await send("Page.enable", {});\n    await send("Runtime.enable", {});\n    await send("Page.navigate", {url});\n    await Promise.race([loaded, new Promise((_,reject)=>setTimeout(()=>reject(Error("page load timeout")),30000))]);')
  (root/'drive.mjs').write_text(drive)
 out=root/'data'/('chromium-bytes.json'if phase=='correctness'else'chromium-timing.json')
 if out.exists() and not json.loads(out.read_text()).get('paused'):continue
 if out.exists():
  prior=json.loads(out.read_text());resume=prior['rows'];out.rename(out.with_suffix(f'.pause-{len(list(out.parent.glob("*.pause-*.json")))}.json'))
 else:resume=[]
 # Recover completed streams from durable browser receipts after a PAUSE.
 if (root/'browser-progress.jsonl').exists() and phase=='timing':
  for line in (root/'browser-progress.jsonl').read_text().splitlines():
   r=json.loads(line)
   if r.get('event')=='stream-complete' and r['row'].get('peers'):
    resume=[x for x in resume if x['id']!=r['row']['id']]+[r['row']]
 (root/'data/resume.json').write_text(json.dumps(resume));(root/'data/mode.json').write_text(json.dumps({'bytes_only':phase=='correctness'}))
 env=dict(os.environ,PEER_CPU=str(json.loads((ART/'epoch.json').read_text())['core']['cpu']))
 cmd=['node',str(root/'drive.mjs')]+(['--bytes-only']if phase=='correctness'else[])
 attempt=0
 while True:
  with (root/f'{phase}-attempt-{attempt}.log').open('w')as f:r=subprocess.run(cmd,env=env,stdout=f,stderr=subprocess.STDOUT,timeout=720)
  if r.returncode==0:break
  event(event='browser-infrastructure-failure',profile=profile,phase=phase,attempt=attempt,exit=r.returncode)
  if attempt:raise SystemExit(r.returncode)
  attempt+=1
 data=json.loads(out.read_text())
 if data.get('paused'):raise SystemExit(75)
 assert len(data['rows'])==42
 if phase=='timing':assert (root/'data/chromium-bytes.json').exists()
 print(profile,phase,'complete',flush=True)
