import urllib.request,json,struct,pathlib,hashlib,sys
out=pathlib.Path(sys.argv[1]); out.mkdir(parents=True,exist_ok=True)
base='https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Assets/main/Models/'
for name in ['Fox','CesiumMan','MorphPrimitivesTest']:
    raw=urllib.request.urlopen(base+name+'/glTF-Binary/'+name+'.glb').read();(out/(name+'.glb')).write_bytes(raw)
    jl=struct.unpack_from('<I',raw,12)[0]; g=json.loads(raw[20:20+jl]); blob=raw[28+jl:]
    def acc(i):
        a=g['accessors'][i];v=g['bufferViews'][a['bufferView']];c={'SCALAR':1,'VEC2':2,'VEC3':3,'VEC4':4,'MAT4':16}[a['type']];t={5126:'f',5123:'H',5125:'I',5121:'B'}[a['componentType']];fmt='<'+t*c;stride=v.get('byteStride',struct.calcsize(fmt));off=v.get('byteOffset',0)+a.get('byteOffset',0)
        rows=[struct.unpack_from(fmt,blob,off+j*stride) for j in range(a['count'])]
        if a.get('normalized') and t!='f': rows=[tuple(x/(65535 if t=='H' else 255) for x in r) for r in rows]
        return rows
    mesh=max((p for m in g['meshes'] for p in m['primitives']),key=lambda p:g['accessors'][p.get('indices',p['attributes']['POSITION'])]['count'])
    attrs=mesh['attributes'];pos=acc(attrs['POSITION']);idx=acc(mesh['indices']) if 'indices' in mesh else [(i,) for i in range(len(pos))];n=len(pos)
    joints=acc(attrs['JOINTS_0']) if 'JOINTS_0'in attrs else [(0,)*4]*n
    weights=acc(attrs['WEIGHTS_0']) if 'WEIGHTS_0'in attrs else [(0.,)*4]*n
    bones=len(g['skins'][0]['joints']) if 'skins'in g else 0
    shapes=[acc(t['POSITION']) for t in mesh.get('targets',[]) if 'POSITION'in t]
    poses=[]
    if bones:
        # Deliberately synthetic model-space joint translations, bounded to 10% extent.
        extent=max(max(p[k] for p in pos)-min(p[k] for p in pos) for k in range(3))
        poses=[[[[1.,0.,0.,0.],[0.,1.,0.,extent*.1*((b%3)-1)],[0.,0.,1.,0.]] for b in range(bones)]]
    b=bytearray(struct.pack('<5I',n,len(idx),bones,len(shapes),len(poses)))
    def pack(fmt,rows):
        for r in rows:b.extend(struct.pack('<'+fmt,*r))
    pack('3f',pos);pack('I',idx);pack('4I',joints);pack('4f',weights)
    for s in shapes:pack('3f',s)
    for pose in poses:
        for matrix in pose:pack('4f',matrix)
    (out/(name+'.bin')).write_bytes(b)
    print(name,n,len(idx)//3,bones,len(shapes),hashlib.sha256(raw).hexdigest())
