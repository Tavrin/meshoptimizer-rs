// usage: node determinism_wasm.mjs <determinism.wasm> <case count>; prints "<index> <hash>" per case.
import { readFileSync } from 'node:fs';

const bytes = readFileSync(process.argv[2]);
const { instance } = await WebAssembly.instantiate(bytes, {});
const count = Number(process.argv[3]);
for (let i = 0; i < count; i++) {
  const hash = BigInt.asUintN(64, instance.exports.run_case(i));
  console.log(i, hash.toString(16).padStart(16, '0'));
}
