"""Check all original function signatures and inventory additive width APIs."""
import json,re,subprocess
from pathlib import Path
from source_blocks import end_block
root=Path.cwd();base='b89cc4fe03efe346e4695594f1ca4e4688a8c8f1'
entries=json.loads(Path('parity/0217-u16/api.json').read_text())['source_declarations']
def signature(text,name):
 m=re.search(r'pub fn '+name+r'\b[^\{]*\{',text)
 assert m,name
 return re.sub(r'\s+','',re.sub(r'\bmut\s+(?=\w+\s*:(?!:))','',m[0]))
for e in entries:
 file,name=e.split('::');current=Path(file).read_text()
 original=subprocess.check_output(['git','show',base+':'+file],text=True)
 assert signature(current,name)==signature(original,name),e
 assert re.search(r'pub fn '+name+r'_generic<I: crate::Index>',current),e
original_methods=[
 ('src/remap.rs','FetchIndices','new'),
 ('src/lod_quality.rs','LodQualityMesh','new'),
 ('src/parallel.rs','EncodeInput','encode'),
 ('src/parallel.rs','MeshletInput','build'),
 ('src/codec/sequence_stream.rs','IndexSequenceDecoder','push'),
 ('src/clusterlod/dag_format.rs','Reader','decode_page'),
]
def method_signature(text,owner,name):
 bodies=[]
 for m in re.finditer(r'\bimpl(?:<[^{}]*>)?\s+'+owner+r'(?:<[^{}]*>)?\s*\{',text):
  body=text[m.end():end_block(text,m.end())-1]
  if re.search(r'pub fn '+name+r'\b',body):bodies.append(body)
 assert len(bodies)==1,(owner,name,len(bodies))
 return signature(bodies[0],name)
for file,owner,name in original_methods:
 original=subprocess.check_output(['git','show',base+':'+file],text=True)
 current=Path(file).read_text()
 new_owner=owner
 assert method_signature(original,owner,name)==method_signature(current,new_owner,name),(owner,name)
methods=[]
for p in Path('src').rglob('*.rs'):
 for m in re.finditer(r'(?m)^    pub fn (\w+_generic)\b',p.read_text()):methods.append(str(p)+'::'+m[1])
assert len(methods)==6,methods
aliases=[('remap::optimize_vertex_fetch_into','optimize_vertex_fetch_in_place'),('simplify::simplify_with_update','simplify_with_update_in_place')]
original_lib=subprocess.check_output(['git','show',base+':src/lib.rs'],text=True)
current_lib=Path('src/lib.rs').read_text()
for target,name in aliases:
 old='pub use '+target+' as '+name+';'
 new='pub use '+target+'_generic as '+name+'_generic;'
 assert old in original_lib and old in current_lib and new in current_lib,name
result={'baseline':base,'exact_original_function_signatures':len(entries),'generic_free_functions':len(entries),'generic_aliases':len(aliases),'generic_methods':methods,'entrypoints':len(entries)+len(aliases)+len(methods)}
result['exact_original_method_signatures']=len(original_methods)
result['generic_types'] = [m[1] for p in Path('src').rglob('*.rs') for m in re.finditer(r'pub (?:struct|enum) (\w+Generic)<', p.read_text())]
assert len(result['generic_types']) == 24
Path('parity/0217-u16c/api.json').write_text(json.dumps(result,indent=2)+'\n')
print(result)
