// Fixed-call, matched-transport instruction probe; deliberately no clock calls.
import {readFileSync} from 'node:fs';
import {loadArms} from './harness.mjs';
const root = '<scratch>/meshopt-decode/browser';
const [name, index, calls] = process.argv.slice(2);
const manifest = {
    'before-scalar': {kind:'rust', file:'before-scalar.wasm'},
    'codec1-scalar': {kind:'rust', file:'codec1-scalar.wasm'},
    'codec2-scalar': {kind:'rust', file:'codec2-scalar.wasm'},
    'after-scalar': {kind:'rust', file:'after-scalar.wasm'},
    'upstream-scalar': {kind:'js', file:'upstream-scalar.wasm'},
};
const spec=manifest[name];
const [arm]=await loadArms([{name,kind:spec.kind,bytes:readFileSync(root+'/out/'+spec.file)}]);
const streams=JSON.parse(readFileSync(root+'/data/streams.json'));
const all=new Uint8Array(readFileSync(root+'/data/streams.bin'));
const stream=streams[Number(index)];
const src=all.subarray(stream.offset,stream.offset+stream.encoded_len);
const mode=stream.mode==='ATTRIBUTES'?0:1;
arm.prepare(src,stream.count*stream.stride);
function call(){return arm.call(stream.count,stream.stride,mode,0,src);}
const expected=spec.kind==='rust'?stream.count*stream.stride:true;
if(call()!==expected)throw new Error('decode');
for(let i=0;i<Number(calls);i++)if(call()!==expected)throw new Error('decode');
arm.release();
