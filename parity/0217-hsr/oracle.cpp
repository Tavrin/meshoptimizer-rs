// Independent brute-force sampled visibility oracle, NOT an upstream HSR API.
// Binary mesh prefix input, output original kept u32 indices. Dedicated CC0 code.
#include <array>
#include <vector>
#include <fstream>
#include <cmath>
#include <algorithm>
#include <cstdint>
#include <limits>
using P=std::array<double,3>;
P sub(P a,P b){return {a[0]-b[0],a[1]-b[1],a[2]-b[2]};}
P cross(P a,P b){return {a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]};}
double dot(P a,P b){return (a[0]*b[0]+a[1]*b[1])+a[2]*b[2];}
int main(int argc,char**argv){
 if(argc!=4)return 2;std::ifstream f(argv[1],std::ios::binary);uint32_t n=0,ni=0;f.read((char*)&n,4);f.read((char*)&ni,4);
 std::vector<P> p(n);double scale=1.;for(auto &v:p)for(auto &x:v){float z;f.read((char*)&z,4);x=z;scale=std::max(scale,std::abs(x));}
 std::vector<uint32_t> i(ni);f.read((char*)i.data(),ni*4);if(!f)return 3;
 unsigned s=unsigned(std::stoi(argv[3]));double eps=64*std::numeric_limits<double>::epsilon()*scale;
 std::ofstream out(argv[2],std::ios::binary);
 for(size_t t=0;t<i.size();t+=3){bool visible=false;auto a=p.at(i[t]),b=p.at(i[t+1]),c=p.at(i[t+2]);
 auto normal=cross(sub(b,a),sub(c,a));if(dot(normal,normal)==0)visible=true;
 for(unsigned r=0;r<=s+1&&!visible;r++)for(unsigned col=0;col<=(r==s+1?0:s-r)&&!visible;col++){
 double u=r==s+1?1./3.:double(r)/s,v=r==s+1?1./3.:double(col)/s;
 P o;for(int k=0;k<3;k++)o[k]=a[k]*(1-u-v)+b[k]*u+c[k]*v;
 for(int x=-1;x<=1&&!visible;x++)for(int y=-1;y<=1&&!visible;y++)for(int z=-1;z<=1&&!visible;z++){
 if(x==0&&y==0&&z==0)continue;double len=std::sqrt(double(x*x+y*y+z*z));P d={x/len,y/len,z/len};bool blocked=false;
 for(size_t q=0;q<i.size()&&!blocked;q+=3){if(q==t)continue;auto aa=p.at(i[q]);auto e1=sub(p.at(i[q+1]),aa),e2=sub(p.at(i[q+2]),aa);auto h=cross(d,e2);double det=dot(e1,h);if(det==0)continue;auto rel=sub(o,aa);double bu=dot(rel,h)/det;auto rr=cross(rel,e1);double bv=dot(d,rr)/det,dist=dot(e2,rr)/det;
 blocked=bu>=0&&bv>=0&&bu+bv<=1&&dist>eps;}
 if(!blocked)visible=true;
 }}
 if(visible)out.write((char*)&i[t],12);
 }
 return out?0:4;
}
