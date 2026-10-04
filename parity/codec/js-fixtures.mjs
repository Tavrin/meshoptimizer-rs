import fs from 'node:fs';
import {pathToFileURL} from 'node:url';
const [reference,destination]=process.argv.slice(2);
let source=fs.readFileSync(reference+'/js/meshopt_decoder.test.js','utf8');
source=source.replace("import { MeshoptDecoder as decoder } from './meshopt_decoder.mjs';",`import { MeshoptDecoder as actual } from ${JSON.stringify(pathToFileURL(reference+'/js/meshopt_decoder.mjs').href)};`);
const prelude=`
import fs from 'node:fs';
let active='',ordinal=0;const pending=[];const manifest=[];
function capture(name,args){
 let count,stride,data,mode=0,filter=0,op;
 const filters={NONE:0,OCTAHEDRAL:1,QUATERNION:2,EXPONENTIAL:3};
 if(name==='decodeGltfBufferAsync'){[count,stride,data,mode,filter]=args;}
 else {[,count,stride,data]=args;if(name==='decodeGltfBuffer'){mode=args[4];filter=args[5];}else filter=args[4];}
 if(filter==='COLOR'){manifest.push({case:active,excluded:'Color belongs to 0.4'});return;}
 if(name.startsWith('decodeGltfBuffer')){op=7;mode={ATTRIBUTES:0,TRIANGLES:1,INDICES:2}[mode];}
 else {op={decodeVertexBuffer:1,decodeIndexBuffer:2,decodeIndexSequence:3}[name];}
 const f=filters[filter??'NONE'];
 // Upstream JS also accepts raw v1 in its glTF-named API; use the raw API
 // when collecting raw vectors instead of broadening the EXT helper.
 if(op===1 && f){op=7;mode=0;}
 const b=Buffer.alloc(44+data.length);b.write('MC02');
 [op,count,stride,mode,f,0,2,0,1,data.length].forEach((v,i)=>b.writeUInt32LE(v,4+i*4));Buffer.from(data).copy(b,44);
 const id='js-'+active+'-'+ordinal++;fs.writeFileSync(${JSON.stringify(destination)}+'/'+id+'.input',b);manifest.push({case:id,operation:name});
}
const decoder=new Proxy(actual,{get(target,name){
 const value=target[name];if(typeof value!=='function')return value;
 return (...args)=>{if(name.startsWith('decode'))capture(name,args);const result=value.apply(target,args);if(result?.then)pending.push(result);return result;};
}});
`;
source=source.replace('var tests = {',prelude+'\nvar tests = {');
source=source.replace('decoder.ready.then(() => {','decoder.ready.then(async () => {');
source=source.replace('tests[key]();','active=key; await tests[key]();');
source=source.replace("console.log(count, 'tests passed');",`await Promise.all(pending);fs.writeFileSync(${JSON.stringify(destination)}+'/js-manifest.json',JSON.stringify(manifest,null,2));console.log(count,'awaited tests passed');`);
const exported=destination+'/js-export.mjs';fs.writeFileSync(exported,source);await import(pathToFileURL(exported).href);
