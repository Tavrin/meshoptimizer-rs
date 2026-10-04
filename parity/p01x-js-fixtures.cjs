// Capture applicable unchanged upstream JS test bodies after module readiness.
const fs = require('fs');
const path = require('path');
const assert = require('assert/strict');
(async () => {
  const [ref, dest] = process.argv.slice(2);
  const {MeshoptSimplifier: simplifier} = await import(path.join(ref, 'js/meshopt_simplifier.js'));
  const {MeshoptEncoder: encoder} = await import(path.join(ref, 'js/meshopt_encoder.js'));
  const {MeshoptDecoder: decoder} = await import(path.join(ref, 'js/meshopt_decoder.mjs'));
  await Promise.all([simplifier.ready, encoder.ready, decoder.ready]);
  let serial = 0;
  const inventory = [];
  const float = x => {const b = Buffer.alloc(4); b.writeFloatLE(x); return b.readUInt32LE();};
  const optionBits = options => (options || []).reduce((n, name) => n | ({LockBorder:1,Sparse:2,ErrorAbsolute:4,Prune:8,Regularize:16,Permissive:32,RegularizeLight:64,PreserveFolds:128,ErrorClamped:256}[name] ?? (() => {throw new Error('unknown option '+name);})()), 0);
  function capture(name, op, ib, p, ps, {a=[], as=0, weights=[], flags=null, target=0, error=0, options=[], remap=null, colors=false}={}) {
    const vc = p ? p.length/ps : remap ? remap.length : Math.max(-1,...ib)+1;
    const ac = weights.length, size = remap ? 4 : 1;
    const words = [op,vc,ib.length,0,0,size,size,remap?1:0,optionBits(options),target,float(error),colors?6:remap?2:0,ac];
    for(let i=0;i<vc;i++) for(let k=0;k<3;k++) words.push(p?float(p[i*ps+k]):0);
    words.push(...ib);
    if(remap) words.push(...remap);
    words.push(...weights.map(float));
    for(let i=0;i<vc;i++) for(let k=0;k<ac;k++) words.push(float(a[i*as+k]));
    for(let i=0;i<vc;i++) words.push(flags?flags[i]:0);
    const b=Buffer.alloc(4+words.length*4);b.write('MO02');words.forEach((x,i)=>b.writeUInt32LE(x>>>0,4+i*4));
    const id=`p01x-js-${name}-${serial++}`;
    fs.writeFileSync(path.join(dest,id+'.input'),b);inventory.push({id,op});
  }
  const capturedSimplifier={...simplifier,
    compactMesh(ib) {
      const original = new Uint32Array(ib);
      capture('compactMesh-fetch-remap',23,original,null,0);
      const result=simplifier.compactMesh(ib);
      capture('compactMesh-index-remap',13,original,null,0,{remap:result[0]});
      return result;
    },
    simplifyWithUpdate(ib,p,ps,a,as,w,flags,target,error,options=[]) {
      capture('simplifyWithUpdate',27,ib,p,ps,{a,as,weights:w,flags,target,error,options});
      return simplifier.simplifyWithUpdate(ib,p,ps,a,as,w,flags,target,error,options);
    },
    simplifyPoints(p,ps,target,c,cs,weight) {
      capture('simplifyPoints',26,[],p,ps,{target,error:weight||0,a:c||[],as:cs||0,weights:c?[0,0,0]:[],colors:!!c});
      return simplifier.simplifyPoints(p,ps,target,c,cs,weight);
    },
    simplifyPrune(ib,p,ps,error) {
      capture('simplifyPrune',25,ib,p,ps,{error});
      return simplifier.simplifyPrune(ib,p,ps,error);
    }
  };
  const capturedEncoder={...encoder,
    reorderMesh(ib,triangles,optsize) {
      const original=new Uint32Array(ib);
      assert(triangles && optsize, 'fixture must use strip optimization');
      capture('reorderMesh-strip',7,original,null,0);
      const result=encoder.reorderMesh(ib,triangles,optsize);
      const inverse=[];result[0].forEach((value,index)=>{if(value!==0xffffffff)inverse[value]=index;});
      const reordered=Array.from(ib,value=>inverse[value]);
      capture('reorderMesh-fetch-remap',23,reordered,null,0);
      capture('reorderMesh-index-remap',13,reordered,null,0,{remap:result[0]});
      return result;
    }
  };
  for(const [file,variable,adapter] of [['meshopt_simplifier.test.js','simplifier',capturedSimplifier],['meshopt_encoder.test.js','encoder',capturedEncoder]]) {
    const source=fs.readFileSync(path.join(ref,'js',file),'utf8').replace(/^import .*;\r?\n/gm,'').replace(/\nPromise\.all\(/,'\nreturn Promise.all(');
    await new Function('assert',variable,'decoder',source)(assert,adapter,decoder);
  }
  assert.equal(inventory.length,10,'missing applicable JS fixture');
  fs.writeFileSync(path.join(dest,'p01x-js-inventory.json'),JSON.stringify(inventory,null,2)+'\n');
})().catch(error=>{console.error(error);process.exitCode=1;});
