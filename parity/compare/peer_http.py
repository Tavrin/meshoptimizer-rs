#!/usr/bin/env python3
"""Loopback-only browser serving and durable per-pair receipts."""
import http.server,json,os,sys,pathlib
from peer_screen import train_waiters
ROOT=pathlib.Path(sys.argv[1]);os.chdir(ROOT)
class Handler(http.server.SimpleHTTPRequestHandler):
 def do_GET(self):
  if self.path=='/train-waiters':
   b=json.dumps(train_waiters()).encode();self.send_response(200);self.send_header('Content-Type','application/json');self.end_headers();self.wfile.write(b)
  else:super().do_GET()
 def do_POST(self):
  if self.path!='/receipt':self.send_error(404);return
  b=self.rfile.read(int(self.headers['Content-Length']));json.loads(b)
  with (ROOT/'browser-progress.jsonl').open('ab')as f:f.write(b+b'\n');f.flush();os.fsync(f.fileno())
  self.send_response(200);self.end_headers()
 def log_message(self,*a):pass
http.server.HTTPServer(('127.0.0.1',8766),Handler).serve_forever()
