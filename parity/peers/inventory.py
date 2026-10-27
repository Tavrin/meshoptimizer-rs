"""Extract pinned upstream public surfaces and bind reviewed port equivalents."""
import hashlib,json,os,re
from pathlib import Path
root=Path(__file__).resolve().parents[2]; art=Path(os.environ['PEERS_ART'])
cpp=Path(os.environ['PEERS_CPP']); peer=Path(os.environ['PEERS_RUST'])
def snake(s):return re.sub(r'(?<=[a-z0-9])(?=[A-Z])','_',s).lower()
def locs(base):
    result={}
    for p in sorted((base/'src').rglob('*.rs')):
        for no,line in enumerate(p.read_text().splitlines(),1):
            for m in re.finditer(r'\bpub (?:const |async )?(?:fn|const|enum|struct|trait)\s+([A-Za-z0-9_]+)',line):
                result.setdefault(m[1],[]).append(f'{p.relative_to(base)}:{no}')
    return result
ours=locs(root); theirs=locs(peer)
def at(name):return ', '.join(ours.get(name,[]))
rows=[]
header=cpp/'vendor/src/meshoptimizer.h'
for no,line in enumerate(header.read_text().splitlines(),1):
    if no>1100:break
    m=re.search(r'^(?:MESHOPTIMIZER_(?:API|EXPERIMENTAL).*?|inline (?:int|float) )\b(meshopt_[A-Za-z0-9_]+)\(',line)
    if not m:continue
    name=m[1];other=snake(name.removeprefix('meshopt_'));target=other;status='present';note='';size='-';user='-';does='-'
    if name=='meshopt_encodeVertexVersion':target='VertexEncoding';note='Explicit per-call version replaces the global setter.'
    if name=='meshopt_encodeIndexVersion':target='IndexEncoding';note='Explicit per-call version replaces the global setter.'
    if name=='meshopt_encodeVertexBufferLevel':target='VertexEncoding';note='Level and version configured per call; encode_vertex_buffer executes it.'
    if name=='meshopt_setAllocator':
        target='Workspace';status='missing';size='L';does='Install library-specific allocate/free callbacks for transient storage.';user='Hosts with per-library arenas, allocation telemetry or allocator isolation.';note='Workspace controls limits/retention, not allocator selection; Rust global allocator remains process-wide.'
    if name=='meshopt_generateNormals':note='Requires experimental feature.'
    if name=='meshopt_remesh':note='Requires experimental feature.'
    assert target in ours,(name,target)
    op=theirs.get(other,[])
    if name.startswith('meshopt_encode') and name.endswith('Version'):op=theirs.get('set_'+other,[])
    rows.append(dict(surface='C++ 1.3',symbol=name,source=f'vendor/src/meshoptimizer.h:{no}',status=status,port=target,port_source=at(target),optimesh='; '.join(op) or 'absent',note=note,does=does,users=user,size=size))
# The demo is public in meshopt 0.7.0, but is not part of the core C API.
clod={'clodDefaultConfig':'default_config','clodDefaultConfigRT':'default_config_rt','clodBuild':'build_with_output','clodLocalIndices':'extract_meshlet_indices','clodBuildHierarchyBound':'build_hierarchy_bound','clodBuildHierarchy':'build_hierarchy'}
for no,line in enumerate((cpp/'vendor/demo/clusterlod.h').read_text().splitlines(),1):
    m=re.search(r'^(?:clodConfig|void|size_t) (clod\w+)\(',line)
    if m:
        name=m[1];target=clod[name];rows.append(dict(surface='C++ cluster LOD demo',symbol=name,source=f'vendor/demo/clusterlod.h:{no}',status='present',port=target,port_source=at(target),optimesh='absent',note='clusterlod feature; equivalent explicit Rust API.',does='-',users='-',size='-'))
        if len([r for r in rows if r['surface']=='C++ cluster LOD demo'])==6:break
# Every public optimesh free function and inherent method, including typed filter variants.
for p in sorted((peer/'src').glob('*.rs')):
    if p.name in ['lib.rs','simd.rs']:continue
    for no,line in enumerate(p.read_text().splitlines(),1):
        m=re.search(r'\bpub (?:const )?fn (\w+)',line)
        if not m:continue
        if p.name in ['lib.rs','simd.rs']:continue # cfg(test) oracle mutex helper, not a library API
        name=m[1];target=name;note='';status='present';does='-';users='-';size='-'
        if name.startswith('set_encode_') and name.endswith('_version'):
            target='VertexEncoding' if 'vertex' in name else 'IndexEncoding';note='Explicit per-call configuration replaces the global setter.'
        if name=='encode_vertex_buffer_level':target='VertexEncoding';note='Pass VertexEncoding to encode_vertex_buffer.'
        if name in ['decode_filter_oct8','decode_filter_oct16','decode_filter_color8','decode_filter_color16','encode_filter_oct8','encode_filter_oct16','encode_filter_color8','encode_filter_color16']:
            target=re.sub(r'(8|16)$','',name);note='Single byte-buffer API selects the 4/8-byte representation by stride.'
        if name=='optimize_vertex_cache_table':
            target='optimize_vertex_cache';status='missing';does='Use caller-supplied cache/valence score tables for triangle ordering.';users='Engine or hardware tuning tools and research optimizers.';size='S';note='Only the standard and strip score presets are exposed.'
        if target not in ours:raise AssertionError((p.name,no,name,target))
        rows.append(dict(surface='optimesh 1.1.0',symbol=f'{p.stem}::{name}',source=f'src/{p.name}:{no}',status=status,port=target,port_source=at(target),optimesh=f'src/{p.name}:{no}',note=note,does=does,users=users,size=size))
# C++ templates add arbitrary index-width adapters on top of the C entry points.
rows.append(dict(surface='C++ 1.3 C++ templates',symbol='meshopt_IndexAdapter<T> and templated index overloads',source='vendor/src/meshoptimizer.h:1072',status='partial',port='u32 processing / byte-oriented codecs',port_source=at('simplify')+'; '+at('decode_index_buffer'),optimesh='indexcodec::IndexWrite and meshletcodec::{MeshletVertex,MeshletTriangle}; processing uses u32',note='Processing accepts u32 slices. Codecs expose 16/32-bit destinations as bytes; no general typed processing adapter.',does='Convert other integer index types around the u32 algorithm.',users='Importers retaining 16-bit topology and generic mesh containers.',size='M'))
# Named flags and enum options.
options=[]
flag_alias={'SimplifyLockBorder':'LOCK_BORDER','SimplifySparse':'SPARSE','SimplifyErrorAbsolute':'ERROR_ABSOLUTE','SimplifyPrune':'PRUNE','SimplifyRegularize':'REGULARIZE','SimplifyPermissive':'PERMISSIVE','SimplifyRegularizeLight':'REGULARIZE_LIGHT','SimplifyPreserveFolds':'PRESERVE_FOLDS','SimplifyErrorClamped':'ERROR_CLAMPED','SimplifyVertex_Lock':'LOCK','SimplifyVertex_Protect':'PROTECT','SimplifyVertex_Priority':'PRIORITY','TangentCompatible':'TANGENT_COMPATIBLE','TangentZeroFallback':'TANGENT_ZERO_FALLBACK','RemeshShell':'REMESH_SHELL','RemeshSolve':'REMESH_SOLVE','EncodeExpSeparate':'Separate','EncodeExpSharedVector':'SharedVector','EncodeExpSharedComponent':'SharedComponent','EncodeExpClamped':'Clamped'}
for no,line in enumerate(header.read_text().splitlines(),1):
    if no>1100:break
    m=re.search(r'^\s*meshopt_(\w+)\s*(?:=|,)',line)
    if not m:continue
    name=m[1];target=flag_alias[name];refs=ours.get(target,[])
    if not refs:
        refs=[f'src/codec/encode.rs:{i}' for i,s in enumerate((root/'src/codec/encode.rs').read_text().splitlines(),1) if re.search(r'\b'+target+r'\b',s)]
    assert refs,(name,target)
    op=snake(name).upper().replace('SIMPLIFY_VERTEX_','SIMPLIFY_VERTEX_')
    available=op in theirs
    if name.startswith('EncodeExp'):available=True
    options.append(dict(surface='C++ 1.3',symbol='meshopt_'+name,source=f'vendor/src/meshoptimizer.h:{no}',status='present',port=target,port_source=', '.join(refs),optimesh='present' if available else 'absent',note='experimental feature' if name in ['SimplifyPreserveFolds','SimplifyErrorClamped','RemeshShell','RemeshSolve'] else ''))
# Every clodConfig field is a public option.
in_config=False
for no,line in enumerate((cpp/'vendor/demo/clusterlod.h').read_text().splitlines(),1):
    if line.startswith('struct clodConfig'):in_config=True
    if in_config and line=='};':break
    m=re.search(r'^\s*(?:size_t|bool|float|int) (\w+);',line)
    if in_config and m:
        name=m[1];ref=[f'src/clusterlod/mod.rs:{i}' for i,s in enumerate((root/'src/clusterlod/mod.rs').read_text().splitlines(),1) if re.search(r'pub '+name+r':',s)]
        assert len(ref)==1,(name,ref)
        options.append(dict(surface='C++ cluster LOD demo',symbol='clodConfig::'+name,source=f'vendor/demo/clusterlod.h:{no}',status='present',port='Config::'+name,port_source=ref[0],optimesh='absent',note='clusterlod feature'))
# optimesh public constants and configuration fields (not output-only structures).
field_map={'MeshletOptions':'MeshletSettings / build_meshlets_flex','Positions':'Positions','VertexData':'Positions','VertexDataMut':'PositionsMut','Attributes':'Attributes','AttributesMut':'AttributesMut','SimplifyTarget':'SimplifySettings','Stream':'VertexStream'}
for p in sorted((peer/'src').glob('*.rs')):
    if p.name in ['lib.rs','simd.rs']:continue
    lines=p.read_text().splitlines();struct_name=None
    for no,line in enumerate(lines,1):
        m=re.search(r'pub struct (\w+)',line)
        if m:struct_name=m[1]
        m=re.search(r'^pub const (\w+)',line)
        if m:
            name=m[1];found=[o for o in options if (snake(o['symbol'].removeprefix('meshopt_')).upper()==name)]
            if found:
                port=found[0]['port'];refs=found[0]['port_source'];status='present';note='Same flag value.'
            else:
                assert name in ['VERTEX_SCORE_TABLE','VERTEX_SCORE_TABLE_STRIP'],name
                port='optimize_vertex_cache'+('_strip' if name.endswith('STRIP') else '');refs=at(port);status='partial';note='Preset behavior present; public data table and caller tuning absent (S).'
            options.append(dict(surface='optimesh 1.1.0',symbol=p.stem+'::'+name,source=f'src/{p.name}:{no}',status=status,port=port,port_source=refs,optimesh='present',note=note))
        m=re.search(r'pub (\w+):',line)
        if m and struct_name in field_map:
            name=m[1];target=field_map[struct_name];lookup=target.split(' / ')[0]
            options.append(dict(surface='optimesh 1.1.0',symbol=f'{struct_name}::{name}',source=f'src/{p.name}:{no}',status='present',port=target,port_source=at(lookup),optimesh='present',note='Byte strides in optimesh; float-element strides in Attributes. count/stride are checked Rust views.'))
# Additional runtime options not represented by named enums.
controls=[('vertex format versions 0/1, encoder levels 0..9','VertexEncoding','src/codec/encode.rs:24'),('index versions 0/1','IndexEncoding','src/codec/encode.rs:64'),('index decode widths 2/4 bytes','decode_index_buffer','src/codec/mod.rs:443'),('meshlet decode vertex widths 2/4 and triangle widths 3/4 bytes','decode_meshlet','src/codec/meshlet.rs:717'),('meshlet max vertices/triangles, cone weight, split/fill factors','build_meshlets_flex','src/meshlet.rs:1722'),('filter strides and bit precision; all four exponent modes','ExpMode','src/codec/encode.rs:93'),('opacity levels, 2/4 states, strides, dimensions, target edge','opacity_map_rasterize','src/opacity.rs:547'),('normal crease angle/smoothing','generate_normals','src/normal.rs:266'),('remesh resolution and max triangle count','remesh','src/remesh.rs:2158'),('cache FIFO size; cache analysis warp/primitive-group sizes','analyze_vertex_cache','src/analyze.rs:60'),('partition target size and optional positions','partition_clusters','src/partition.rs:998'),('strip restart sentinel; spatial cluster size','stripify','src/strip.rs:272'),('simplification attribute weights, locks, target/error; point colors/weight','simplify_with_attributes','src/simplify.rs:5005')]
for label,target,_ in controls:
    options.append(dict(surface='Shared numeric/layout controls',symbol=label,source='See corresponding function declaration in tables',status='present',port=target,port_source=at(target),optimesh='absent for normals/remesh; otherwise present',note='Supported domains follow checked API validation; this is source coverage, not exhaustive option parity testing.'))
# All enum variants are captured separately, including error/output representations.
enums=[]
for p in sorted((peer/'src').glob('*.rs')):
    if p.name in ['lib.rs','simd.rs']:continue
    active=None;depth=0
    for no,line in enumerate(p.read_text().splitlines(),1):
        m=re.search(r'pub enum (\w+)',line)
        if m:active=m[1];depth=0
        if active:
            depth+=line.count('{')-line.count('}')
            m=re.match(r'\s+([A-Z]\w*)\s*(?:,|=|\{|\()',line)
            if m:enums.append(dict(symbol=p.stem+'::'+active+'::'+m[1],source=f'src/{p.name}:{no}',classification='option' if active=='ExpMode' else 'error representation'))
            if depth==0:active=None
for e in enums:
    if e['classification']=='option':
        target=e['symbol'].split('::')[-1];matching=next(o for o in options if o['port']==target)
        options.append(dict(surface='optimesh 1.1.0',symbol=e['symbol'],source=e['source'],status='present',port=target,port_source=matching['port_source'],optimesh='present',note='Equivalent ExpMode variant.'))
# Enumerate the public C++ overload declarations individually, preserving signatures.
for no,line in enumerate(header.read_text().splitlines(),1):
    if not 1076<=no<=1167:continue
    m=re.search(r'^inline .*?\b(meshopt_\w+)\(',line)
    if not m:continue
    name=m[1];base=next(r for r in rows if r['surface']=='C++ 1.3' and r['symbol']==name)
    typed=bool(re.search(r'\b(?:T|V)\*',line))
    rows.append(dict(base,surface='C++ template overload',source=f'vendor/src/meshoptimizer.h:{no}',status='partial' if typed else 'present',note='Typed integer conversion facade absent; use explicit u32 conversion around processing or byte-width codecs.' if typed else 'Rust closures replace callback templates.',signature=line.strip()))
# Trait methods are implicitly public in Rust, including the feature-gated SIMD dispatch hooks.
trait_targets={'IndexWrite':'decode_index_buffer','MeshletVertex':'decode_meshlet','MeshletTriangle':'decode_meshlet'}
for p in sorted((peer/'src').glob('*.rs')):
    active=None;depth=0
    for no,line in enumerate(p.read_text().splitlines(),1):
        m=re.match(r'pub trait (\w+)',line)
        if m:active=m[1];depth=0
        if active:
            depth+=line.count('{')-line.count('}')
            m=re.search(r'\bfn (\w+)\(',line)
            if m:
                target=trait_targets[active]
                rows.append(dict(surface='optimesh public trait',symbol=active+'::'+m[1],source=f'src/{p.name}:{no}',status='partial',port=target,port_source=at(target),optimesh=f'src/{p.name}:{no}',note='Byte-width output behavior exists; extensible typed output trait does not. SIMD dispatch methods require the simd feature.',does='Expose typed/custom decoder output conversion without a separate byte-buffer conversion.',users='Generic asset importers and custom decoded index storage.',size='M'))
            if depth==0:active=None
for no,line in enumerate((peer/'src/vcacheoptimizer.rs').read_text().splitlines(),1):
    m=re.search(r'pub (cache|live):',line)
    if m:options.append(dict(surface='optimesh 1.1.0',symbol='VertexScoreTable::'+m[1],source=f'src/vcacheoptimizer.rs:{no}',status='missing',port='optimize_vertex_cache / optimize_vertex_cache_strip',port_source=at('optimize_vertex_cache'),optimesh='present',note='Caller-defined cache-position/live-valence scores; engine tuning users; S, same gap as optimize_vertex_cache_table.'))
result=dict(functions=rows,options=options,peer_enum_variants=enums,scope='C++ 1.3 header + cluster LOD demo; optimesh all public functions, constants, input/config fields and enum options. Output/stat/error fields are not options. Function rows mean API presence, not a new exhaustive parity proof.',sources={label:{str(p.relative_to(base)):hashlib.sha256(p.read_bytes()).hexdigest() for p in files} for label,base,files in [('cpp',cpp,[header,cpp/'vendor/demo/clusterlod.h']),('optimesh',peer,list((peer/'src').glob('*.rs')))]})
(art/'inventory.json').write_text(json.dumps(result,indent=2)+'\n')
(root/'parity/peers/inventory.json').write_text(json.dumps(result,indent=2)+'\n')
print(len(rows),'function rows;',len(options),'option rows')
