import {readFileSync} from 'node:fs';
import {performance} from 'node:perf_hooks';
const {instance}=await WebAssembly.instantiate(readFileSync(process.argv[2]),{});
const run=instance.exports.decode_bench,n=100000;
run(1000);let times=[];let checks=[];
for(let i=0;i<7;i++){const start=performance.now();checks.push(String(run(n)));times.push((performance.now()-start)*1e6/n);}
times.sort((a,b)=>a-b);
console.log(JSON.stringify({iterations:n,repeats:7,median_ns_per_page:times[3],min_ns_per_page:times[0],max_ns_per_page:times[6],checks,fixture:'288 triangles, two clusters with 122 and 61 positions, compressed page; transcode plus position and triangle decode; setup amortized per batch',node:process.version}));
