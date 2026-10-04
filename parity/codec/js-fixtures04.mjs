// Run the unchanged upstream encoder suite with a recording proxy, and the
// decoder suite's COLOR vectors, exporting 0.4 harness requests. Each encoder
// request keeps the shipped WASM encoder's output as `<id>.expected`.
import fs from 'node:fs';
import { pathToFileURL } from 'node:url';
const [reference, destination] = process.argv.slice(2);
const out = JSON.stringify(destination);

function header(prelude, source, name) {
	source = source.replace('var tests = {', prelude + '\nvar tests = {');
	source = source.replace('tests[key]();', 'active=key; await tests[key]();');
	source = source.replace(/\.then\(\(\) => \{/, '.then(async () => {');
	source = source.replace("console.log(count, 'tests passed');", `await Promise.all(pending);fs.writeFileSync(${out}+'/${name}-manifest.json',JSON.stringify(manifest,null,2));console.log(count,'awaited tests passed');`);
	return source;
}

const common = `
import fs from 'node:fs';
let active='',ordinal=0;const pending=[];const manifest=[];
function request(op,count,stride,data,mode,version,level){
 const bytes=Buffer.from(data.buffer,data.byteOffset,data.byteLength);
 const b=Buffer.alloc(44+bytes.length);b.write('MC02');
 [op,count,stride,mode,0,version,level,0,1,bytes.length].forEach((v,i)=>b.writeUInt32LE(v>>>0,4+i*4));bytes.copy(b,44);
 return b;
}
function save(prefix,b,expected,operation){
 const id=prefix+'-'+active+'-'+ordinal++;fs.writeFileSync(${out}+'/'+id+'.input',b);
 if(expected)fs.writeFileSync(${out}+'/'+id+'.expected',Buffer.from(expected.buffer,expected.byteOffset,expected.byteLength));
 manifest.push({case:id,operation});
}
function index32(source,size){return size==4?new Uint32Array(source.buffer,source.byteOffset,source.byteLength/4).slice():Uint32Array.from(new Uint16Array(source.buffer,source.byteOffset,source.byteLength/2));}
`;

// Encoder suite: capture every encoding call and its shipped-WASM output.
let encoder = fs.readFileSync(reference + '/js/meshopt_encoder.test.js', 'utf8');
encoder = encoder.replace("import { MeshoptEncoder as encoder } from './meshopt_encoder.js';", `import { MeshoptEncoder as actualEncoder } from ${JSON.stringify(pathToFileURL(reference + '/js/meshopt_encoder.js').href)};`);
encoder = encoder.replace("import { MeshoptDecoder as decoder } from './meshopt_decoder.mjs';", `import { MeshoptDecoder as decoder } from ${JSON.stringify(pathToFileURL(reference + '/js/meshopt_decoder.mjs').href)};`);
const encoderPrelude = common + `
const modes={Separate:0,SharedVector:1,SharedComponent:2,Clamped:3};
function capture(name,args,result){
 const [source,count,size]=args;
 const raw=new Uint8Array(source.buffer,source.byteOffset,source.byteLength);
 let b=null;
 if(name==='encodeVertexBuffer')b=request(11,count,size,raw.subarray(0,count*size),0,1,2);
 else if(name==='encodeVertexBufferLevel')b=request(11,count,size,raw.subarray(0,count*size),0,args[4]===undefined?1:args[4],args[3]);
 else if(name==='encodeIndexBuffer'||name==='encodeIndexSequence')b=request(name==='encodeIndexBuffer'?12:13,count,4,index32(raw,size),0,1,2);
 else if(name==='encodeGltfBuffer'){
  const mode=args[3];
  if(mode==='ATTRIBUTES')b=request(11,count,size,raw.subarray(0,count*size),0,args[4]===undefined?0:args[4],2);
  else b=request(mode==='TRIANGLES'?12:13,count,4,index32(raw,size),0,1,2);
 }
 else if(name==='encodeFilterOct'||name==='encodeFilterQuat'||name==='encodeFilterColor')
  b=request({encodeFilterOct:14,encodeFilterQuat:15,encodeFilterColor:17}[name],count,size,source,0,0,args[3]);
 else if(name==='encodeFilterExp')b=request(16,count,size,source,args[4]?modes[args[4]]:1,0,args[3]);
 if(b)save('js04',b,result,name);
}
const encoder=new Proxy(actualEncoder,{get(target,name){
 const value=target[name];if(typeof value!=='function')return value;
 return (...args)=>{const result=value.apply(target,args);if(name.startsWith('encode'))capture(name,args,result);return result;};
}});
`;
const encoderExport = destination + '/js04-encoder.mjs';
fs.writeFileSync(encoderExport, header(encoderPrelude, encoder, 'js04-encoder'));
await import(pathToFileURL(encoderExport).href);

// Decoder suite: the COLOR filter vectors that 0.2 recorded as outside its scope.
let decoder = fs.readFileSync(reference + '/js/meshopt_decoder.test.js', 'utf8');
decoder = decoder.replace("import { MeshoptDecoder as decoder } from './meshopt_decoder.mjs';", `import { MeshoptDecoder as actual } from ${JSON.stringify(pathToFileURL(reference + '/js/meshopt_decoder.mjs').href)};`);
const decoderPrelude = common + `
function capture(target,name,args){
 let count,stride,data,filter;
 if(name==='decodeGltfBuffer'||name==='decodeGltfBufferAsync'){if(name==='decodeGltfBufferAsync'){[count,stride,data,,filter]=args;}else{[,count,stride,data,,filter]=args;}}
 else{[,count,stride,data,filter]=args;}
 if(filter!=='COLOR')return;
 // Decode the attribute stream without a filter to obtain the filter input.
 const plain=new Uint8Array(count*stride);target.decodeVertexBuffer(plain,count,stride,data);
 save('js04',request(18,count,stride,plain,0,0,0),null,name);
}
const decoder=new Proxy(actual,{get(target,name){
 const value=target[name];if(typeof value!=='function')return value;
 return (...args)=>{if(name.startsWith('decode'))capture(target,name,args);const result=value.apply(target,args);if(result?.then)pending.push(result);return result;};
}});
`;
const decoderExport = destination + '/js04-decoder.mjs';
fs.writeFileSync(decoderExport, header(decoderPrelude, decoder, 'js04-decoder'));
await import(pathToFileURL(decoderExport).href);
