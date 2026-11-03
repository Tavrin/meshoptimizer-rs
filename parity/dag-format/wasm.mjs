import { WASI } from 'node:wasi';
import fs from 'node:fs';
const wasi = new WASI({version:'preview1',args:["dag_identity",...process.argv.slice(3)],env:{},preopens:{".":process.cwd()}});
const module = await WebAssembly.compile(fs.readFileSync(process.argv[2]));
const instance = await WebAssembly.instantiate(module,wasi.getImportObject());
wasi.start(instance);
