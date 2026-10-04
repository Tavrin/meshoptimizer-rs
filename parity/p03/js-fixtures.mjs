// Capture all applicable clusterizer calls and await the upstream suite.
import fs from 'node:fs';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
const root=process.argv[2],directory=process.argv[3];
const {MeshoptClusterizer:c}=await import(pathToFileURL(path.join(root,'js/meshopt_clusterizer.js')));
await c.ready;
let id=0;
function write(op,p,stride,indices,options={}){
 const nv=p.length/stride,idx=Array.from(indices||[]),v=[];for(let i=0;i<nv;i++)v.push(...p.subarray(i*stride,i*stride+3));
 const b=Buffer.alloc(48+nv*16+idx.length*4);b.write('MO03');const header=[op,nv,idx.length,options.mv||64,16,options.mt||64];header.forEach((n,i)=>b.writeUInt32LE(n,4+i*4));b.writeFloatLE(options.weight||0,28);b.writeFloatLE(0,32);b.writeUInt32LE(options.radii?1:(op===8?0:4),36);b.writeUInt32LE(0,40);b.writeUInt32LE(0,44);let at=48;for(const f of v){b.writeFloatLE(f,at);at+=4;}for(const x of idx){b.writeUInt32LE(x,at);at+=4;}for(let i=0;i<nv;i++){b.writeFloatLE(options.radii?options.radii[i*(options.rstride||1)]:0,at);at+=4;}
 fs.writeFileSync(path.join(directory,`js-clusterizer-${op}-${id++}.input`),b);
}
const original={};for(const n of ['buildMeshlets','computeClusterBounds','computeMeshletBounds','computeSphereBounds'])original[n]=c[n];
c.buildMeshlets=(idx,p,stride,mv,mt,w)=>{write(1,p,stride,idx,{mv,mt,weight:w});return original.buildMeshlets(idx,p,stride,mv,mt,w);};
c.computeClusterBounds=(idx,p,stride)=>{write(6,p,stride,idx);return original.computeClusterBounds(idx,p,stride);};
c.computeMeshletBounds=(buffers,p,stride)=>{for(let i=0;i<buffers.meshletCount;i++){const m=c.extractMeshlet(buffers,i);write(7,p,stride,Array.from(m.triangles,t=>m.vertices[t]));}return original.computeMeshletBounds(buffers,p,stride);};
c.computeSphereBounds=(p,stride,radii,rstride)=>{write(8,p,stride,[],{radii,rstride});return original.computeSphereBounds(p,stride,radii,rstride);};
await import(pathToFileURL(path.join(root,'js/meshopt_clusterizer.test.js')));
await new Promise(resolve=>setImmediate(resolve));
if(id!==16)throw new Error(`expected 16 captured calls, got ${id}`);
console.log(`captured ${id} applicable clusterizer calls`);
