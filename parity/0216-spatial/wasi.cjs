// Execute the focused Rust kernel tests in Node's WASI runtime.
const fs = require('node:fs');
const { WASI } = require('node:wasi');
const wasi = new WASI({version: 'preview1', args: process.argv.slice(2), env: {}, preopens: {}});
WebAssembly.instantiate(fs.readFileSync(process.argv[2]), {wasi_snapshot_preview1: wasi.wasiImport})
  .then(({instance}) => { process.exitCode = wasi.start(instance); })
  .catch(error => { console.error(error); process.exitCode = 1; });
