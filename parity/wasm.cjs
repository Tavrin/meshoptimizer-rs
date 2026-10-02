// Execute the exact native binary protocol through safe byte exports.
const fs = require('fs');
const readline = require('readline');
(async () => {
  const {instance} = await WebAssembly.instantiate(fs.readFileSync(process.argv[2]), {});
  const api = instance.exports;
  for await (const line of readline.createInterface({input: process.stdin, crlfDelay: Infinity})) {
    const input = Buffer.from(line, 'base64');
    if (api.request(input.length)) throw new Error('WASM request rejected');
    for (let i=0; i<input.length; ++i) if (api.put_byte(i,input[i])) throw new Error('WASM input rejected');
    if (api.run()) throw new Error('WASM execution failed');
    const output = Buffer.alloc(api.output_len());
    for (let i=0; i<output.length; ++i) { const b=api.get_byte(i); if (b>255) throw new Error('WASM output bounds'); output[i]=b; }
    process.stdout.write(output.toString('base64')+'\n');
  }
})().catch(e => {console.error(e);process.exitCode=1;});
