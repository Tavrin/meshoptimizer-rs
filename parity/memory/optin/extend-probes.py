from pathlib import Path
s=Path(__file__).parent;old=s.parent/'spec027/probe-library'
state=(old/'src/simplify.rs').read_text();state=state[state.rindex('impl State { pub(crate) fn diag_fields'):]
workspace=(old/'src/workspace.rs').read_text();workspace=workspace[workspace.rindex('impl Workspace {'):]
for arm in ['A','B']:
 lib=s/('probe-library-'+arm)
 for name,tail in [('simplify.rs',state),('workspace.rs',workspace)]:
  p=lib/'src'/name;t=p.read_text()
  if 'fn diag_fields' not in t:p.write_text(t+'\n'+tail)
 p=s/('probe-harness-'+arm)/'src/backend.rs';t=p.read_text().replace('let detail=json!({"retained":','let detail=json!({"fields":self.workspace.diag_fields(),"retained":');p.write_text(t)
