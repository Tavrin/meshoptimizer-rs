"""Analytic importer check, separate from the real-asset measurement corpus."""
import importlib.util
import json
import pathlib
import struct
import tempfile
import unittest
import numpy as np

spec = importlib.util.spec_from_file_location('eval_models', pathlib.Path(__file__).with_name('eval_models.py'))
module = importlib.util.module_from_spec(spec)
spec.loader.exec_module(module)


class ImportTests(unittest.TestCase):
    def test_primitive_indices_promote_before_global_offset(self):
        local=np.array([0, 1, 65535],dtype=np.uint16)
        self.assertEqual(module.offset_indices(local,65536,65536),[65536,65537,131071])
        with self.assertRaises(ValueError):module.offset_indices(local,0,3)
        with self.assertRaises(OverflowError):module.offset_indices(local,0xffffffff,65536)

    def test_world_space_skin_morph_and_all_primitives(self):
        blob = bytearray(); accessors = []; views = []
        def accessor(values, kind, component=5126):
            while len(blob) % 4: blob.append(0)
            raw = np.asarray(values, dtype={5126:'<f4',5123:'<u2'}[component]).tobytes()
            views.append({'buffer':0, 'byteOffset':len(blob), 'byteLength':len(raw)})
            blob.extend(raw)
            accessors.append({'bufferView':len(views)-1,'componentType':component,'count':len(values),'type':kind})
            return len(accessors)-1
        pos=accessor([[0,0,0],[1,0,0],[0,1,0]],'VEC3')
        joints=accessor([[0,0,0,0]]*3,'VEC4',5123)
        weights=accessor([[1,0,0,0]]*3,'VEC4')
        delta=accessor([[0,0,1]]*3,'VEC3')
        times=accessor([0,1],'SCALAR')
        late_times=accessor([1,2],'SCALAR')
        scales=accessor([[1,1,1],[1,1,1]],'VEC3')
        translation=accessor([[5,0,0],[5,3,0]],'VEC3')
        ib=np.eye(4);ib[0,3]=-3
        inverse_bind=accessor([ib.T.ravel()], 'MAT4')
        normal=accessor([[0,0,1]]*3,'VEC3')
        color=accessor([[1,0,0,1]]*3,'VEC4')
        tangent=accessor([[1,0,0,1]]*3,'VEC4')
        uv=accessor([[0,0]]*3,'VEC2')
        prim={'attributes':{'POSITION':pos,'JOINTS_0':joints,'WEIGHTS_0':weights,'NORMAL':normal,'COLOR_0':color,'TANGENT':tangent,'TEXCOORD_0':uv},'targets':[{'POSITION':delta}]}
        g={'asset':{'version':'2.0'},'buffers':[{'byteLength':len(blob)}],'bufferViews':views,'accessors':accessors,
           'nodes':[{'mesh':0,'skin':0,'translation':[2,0,0]}, {'name':'Head','translation':[5,0,0],'children':[2]}, {'name':'Face','mesh':0,'translation':[-3,0,0]}],
           'skins':[{'joints':[1],'inverseBindMatrices':inverse_bind}],'meshes':[{'primitives':[prim,prim],'extras':{'targetNames':['expression']}}],
           'scenes':[{'nodes':[0,1]}],'scene':0,
           'animations':[{'samplers':[{'input':late_times,'output':translation},{'input':times,'output':scales}],
                          'channels':[{'sampler':0,'target':{'node':1,'path':'translation'}},{'sampler':1,'target':{'node':1,'path':'scale'}}]}]}
        js=json.dumps(g).encode();js+=b' '*((-len(js))%4);blob+=b'\0'*((-len(blob))%4)
        raw=struct.pack('<III',0x46546c67,2,28+len(js)+len(blob))+struct.pack('<I4s',len(js),b'JSON')+js+struct.pack('<I4s',len(blob),b'BIN\0')+blob
        with tempfile.TemporaryDirectory() as tmp:
            p=pathlib.Path(tmp)/'fixture.glb';p.write_bytes(raw);module.prepare(p)
            meta=json.loads(p.with_suffix('.eval.json').read_text())
            self.assertEqual((meta['parts'],meta['vertices'],meta['triangles'],meta['bones'],meta['shapes'],meta['animation_poses']),(4,12,4,2,1,3))
            data=p.with_suffix('.eval.bin').read_bytes();n,ni,nb,ns,np_,na,inf,aw,kw=struct.unpack_from('<9I',data)
            self.assertEqual(meta['rigid_morph_nodes'],['Face'])
            j_offset=36+n*12+ni*4
            js=np.frombuffer(data,dtype='<u4',count=n*inf,offset=j_offset).reshape(n,inf)
            ws=np.frombuffer(data,dtype='<f4',count=n*inf,offset=j_offset+n*inf*4).reshape(n,inf)
            np.testing.assert_array_equal(js[:,0],[0]*6+[1]*6)
            np.testing.assert_array_equal(ws[:,0],np.ones(n))
            self.assertEqual(aw,13)
            attr_offset=36+n*12+ni*4+n*inf*8+ns*n*12
            attrs=np.frombuffer(data,dtype='<f4',count=n*aw,offset=attr_offset).reshape(n,aw)
            np.testing.assert_array_equal(attrs,np.tile([1,0,0,1,0,0,1,1,0,0,1,0,0],(n,1)))
            offset=attr_offset+n*aw*4+n*kw+nb*4
            poses=np.frombuffer(data,dtype='<f4',offset=offset).reshape(np_,n,3)
            expected=np.array([[2,0,0],[3,0,0],[2,1,0]]*4)
            np.testing.assert_array_equal(poses[0],expected)
            # Before a sampler's first key, glTF holds that first value.
            np.testing.assert_array_equal(poses[1],expected)
            np.testing.assert_array_equal(poses[2],expected+[0,3,0])
            np.testing.assert_array_equal(poses[3],expected+[0,0,1])


if __name__ == '__main__': unittest.main()
