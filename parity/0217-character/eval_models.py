"""Round-two GLB importer. Only reads assets; never executes asset content.

Usage: eval_models.py external-model-directory
Requires numpy. Preserves all vertex attributes in exact weld keys. Samples
at most twenty authored key times per clip, interpolating other channels there.
The packed format consumed by character_eval.rs contains already evaluated
world-space poses, so no animation evaluator is needed in Rust.
"""
import hashlib
import json
import pathlib
import struct
import sys
import numpy as np


def matrix(node):
    if 'matrix' in node:
        return np.array(node['matrix']).reshape(4, 4).T
    x, y, z, w = node.get('rotation', [0, 0, 0, 1])
    r = np.array([[1-2*(y*y+z*z), 2*(x*y-z*w), 2*(x*z+y*w)],
                  [2*(x*y+z*w), 1-2*(x*x+z*z), 2*(y*z-x*w)],
                  [2*(x*z-y*w), 2*(y*z+x*w), 1-2*(x*x+y*y)]])
    m = np.eye(4)
    m[:3, :3] = r @ np.diag(node.get('scale', [1, 1, 1]))
    m[:3, 3] = node.get('translation', [0, 0, 0])
    return m


def load(path):
    raw = path.read_bytes()
    jl = struct.unpack_from('<I', raw, 12)[0]
    g = json.loads(raw[20:20+jl]); blob = raw[28+jl:]
    assert not any('EXT_meshopt_compression' in v.get('extensions',{}) for v in g.get('bufferViews',[])), 'compressed views require decoding'
    def acc(i):
        a = g['accessors'][i]
        c = {'SCALAR':1, 'VEC2':2, 'VEC3':3, 'VEC4':4, 'MAT4':16}[a['type']]
        dt = np.dtype({5120:'i1',5121:'u1',5122:'<i2',5123:'<u2',5125:'<u4',5126:'<f4'}[a['componentType']])
        def read(view, off, count, width, dtype):
            v = g['bufferViews'][view]
            assert v.get('buffer', 0) == 0
            return np.ndarray((count, width), dtype=dtype, buffer=blob,
                              offset=v.get('byteOffset',0)+off,
                              strides=(v.get('byteStride',width*dtype.itemsize),dtype.itemsize)).copy()
        rows = (read(a['bufferView'],a.get('byteOffset',0),a['count'],c,dt)
                if 'bufferView' in a else np.zeros((a['count'],c),dtype=dt))
        if 'sparse' in a:
            s=a['sparse']; si=s['indices']; sv=s['values']
            it=np.dtype({5121:'u1',5123:'<u2',5125:'<u4'}[si['componentType']])
            ids=read(si['bufferView'],si.get('byteOffset',0),s['count'],1,it).ravel()
            rows[ids]=read(sv['bufferView'],sv.get('byteOffset',0),s['count'],c,dt)
        if a.get('normalized') and dt.kind != 'f':
            rows=np.maximum(rows.astype(float)/np.iinfo(dt).max,-1)
        return rows
    return g, acc, hashlib.sha256(raw).hexdigest()


def offset_indices(local, offset, count):
    """Promote primitive indices before adding a global vertex offset."""
    if offset < 0 or count < 0 or offset + count > 0xffffffff:
        raise OverflowError('merged vertex count exceeds packed u32 range')
    values=[int(index) for index in local]
    if any(index < 0 or index >= count for index in values):
        raise ValueError('primitive index outside its vertex stream')
    return [index + offset for index in values]


def prepare(path):
    g, acc, sha = load(path); nodes = g['nodes']
    parents={c:i for i,n in enumerate(nodes) for c in n.get('children',[])}
    active=set()
    def visit(i):
        active.add(i)
        for c in nodes[i].get('children',[]): visit(c)
    for i in g['scenes'][g.get('scene',0)]['nodes']:visit(i)
    def worlds(overrides=None):
        cache={}; overrides=overrides or {}
        def world(i):
            if i not in cache:
                n=dict(nodes[i]);n.update(overrides.get(i,{}))
                cache[i]=(world(parents[i]) if i in parents else np.eye(4)) @ matrix(n)
            return cache[i]
        return [world(i) for i in range(len(nodes))]
    has_skin=any('skin' in nodes[i] and 'mesh' in nodes[i] for i in active)
    rest=worlds(); parts=[]; bone_names=[]; skin_offsets={}; bone_ids={}; shape_offsets={}; shape_ids={}; shape_count=0
    for i in sorted(active):
        node=nodes[i]
        if 'mesh' not in node:continue
        mesh=g['meshes'][node['mesh']]
        # Include morph-only test meshes, but do not add unrelated scene props.
        if 'skin' not in node and not any(p.get('targets') for p in mesh['primitives']):continue
        if 'skin' in node:
            s=g['skins'][node['skin']]
            for b in s['joints']:
                if b not in bone_ids:
                    bone_ids[b]=len(bone_names);bone_names.append(nodes[b].get('name',''))
            skin_offsets[i]=np.array([bone_ids[b] for b in s['joints']])
        elif has_skin and i not in bone_ids:
            bone_ids[i]=len(bone_names);bone_names.append(node.get('name',str(i)))
        shape_offsets[i]=[]
        for k in range(max(len(p.get('targets',[])) for p in mesh['primitives'])):
            names=mesh.get('extras',{}).get('targetNames',[])
            identity=('named',names[k]) if k<len(names) else (i,k)
            if identity not in shape_ids:
                shape_ids[identity]=shape_count;shape_count+=1
            shape_offsets[i].append(shape_ids[identity])
        for prim in mesh['primitives']:
            assert prim.get('mode',4)==4, 'non-triangle primitive'
            assert 'KHR_draco_mesh_compression' not in prim.get('extensions',{}), 'Draco requires decoding'
            assert not any(k.startswith('JOINTS_') and k not in ['JOINTS_0','JOINTS_1'] for k in prim['attributes']), 'more than eight influences'
            attrs={k:acc(v) for k,v in prim['attributes'].items()}
            p=attrs['POSITION']; count=len(p)
            ix=acc(prim['indices']).ravel() if 'indices'in prim else np.arange(count)
            parts.append((i,prim,attrs,ix))
    assert parts, 'no deformable meshes'
    influences=8 if any('JOINTS_1'in a for _,_,a,_ in parts) else 4
    # Every original semantic participates in the weld; material boundaries too.
    semantics=sorted({k for _,_,a,_ in parts for k in a})
    widths={k:next(a[k].shape[1] for _,_,a,_ in parts if k in a) for k in semantics}
    positions=[]; joints=[]; weights=[]; attributes=[]; keys=[]; deltas=[]; indices=[]; offset=0
    def transform(p,m,delta=False):return p @ m[:3,:3].T + (0 if delta else m[:3,3])
    for i,prim,a,ix in parts:
        p=transform(a['POSITION'],rest[i]); n=len(p);positions.append(p)
        j=np.zeros((n,influences),dtype=np.uint32); w=np.zeros((n,influences))
        for k in range(influences//4):
            if 'skin' in nodes[i] and f'JOINTS_{k}' in a:
                j[:,k*4:k*4+4]=skin_offsets[i][a[f'JOINTS_{k}'].astype(int)]
                w[:,k*4:k*4+4]=a[f'WEIGHTS_{k}']
        if 'skin'in nodes[i]: w/=w.sum(axis=1)[:,None]
        elif has_skin:
            j[:,0]=bone_ids[i];w[:,0]=1.
        joints.append(j);weights.append(w)
        ds=np.zeros((n,shape_count,3))
        for k,t in enumerate(prim.get('targets',[])):
            if 'POSITION'in t:ds[:,shape_offsets[i][k]]=transform(acc(t['POSITION']),rest[i],True)
        deltas.append(ds)
        # All non-skin vertex attributes are supplied to plain.
        ap=[]
        for k in semantics:
            if k=='NORMAL':
                v=(a[k] @ np.linalg.inv(rest[i][:3,:3])) if k in a else np.zeros((n,widths[k]))
                v/=np.maximum(np.linalg.norm(v,axis=1)[:,None],1e-30);ap.append(v)
            elif k=='TANGENT':
                v=a.get(k,np.zeros((n,4))).copy()
                v[:,:3]=v[:,:3] @ rest[i][:3,:3].T
                v[:,:3]/=np.maximum(np.linalg.norm(v[:,:3],axis=1)[:,None],1e-30)
                if np.linalg.det(rest[i][:3,:3])<0:v[:,3]*=-1
                ap.append(v)
            elif k!='POSITION' and not k.startswith(('JOINTS_','WEIGHTS_')):
                ap.append(a.get(k,np.zeros((n,widths[k]))))
        attributes.append(np.concatenate(ap,axis=1) if ap else np.empty((n,0)))
        key=[p,j,w,ds.reshape(n,-1),np.full((n,1),prim.get('material',-1)),attributes[-1]]
        for k in semantics:key.append(a.get(k,np.zeros((n,widths[k]))))
        # Include NORMAL/TANGENT morph data as well as position deltas.
        for k in range(shape_count):
            for semantic in ['NORMAL','TANGENT']:
                local=shape_offsets[i].index(k) if k in shape_offsets[i] else -1;ts=prim.get('targets',[])
                key.append(acc(ts[local][semantic]) if 0<=local<len(ts) and semantic in ts[local] else np.zeros((n,3)))
        keys.append(np.concatenate(key,axis=1));indices.extend(offset_indices(ix,offset,n));offset+=n
    p=np.concatenate(positions);j=np.concatenate(joints);w=np.concatenate(weights);ds=np.concatenate(deltas)
    labels=[]; samples=[]; clip_info=[]
    for ai,anim in enumerate(g.get('animations',[])):
        times=np.unique(np.concatenate([acc(s['input']).ravel() for s in anim['samplers']]))
        times=times[np.unique(np.linspace(0,len(times)-1,min(20,len(times))).round().astype(int))]
        clip_info.append({'clip':anim.get('name',str(ai)), 'times':times.tolist()})
        for time in times:
            overrides={}; morph={}
            for ch in anim['channels']:
                target=ch['target']; ni=target.get('node'); prop=target['path']
                assert prop in ['translation','rotation','scale','weights'], 'unsupported animation channel'
                if ni is None: raise ValueError('animation pointer unsupported')
                s=anim['samplers'][ch['sampler']];ts=acc(s['input']).ravel();vs=acc(s['output'])
                interpolation=s.get('interpolation','LINEAR'); mul=3 if interpolation=='CUBICSPLINE' else 1
                vs=vs.reshape(len(ts),mul,-1)
                lo=max(0,min(len(ts)-1,np.searchsorted(ts,time,side='right')-1));hi=min(lo+1,len(ts)-1)
                alpha=0 if lo==hi else np.clip((time-ts[lo])/(ts[hi]-ts[lo]),0,1)
                if interpolation=='STEP':v=vs[lo,0]
                elif interpolation=='CUBICSPLINE':
                    t=alpha;dt=ts[hi]-ts[lo]
                    v=(2*t**3-3*t*t+1)*vs[lo,1]+(t**3-2*t*t+t)*dt*vs[lo,2]+(-2*t**3+3*t*t)*vs[hi,1]+(t**3-t*t)*dt*vs[hi,0]
                else:
                    a=vs[lo,0]; b=vs[hi,0];dot=np.dot(a,b)
                    if prop=='rotation' and dot<0:b=-b;dot=-dot
                    if prop=='rotation' and dot<0.9995:
                        theta=np.arccos(np.clip(dot,-1,1));v=(np.sin((1-alpha)*theta)*a+np.sin(alpha*theta)*b)/np.sin(theta)
                    else:v=(1-alpha)*a+alpha*b
                if prop=='rotation':v=v/np.linalg.norm(v)
                if prop=='weights':morph[ni]=v
                else:overrides.setdefault(ni,{})[prop]=v.tolist()
            world=worlds(overrides);posed=[]
            for i,prim,a,_ in parts:
                local=a['POSITION'].astype(float).copy();ts=prim.get('targets',[])
                mw=morph.get(i,nodes[i].get('weights',g['meshes'][nodes[i]['mesh']].get('weights',[0]*len(ts))))
                for k,t in enumerate(ts):
                    if 'POSITION'in t: local+=float(mw[k])*acc(t['POSITION'])
                if 'skin'in nodes[i]:
                    s=g['skins'][nodes[i]['skin']];ib=acc(s['inverseBindMatrices']).reshape(-1,4,4).transpose(0,2,1) if 'inverseBindMatrices'in s else np.tile(np.eye(4),(len(s['joints']),1,1))
                    matrices=np.array([world[b] @ ib[k] for k,b in enumerate(s['joints'])]);q=np.zeros_like(local)
                    total=np.zeros(len(local))
                    for k in range(influences//4):
                        if f'JOINTS_{k}' not in a:continue
                        for z in range(4):
                            weights_=a[f'WEIGHTS_{k}'][:,z];total+=weights_
                            mat=matrices[a[f'JOINTS_{k}'][:,z].astype(int)]
                            q+=weights_[:,None]*(np.einsum('nij,nj->ni',mat[:,:3,:3],local)+mat[:,:3,3])
                    q/=total[:,None]
                else:q=transform(local,world[i])
                posed.append(q)
            samples.append(np.concatenate(posed));labels.append(f'{ai}:{anim.get("name",ai)}@{time:.9g}')
    for k in range(shape_count):samples.append(p+ds[:,k]);labels.append(f'full_shape:{k}')
    na=len(samples)-shape_count
    attr=np.concatenate(attributes); key=np.concatenate(keys).astype('<f4'); key[key==0]=0
    dest=path.with_suffix('.eval.bin')
    temporary=dest.with_suffix(dest.suffix+'.part')
    with temporary.open('wb') as f:
        f.write(struct.pack('<9I',len(p),len(indices),len(bone_names),shape_count,len(samples),na,influences,attr.shape[1],key.shape[1]*4))
        for array,dtype in [(p,'<f4'),(indices,'<u4'),(j,'<u4'),(w,'<f4'),(ds.transpose(1,0,2),'<f4'),(attr,'<f4'),(key,'<f4')]:f.write(np.asarray(array,dtype=dtype).tobytes())
        importance=[4 if any(k in name.lower() for k in ['head','hand','finger','thumb']) else 1 for name in bone_names]
        f.write(np.asarray(importance,dtype='<f4').tobytes())
        f.write(np.asarray(samples,dtype='<f4').tobytes())
    temporary.replace(dest)
    meta={'sha256':sha,'vertices':len(p),'triangles':len(indices)//3,'bones':len(bone_names),'shapes':shape_count,'shape_names':[str(k) for k in shape_ids],'clips':clip_info,'animation_poses':na,'labels':labels,'important_bones':[s for s,v in zip(bone_names,importance) if v>1],'parts':len(parts),'rigid_morph_nodes':[nodes[i].get('name',str(i)) for i in shape_offsets if has_skin and 'skin' not in nodes[i]],'attribute_width':attr.shape[1]}
    path.with_suffix('.eval.json').write_text(json.dumps(meta,indent=2));print(path.stem,json.dumps({k:v for k,v in meta.items() if k not in ['labels','clips']}),flush=True)


if __name__=='__main__':
    selection=set(sys.argv[2:])
    for path in sorted(pathlib.Path(sys.argv[1]).glob('*/*.glb')):
        if selection and path.stem not in selection:continue
        if path.stem == 'minion-b01': continue  # Retained alternate, outside the 12-character corpus.
        try:prepare(path)
        except Exception as e:print(path,repr(e),file=sys.stderr);raise
