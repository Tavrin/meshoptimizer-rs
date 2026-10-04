// Unmodified upstream algorithms, protocol-2 preprocessing adapter.
struct CustomContext { std::vector<unsigned> calls;unsigned mode; };
static int custom_equal(void* context, unsigned a, unsigned b) {
    auto& c=*static_cast<CustomContext*>(context);
    c.calls.push_back(a);c.calls.push_back(b);return c.mode&8?true:c.mode&16?false:a%3==b%3;
}
static int preprocessing(const std::vector<unsigned char>& in,bool paired) {
    if(in.size()<56) throw std::runtime_error("short preprocessing message");
    unsigned op=read(in,4),vc=read(in,8),ic=read(in,12),mode=read(in,16),samples=read(in,20),size=read(in,24),stride=read(in,28),sc=read(in,32),options=read(in,36),target=read(in,40),p0=read(in,44),p1=read(in,48),ac=read(in,52);
    if(op<7||op>38||mode>2||samples>100||sc>16||ac>32||(!mode&&samples)||(mode&&!samples)) throw std::runtime_error("invalid preprocessing mode");
    uint64_t length=56+uint64_t(vc)*12+uint64_t(ic)*4+uint64_t(sc)*vc*stride+uint64_t(ac)*4+uint64_t(vc)*ac*4+uint64_t(vc)*4;
    if(!size||size>256||stride<size||stride>128*1024*1024)throw std::runtime_error("invalid transport layout");
    if(length!=in.size())throw std::runtime_error("preprocessing length");
    size_t at=56;std::vector<float> positions(vc*3);for(auto& f:positions){f=as_float(read(in,at));at+=4;}
    std::vector<unsigned> indices(ic);for(auto& i:indices){i=read(in,at);at+=4;}
    std::vector<meshopt_Stream> streams;for(unsigned i=0;i<sc;i++){streams.push_back({in.data()+at,size,stride});at+=size_t(vc)*stride;}
    std::vector<float> weights(ac),attributes(size_t(vc)*ac);for(auto& f:weights){f=as_float(read(in,at));at+=4;}for(auto& f:attributes){f=as_float(read(in,at));at+=4;}
    std::vector<unsigned char> flags(vc);for(auto& f:flags){f=static_cast<unsigned char>(read(in,at));at+=4;}

    bool caller=mode==2||(options&0x80000000);options&=0x7fffffff;
    if(op==33&&vc!=2)throw std::runtime_error("exponent bounds");
    size_t capacity=op==19?size_t(ic)*2:op==20?size_t(ic)*4:op==9||op==10||op==11||op==18||op==23?size_t(vc):op==13?std::max(size_t(vc),size_t(ic)):op==26?size_t(target):op>=28&&op<=32?size_t(ic):op==12||op==22||op==27||op==33?0:size_t(ic);
    size_t byte_capacity=(op==12||op==22)?size_t(vc)*size:0;
    if(capacity*4>128*1024*1024||byte_capacity>128*1024*1024)throw std::runtime_error("transport output limit");
    std::vector<unsigned> destination(capacity),caller_reorder(size_t(vc)+ic/3),buffer_remap(vc);
    std::vector<unsigned char> bytes(byte_capacity);std::vector<float> colors(vc*3);
    size_t remapped_bytes=0;
    for(unsigned i=0;i<vc;i++){buffer_remap[i]=op==12&&i%5==0?~0u:vc-1-i;if(buffer_remap[i]!=~0u)remapped_bytes=std::max(remapped_bytes,(size_t(buffer_remap[i])+1)*size);colors[i*3]=float(i%3);colors[i*3+1]=float(i%5);colors[i*3+2]=float(i%7);}
    if(op==13&&(p1&2)){if(sc!=1||size!=4||stride!=4)throw std::runtime_error("explicit remap layout");for(unsigned i=0;i<vc;i++){const auto* b=static_cast<const unsigned char*>(streams[0].data)+i*4;buffer_remap[i]=unsigned(b[0])|(unsigned(b[1])<<8)|(unsigned(b[2])<<16)|(unsigned(b[3])<<24);}}
    if(op==26&&(p1&4)){if(ac!=3)throw std::runtime_error("explicit color layout");colors=attributes;}
    struct Payload{std::vector<unsigned> indices,reorder,calls;std::vector<unsigned char> bytes;std::vector<float> positions,attributes;size_t count=0;float error=0;};
    meshopt_setAllocator(tracked_allocate,tracked_free);
    auto operation=[&](){
        Payload r;r.count=ic;if(!caller && !(op>=28&&op<=32))r.indices.resize(capacity);unsigned* dest=caller?destination.data():r.indices.data();
        unsigned dummy=0;auto optional=p1&1?nullptr:(indices.empty()?&dummy:indices.data());
        if((op<28 || op>=34) && op!=18 && op!=26){
            bool triangles=op==7||op==8||(op>=14&&op<=17)||(op>=19&&op<=21)||op==24||op==25||op==27||op>=34;
            if(triangles&&ic%3)throw std::runtime_error("invalid topology");
            if(op!=13&&(optional||!(op==9||op==10||op==11)))for(auto i:indices)if(i>=vc)throw std::runtime_error("invalid index");
        }
        if(op==13){bool missing=false;if(optional){for(auto i:indices){if(i>=vc)throw std::runtime_error("invalid index");missing|=buffer_remap[i]==~0u;}}else{for(auto r:buffer_remap)missing|=r==~0u;}if(missing)throw std::runtime_error("unused remap reference");}
        if(ic&&((op==28&&target>30)||(op==29&&(target<1||target>31))||(op==31&&target>23)))throw std::runtime_error("invalid quantization parameter");
        if(op==26){if(target>vc||!std::isfinite(as_float(p0)))throw std::runtime_error("invalid point parameter");if(p1&2)for(float c:colors)if(!std::isfinite(c))throw std::runtime_error("invalid point color");}
        if((op==24||op==27||op>=34) && (target>ic||!std::isfinite(as_float(p0))||as_float(p0)<0))throw std::runtime_error("invalid target");
        if(op==24||op==25||op==26||op==27||op>=34){for(float p:positions)if(!std::isfinite(p))throw std::runtime_error("invalid position");if(op==27||op>=34){for(float a:attributes)if(!std::isfinite(a))throw std::runtime_error("invalid attribute");for(float w:weights)if(!std::isfinite(w)||w<0)throw std::runtime_error("invalid weight");}}
        switch(op){
        case 7:meshopt_optimizeVertexCacheStrip(dest,indices.data(),ic,vc);break;
        case 8:meshopt_optimizeVertexCacheFifo(dest,indices.data(),ic,vc,p0);break;
        case 9:r.count=meshopt_generateVertexRemap(dest,optional,optional?ic:vc,streams[0].data,vc,size);break;
        case 10:r.count=meshopt_generateVertexRemapMulti(dest,optional,optional?ic:vc,vc,streams.data(),sc);break;
        case 11:{CustomContext c{{},p1};r.count=meshopt_generateVertexRemapCustom(dest,optional,optional?ic:vc,positions.data(),vc,12,p1&4?nullptr:custom_equal,&c);r.calls=std::move(c.calls);break;}
        case 12:{if(!caller)r.bytes.resize(remapped_bytes);auto out=caller?bytes.data():r.bytes.data();if(caller)std::fill(bytes.begin(),bytes.end(),0);meshopt_remapVertexBuffer(out,streams[0].data,vc,size,buffer_remap.data());r.count=remapped_bytes;break;}
        case 13:r.count=optional?ic:vc;meshopt_remapIndexBuffer(dest,optional,r.count,buffer_remap.data());break;
        case 14:r.count=meshopt_filterIndexBuffer(dest,indices.data(),ic,streams[0].data,vc,size,stride);break;
        case 15:r.count=meshopt_filterIndexBufferMulti(dest,indices.data(),ic,vc,streams.data(),sc);break;
        case 16:meshopt_generateShadowIndexBuffer(dest,indices.data(),ic,streams[0].data,vc,size,stride);break;
        case 17:meshopt_generateShadowIndexBufferMulti(dest,indices.data(),ic,vc,streams.data(),sc);break;
        case 18:r.count=vc;meshopt_generatePositionRemap(dest,positions.data(),vc,12);break;
        case 19:r.count=ic*2;meshopt_generateAdjacencyIndexBuffer(dest,indices.data(),ic,positions.data(),vc,12);break;
        case 20:r.count=ic*4;meshopt_generateTessellationIndexBuffer(dest,indices.data(),ic,positions.data(),vc,12);break;
        case 21:{if(!caller)r.reorder.resize(size_t(vc)+ic/3);auto reorder=caller?caller_reorder.data():r.reorder.data();r.count=meshopt_generateProvokingIndexBuffer(dest,reorder,indices.data(),ic,vc);break;}
        case 22:{r.indices=indices;if(!caller)r.bytes.resize(size_t(vc)*size);r.count=meshopt_optimizeVertexFetch(caller?bytes.data():r.bytes.data(),r.indices.data(),ic,streams[0].data,vc,size);break;}
        case 23:r.count=meshopt_optimizeVertexFetchRemap(dest,indices.data(),ic,vc);break;
        case 24:r.count=meshopt_simplifySloppy(dest,indices.data(),ic,positions.data(),vc,12,p1&2?flags.data():nullptr,target,as_float(p0),&r.error);break;
        case 25:r.count=meshopt_simplifyPrune(dest,indices.data(),ic,positions.data(),vc,12,as_float(p0));break;
        case 26:r.count=meshopt_simplifyPoints(dest,positions.data(),vc,12,p1&2?colors.data():nullptr,12,as_float(p0),target);break;
        case 27:r.positions=positions;r.attributes=attributes;r.indices=indices;r.count=meshopt_simplifyWithUpdate(r.indices.data(),ic,r.positions.data(),vc,12,r.attributes.data(),ac*4,weights.data(),ac,flags.data(),target,as_float(p0),options,&r.error);break;
        case 28:case 29:case 30:case 31:case 32:{
            switch(op){
            case 28:for(size_t j=0;j<ic;j++)destination[j]=unsigned(meshopt_quantizeUnorm(as_float(indices[j]),int(target)));break;
            case 29:for(size_t j=0;j<ic;j++)destination[j]=unsigned(meshopt_quantizeSnorm(as_float(indices[j]),int(target)));break;
            case 30:for(size_t j=0;j<ic;j++)destination[j]=meshopt_quantizeHalf(as_float(indices[j]));break;
            case 31:for(size_t j=0;j<ic;j++)destination[j]=bits(meshopt_quantizeFloat(as_float(indices[j]),int(target)));break;
            default:for(size_t j=0;j<ic;j++)destination[j]=bits(meshopt_dequantizeHalf(static_cast<unsigned short>(indices[j])));break;
            }break;}
        case 33:{
            if(int(p0)<-126||p1<2||p1>24)throw std::runtime_error("invalid exponent parameter");
            for(int k=0;k<3;k++)if(!std::isfinite(positions[k])||!std::isfinite(positions[k+3])||positions[k]>positions[k+3])throw std::runtime_error("invalid exponent bounds");
            r.count=unsigned(meshopt_computePositionExponent(positions.data(),positions.data()+3,int(p0),int(p1)));break;
        }
        default:r.count=meshopt_simplifyWithAttributes(dest,indices.data(),ic,positions.data(),vc,12,attributes.data(),ac*4,weights.data(),ac,flags.data(),target,as_float(p0),options,&r.error);break;
        }return r;
    };
    auto emit=[&](const Payload& r){std::vector<unsigned> out;auto values=caller?destination.data():r.indices.data();auto append=[&](const unsigned* p,size_t n){if(n)out.insert(out.end(),p,p+n);};
        if(op==9||op==10||op==11||op==23){out.push_back(unsigned(r.count));append(values,vc);if(op==11){out.push_back(unsigned(r.calls.size()));append(r.calls.data(),r.calls.size());}}
        else if(op==12){auto b=caller?bytes.data():r.bytes.data();for(size_t i=0;i<r.count;i++)out.push_back(b[i]);}
        else if(op==21){out.push_back(unsigned(r.count));append(values,ic);append(caller?caller_reorder.data():r.reorder.data(),r.count);}
        else if(op==22){out.push_back(unsigned(r.count));append(r.indices.data(),ic);auto b=caller?bytes.data():r.bytes.data();for(size_t i=0;i<r.count*size;i++)out.push_back(b[i]);}
        else if(op==27){out.push_back(bits(r.error));out.push_back(unsigned(r.count));append(r.indices.data(),r.count);for(auto p:r.positions)out.push_back(bits(p));for(auto a:r.attributes)out.push_back(bits(a));}
        else if(op==33)out.push_back(unsigned(r.count));
        else {if(op==24||op>=34)out.push_back(bits(r.error));if(op>=28&&op<=32)values=destination.data();append(values,r.count);}
        return out;
    };
    auto result=operation();size_t output_bytes=caller||op>=28&&op<=33||op==27?0:result.indices.capacity()*4+result.reorder.capacity()*4+result.bytes.capacity();size_t memory=peak_bytes+output_bytes;tracking=false;auto expected=emit(result);std::vector<double> times;
    if(paired){std::cout.put('R');std::cout.flush();}
    for(unsigned s=0;s<samples;s++){if(paired){char cmd;if(!std::cin.get(cmd))throw std::runtime_error("paired EOF");if(cmd=='S')break;if(cmd!='R')throw std::runtime_error("paired command");}auto start=std::chrono::steady_clock::now();unsigned repeats=op==33?262144:ic<3000?32768:ic<300000?16:1;for(unsigned j=0;j<repeats;j++)result=operation();double t=std::chrono::duration<double>(std::chrono::steady_clock::now()-start).count()/repeats;times.push_back(t);if(paired){uint64_t b;std::memcpy(&b,&t,8);std::cout.put('T');for(int k=0;k<8;k++)std::cout.put(char(b>>(k*8)));std::cout.flush();}}
    auto out=emit(result);if(out!=expected)throw std::runtime_error("unstable output");encoded.insert(encoded.end(),{'M','R','0','1'});write(0);write(unsigned(out.size()));write(unsigned(times.size()));for(auto i:out)write(i);for(auto t:times){uint64_t b;std::memcpy(&b,&t,8);for(int i=0;i<8;i++)encoded.push_back(char(b>>(i*8)));}if(mode){write(unsigned(memory));write(unsigned(memory>>32));}std::cout.write(encoded.data(),std::streamsize(encoded.size()));return std::cout.good()?0:1;
}
