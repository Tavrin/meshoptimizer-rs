import fs from 'node:fs';
import assert from 'node:assert/strict';
import readline from 'node:readline';
// Nanosecond monotonic clock; timed batches are calibrated externally.
const {MeshoptDecoder:cpp}=await import(process.argv[4]);await cpp.ready;
const rust=(await WebAssembly.instantiate(fs.readFileSync(process.argv[2]),{})).instance.exports;
const scalar=(await WebAssembly.instantiate(fs.readFileSync(process.argv[3]),{})).instance.exports;
const filters=['NONE','OCTAHEDRAL','QUATERNION','EXPONENTIAL'];
for await (const line of readline.createInterface({input:process.stdin,crlfDelay:Infinity})) {
    const q=JSON.parse(line),b=Buffer.from(q.input,'base64');
    const op=b.readUInt32LE(4),count=b.readUInt32LE(8),stride=b.readUInt32LE(12),mode=b.readUInt32LE(16)&127,filter=b.readUInt32LE(20),source=b.subarray(44),bytes=count*stride;
    for(const api of [rust,scalar])if(api.bench_prepare(source.length,op,count,stride,mode,filter))throw new Error('Rust preparation failed');
    let output=new Uint8Array(bytes);
    const funcs={
        rust:()=>{
            new Uint8Array(rust.memory.buffer,rust.bench_source_ptr(),source.length).set(source);
            if(rust.bench_run(q.into))throw new Error('Rust decode failed');
            if(!q.into)output=new Uint8Array(bytes);
            output.set(new Uint8Array(rust.memory.buffer,rust.bench_output_ptr(),bytes));
        },
        scalar:()=>{
            new Uint8Array(scalar.memory.buffer,scalar.bench_source_ptr(),source.length).set(source);
            if(scalar.bench_run(q.into))throw new Error('scalar decode failed');
            if(!q.into)output=new Uint8Array(bytes);
            output.set(new Uint8Array(scalar.memory.buffer,scalar.bench_output_ptr(),bytes));
        },
        cpp:()=>{
            if(!q.into)output=new Uint8Array(bytes);
            if(op===1)cpp.decodeVertexBuffer(output,count,stride,source);
            else if(op===2)cpp.decodeIndexBuffer(output,count,stride,source);
            else if(op===3)cpp.decodeIndexSequence(output,count,stride,source);
            else cpp.decodeGltfBuffer(output,count,stride,source,['ATTRIBUTES','TRIANGLES','INDICES'][mode],filters[filter]);
        }
    };
    // Setup/warm-up, compilation, message parsing and I/O excluded.
    funcs.cpp();const expected=Buffer.from(output);
    for(const f of Object.values(funcs)){f();assert.deepEqual(Buffer.from(output),expected);}
    const order=['rust','cpp','scalar'];let shift=q.pair%3;order.push(...order.splice(0,shift));
    const times={};
    for(const k of order){const began=process.hrtime.bigint();for(let i=0;i<q.iterations;i++)funcs[k]();times[k]=Number(process.hrtime.bigint()-began)/1e9/q.iterations;}
    process.stdout.write(JSON.stringify({times,order,node:process.version})+'\n');
}
