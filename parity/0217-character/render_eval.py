"""CPU front-view diagnostic raster, original / plain / character in one pose.

Usage: render_eval.py external-campaign-directory
Ranks characters by their 10% maximum sampled deviation divided by posed extent.
The same worst-pose coordinates and orthographic framing are used in all panels.
"""
import csv
import json
import pathlib
import sys
import numpy as np
from PIL import Image, ImageDraw


def render(root):
    allowed={s['model'] for s in json.loads(pathlib.Path(__file__).with_name('SOURCES.json').read_text()) if s['role']=='character'}
    ranked=[]
    input_root=root/'final-inputs' if (root/'final-inputs').exists() else root/'models'
    for meta in input_root.glob('*/*.eval.json'):
        name=meta.name.removesuffix('.eval.json')
        if name not in allowed:continue
        csvpath=root/'logs'/('final-'+name+'.csv')
        if not csvpath.exists():csvpath=root/'logs'/(name+'.csv')
        if not csvpath.exists():continue
        rows=list(csv.DictReader(csvpath.open()))
        candidates=[r for r in rows if r['ratio']=='0.1' and r['status']=='ok']
        if not candidates:continue
        row=max(candidates,key=lambda r:max(float(r['animation_max']),float(r['morph_max']),float(r['bind'])))
        base=str(meta).removesuffix('.eval.json')+'.eval.bin'
        pose=np.fromfile(f'{base}.0.1.{row["method"]}.pose',dtype='<f4').reshape(-1,3)
        extent=np.ptp(pose,axis=0).max()
        score=max(float(row[k]) for k in ['animation_max','morph_max','bind'])/max(extent,1e-30)
        ranked.append((score,name,base,pose,row,rows))
    out=root/'renders';out.mkdir(exist_ok=True)
    records=[]
    for score,name,base,pose,row,rows in sorted(ranked,key=lambda r:(r[0],r[1]),reverse=True)[:5]:
        if name == 'Fox':
            pose=np.stack([-pose[:,2],pose[:,1],pose[:,0]],axis=1)
        image=Image.new('RGB',(960,360),'#f3f5f8');draw=ImageDraw.Draw(image)
        lo=pose.min(axis=0);hi=pose.max(axis=0);center=(lo+hi)/2
        scale=260/max(hi[0]-lo[0],hi[1]-lo[1],1e-30)
        xy=(pose[:,:2]-center[:2])*[scale,-scale]+[160,190]
        source=np.fromfile(base+'.source.indices',dtype='<u4').reshape(-1,3)
        for panel,method in enumerate(['source','plain','character']):
            draw.text((panel*320+12,12),name+' / '+method,fill='#192532')
            if method=='source':triangles=source
            else:
                path=pathlib.Path(f'{base}.0.1.{method}.indices')
                if not path.exists():
                    draw.text((panel*320+20,170),'InvalidLayout (channel limit)',fill='#a33838');continue
                triangles=np.fromfile(path,dtype='<u4').reshape(-1,3)
            xyz=pose[triangles];depth=xyz[:,:,2].mean(axis=1)
            for t in triangles[np.argsort(depth)]:
                points=[(float(xy[i,0]+panel*320),float(xy[i,1])) for i in t]
                normal=np.cross(pose[t[1]]-pose[t[0]],pose[t[2]]-pose[t[0]])
                shade=abs(normal[2])/max(np.linalg.norm(normal),1e-30)
                color=tuple(int(v*(0.4+0.6*shade)) for v in (93,158,199))
                draw.polygon(points,fill=color,outline='#314252' if len(triangles)<=5000 else None)
            draw.text((panel*320+12,335),f'{len(triangles)} triangles',fill='#192532')
        image.save(out/(name+'.png'))
        records.append({'model':name,'score':float(score),'worst_pose_index':row['worst_pose'],'pose_selected_from':row['method'],'front_axis':'+X' if name == 'Fox' else '+Z','ratio':0.1})
    contact=Image.new('RGB',(960,360*len(records)),'white')
    for i,record in enumerate(records):contact.paste(Image.open(out/(record['model']+'.png')),(0,360*i))
    if records:contact.save(out/'contactsheet.png')
    (out/'selection.json').write_text(json.dumps(records,indent=2)+'\n')
    print(json.dumps(records,indent=2))


if __name__=='__main__':render(pathlib.Path(sys.argv[1]))
