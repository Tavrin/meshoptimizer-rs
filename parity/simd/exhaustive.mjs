import fs from 'node:fs';
const {instance}=await WebAssembly.instantiate(fs.readFileSync(process.argv[2]),{});
const [family,start,end]=process.argv.slice(3).map(Number);
const result=instance.exports.check_range(family,start,end-start);
if(result!==0n)throw new Error(`arithmetic mismatch index plus one: ${result}`);
if(instance.exports.check_edges()!==0n)throw new Error('edge mismatch');
console.log(JSON.stringify({family,start,end,records:end-start,mismatches:0,node:process.version}));
