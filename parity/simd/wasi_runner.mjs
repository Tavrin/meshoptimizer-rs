// Cargo runner for wasm32-wasip1 test binaries: executes them under Node's WASI.
import { readFile } from 'node:fs/promises';
import { argv, env, exit } from 'node:process';
import { WASI } from 'node:wasi';

const [binary, ...args] = argv.slice(2);
const wasi = new WASI({ version: 'preview1', args: [binary, ...args], env, returnOnExit: true });
const module = await WebAssembly.compile(await readFile(binary));
const instance = await WebAssembly.instantiate(module, wasi.getImportObject());
exit(wasi.start(instance));
