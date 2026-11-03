// Independent exhaustive distance oracle. No upstream quality API exists.
// Closest point uses Voronoi-region tests, rather than the BVH implementation.
#include <algorithm>
#include <array>
#include <cmath>
#include <cstdint>
#include <iomanip>
#include <iostream>
#include <vector>
using V=std::array<double,3>;
using T=std::array<V,3>;
V sub(V a,V b){return {a[0]-b[0],a[1]-b[1],a[2]-b[2]};}
V add(V a,V b,double s){return {a[0]+b[0]*s,a[1]+b[1]*s,a[2]+b[2]*s};}
double dot(V a,V b){return a[0]*b[0]+a[1]*b[1]+a[2]*b[2];}
double ds(V a,V b){V d=sub(a,b);return dot(d,d);}
V cross(V a,V b){return {a[1]*b[2]-a[2]*b[1],a[2]*b[0]-a[0]*b[2],a[0]*b[1]-a[1]*b[0]};}
double segment(V p,V a,V b){V e=sub(b,a);double l=dot(e,e);return ds(p,add(a,e,l?std::clamp(dot(sub(p,a),e)/l,0.,1.):0.));}
double distance(V p,T t){
 V a=t[0],b=t[1],c=t[2],ab=sub(b,a),ac=sub(c,a),ap=sub(p,a),n=cross(ab,ac);
 if(dot(n,n)==0.)return std::min({segment(p,a,b),segment(p,b,c),segment(p,c,a)});
 double d1=dot(ab,ap),d2=dot(ac,ap);if(d1<=0&&d2<=0)return ds(p,a);
 V bp=sub(p,b);double d3=dot(ab,bp),d4=dot(ac,bp);if(d3>=0&&d4<=d3)return ds(p,b);
 double vc=d1*d4-d3*d2;if(vc<=0&&d1>=0&&d3<=0)return ds(p,add(a,ab,d1/(d1-d3)));
 V cp=sub(p,c);double d5=dot(ab,cp),d6=dot(ac,cp);if(d6>=0&&d5<=d6)return ds(p,c);
 double vb=d5*d2-d1*d6;if(vb<=0&&d2>=0&&d6<=0)return ds(p,add(a,ac,d2/(d2-d6)));
 double va=d3*d6-d5*d4;if(va<=0&&(d4-d3)>=0&&(d5-d6)>=0)return ds(p,add(b,sub(c,b),(d4-d3)/((d4-d3)+(d5-d6))));
 double inv=1./(va+vb+vc);return ds(p,add(add(a,ab,vb*inv),ac,vc*inv));
}
uint32_t reverse(uint32_t v){uint32_t r=0;for(int i=0;i<32;++i){r=(r<<1)|(v&1);v>>=1;}return r;}
void measure(const std::vector<T>& a,const std::vector<T>& b,double density){
 double max=0,sum=0;uint64_t count=0;
 auto sample=[&](V p){double d=INFINITY;for(T t:b)d=std::min(d,distance(p,t));max=std::max(max,d);sum+=d;++count;};
 for(T t:a){for(V v:t)sample(v);V n=cross(sub(t[1],t[0]),sub(t[2],t[0]));uint32_t num=uint32_t(std::ceil(.5*std::sqrt(dot(n,n))*density));
  for(uint32_t j=0;j<num;++j){double r=std::sqrt((double(j)+.5)/num),v=(double(reverse(j))+.5)/4294967296.;V p;for(int k=0;k<3;++k)p[k]=t[0][k]*(1-r)+t[1][k]*r*(1-v)+t[2][k]*r*v;sample(p);}}
 std::cout<<std::sqrt(max)<<' '<<std::sqrt(sum/count)<<' '<<count<<' ';
}
int main(){std::cout<<std::setprecision(17);size_t na,nb;double density;while(std::cin>>na>>nb>>density){std::vector<T>a(na),b(nb);for(auto* m:{&a,&b})for(auto& t:*m)for(auto& p:t)for(double& x:p)std::cin>>x;measure(a,b,density);measure(b,a,density);std::cout<<'\n';}}
