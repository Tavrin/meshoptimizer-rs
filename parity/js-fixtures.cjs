// Execute the unchanged applicable JS test bodies with a capturing adapter.
const fs = require('fs');
const path = require('path');
const assert = require('assert/strict');
(async () => {
  const [ref, dest] = process.argv.slice(2);
  const {MeshoptSimplifier: original} = await import(path.join(ref, 'js/meshopt_simplifier.js'));
  await original.ready;
  let serial = 0;
  function capture(name, op, indices, p, ps, a=[], as=0, weights=[], flags=null, target=0, error=0, options=[]) {
    const vc=p.length/ps, ac=weights.length, words=[];
    const f = x => {const b=Buffer.alloc(4);b.writeFloatLE(x);return b.readUInt32LE();};
    words.push(op,vc,indices.length,f(1.05),0,0);
    for(let i=0;i<vc;++i) for(let k=0;k<3;++k) words.push(f(p[i*ps+k]));
    words.push(...indices,target,f(error),options.includes('LockBorder')?1:0,ac);
    words.push(...weights.map(f));
    for(let i=0;i<vc;++i) for(let k=0;k<ac;++k) words.push(f(a[i*as+k]));
    for(let i=0;i<vc;++i) words.push(flags?flags[i]:0);
    const b=Buffer.alloc(4+words.length*4);b.write('MO01');words.forEach((x,i)=>b.writeUInt32LE(x,4+i*4));
    fs.writeFileSync(path.join(dest,`js-${name}-${serial++}.input`),b);
  }
  const adapter = {...original,
    simplify(ib,p,ps,target,error,options=[]) {capture(ib instanceof Uint16Array?'simplify16':'simplify',4,ib,p,ps,[],0,[],null,target,error,options);return original.simplify(ib,p,ps,target,error,options);},
    simplifyWithAttributes(ib,p,ps,a,as,w,flags,target,error,options=[]) {capture('simplifyWithAttributes',5,ib,p,ps,a,as,w,flags,target,error,options);return original.simplifyWithAttributes(ib,p,ps,a,as,w,flags,target,error,options);},
    getScale(p,ps) {capture('getScale',6,[],p,ps);return original.getScale(p,ps);}
  };
  const source=fs.readFileSync(path.join(ref,'js/meshopt_simplifier.test.js'),'utf8').replace(/^import .*;\r?\n/gm,'');
  new Function('assert','simplifier',source)(assert,adapter);
})().catch(e=>{console.error(e);process.exitCode=1;});
