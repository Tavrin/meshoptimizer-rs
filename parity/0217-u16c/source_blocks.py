import re
pattern=re.compile(r'(?m)^#\[cfg\([^\n]*\btest\b[^\n]*\)\]\n(?:#\[[^\n]+\]\n)*mod (\w+)\s*\{')
def end_block(s,i):
 depth=1
 while depth:
  if s.startswith('//',i):
   pos=s.find('\n',i);i=len(s) if pos<0 else pos+1;continue
  if s.startswith('/*',i):
   level=1;i+=2
   while level:
    if s.startswith('/*',i):level+=1;i+=2
    elif s.startswith('*/',i):level-=1;i+=2
    else:i+=1
   continue
  raw=re.match(r'r(#+)?"',s[i:]) if s[i]=='r' else None
  if raw:
   marker='"'+(raw[1] or '');i=s.index(marker,i+raw.end())+len(marker);continue
  if s[i]=='"':
   i+=1
   while s[i]!='"':i+=2 if s[i]=='\\' else 1
   i+=1;continue
  char=re.match(r"'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^\\'\n])'",s[i:]) if s[i]=="'" else None
  if char:i+=char.end();continue
  depth+=(s[i]=='{')-(s[i]=='}');i+=1
 return i
def blocks(s):
 return [(m[1],m.start(),end_block(s,m.end()),s[m.start():end_block(s,m.end())]) for m in pattern.finditer(s)]
