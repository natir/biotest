//! VCF generation
//!
//! Usage:
//! ```no_run
//! use biotest::Format as _; // import Format trait is required
//!
//! # fn main() -> Result<(), biotest::error::Error> {
//! let mut rng = biotest::rand(); // Create a random generator with a fixed seed
//!
//! let mut output = Vec::new();
//! let generator = biotest::Vcf::default();
//!
//! // Write one vcf record in output with 3 samples and all possible INFO and FORMAT
//! generator.record(&mut output, &mut rng)?;
//! generator.records(&mut output, &mut rng, 5)?; // Write five vcf records in output
//!
//! // Write five vcf record in "test.vcf" with complete header
//! generator.create("test.vcf", &mut rng, 5)?;
//! # Ok(())
//! # }
//! ```
//!

/* std use */

/* crates use */

/* projet use */
use crate::error;

/* module declaration */
pub mod header;
pub mod record;

/// Struct to generate random fastq record
#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
pub struct Vcf {
    /// Structure to define header
    #[builder(default = "header::Header::default()")]
    header: header::Header,

    /// Structure to define record
    #[builder(default = "record::Record::default()")]
    record: record::Record,
}

impl Vcf {
    /// Create a VcfBuilder
    pub fn builder() -> VcfBuilder {
        VcfBuilder::default()
    }
}

impl core::default::Default for Vcf {
    fn default() -> Self {
        VcfBuilder::default().build().unwrap() // it's default no error
    }
}

impl crate::format::Format for Vcf {
    fn header(
        &self,
        output: &mut dyn std::io::Write,
        _rng: &mut rand::rngs::StdRng,
    ) -> error::Result<()> {
        self.header.generate(output)
    }

    fn record(
        &self,
        output: &mut dyn std::io::Write,
        rng: &mut rand::rngs::StdRng,
    ) -> error::Result<()> {
        self.record.generate(output, rng)
    }
}

#[cfg(test)]
mod tests {
    /* std use */
    use std::io::Read as _;

    /* project use */
    use super::*;
    use crate::format::Format as _;

    const DEFAULT: &[u8] = b"##fileformat=VCFv4.3\n##contig=<ID=chr1,length=4294967295,species=\"random\">\n##contig=<ID=23,length=4294967295,species=\"random\">\n##contig=<ID=93,length=4294967295,species=\"random\">\n##contig=<ID=chrMT,length=4294967295,species=\"random\">\n##contig=<ID=X,length=4294967295,species=\"random\">\n##contig=<ID=NC_000015.10,length=4294967295,species=\"random\">\n##contig=<ID=ENA|LT795502|LT795502.1,length=4294967295,species=\"random\">\n##contig=<ID=NC_016845.1,length=4294967295,species=\"random\">\n##contig=<ID=YAR028W,length=4294967295,species=\"random\">\n##contig=<ID=1,length=4294967295,species=\"random\">\n##FILTER=<ID=filter_0,Description=\"generated vcf filter field\">\n##FILTER=<ID=filter_1,Description=\"generated vcf filter field\">\n##FILTER=<ID=filter_2,Description=\"generated vcf filter field\">\n##INFO=<ID=info_Integer_1,Number=1,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Integer_2,Number=2,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Integer_A,Number=A,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Integer_R,Number=R,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Integer_G,Number=G,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Integer_.,Number=.,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Float_1,Number=1,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Float_2,Number=2,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Float_A,Number=A,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Float_R,Number=R,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Float_G,Number=G,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Float_.,Number=.,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Flag_0,Number=0,Type=Flag,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Character_1,Number=1,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Character_2,Number=2,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Character_A,Number=A,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Character_R,Number=R,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Character_G,Number=G,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_Character_.,Number=.,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_String_1,Number=1,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_String_2,Number=2,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_String_A,Number=A,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_String_R,Number=R,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_String_G,Number=G,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##INFO=<ID=info_String_.,Number=.,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">\n##FORMAT=<ID=format_Integer_1,Number=1,Type=Integer,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Integer_2,Number=2,Type=Integer,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Integer_A,Number=A,Type=Integer,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Integer_R,Number=R,Type=Integer,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Integer_G,Number=G,Type=Integer,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Integer_.,Number=.,Type=Integer,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Float_1,Number=1,Type=Float,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Float_2,Number=2,Type=Float,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Float_A,Number=A,Type=Float,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Float_R,Number=R,Type=Float,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Float_G,Number=G,Type=Float,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Float_.,Number=.,Type=Float,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Character_1,Number=1,Type=Character,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Character_2,Number=2,Type=Character,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Character_A,Number=A,Type=Character,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Character_R,Number=R,Type=Character,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Character_G,Number=G,Type=Character,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_Character_.,Number=.,Type=Character,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_String_1,Number=1,Type=String,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_String_2,Number=2,Type=String,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_String_A,Number=A,Type=String,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_String_R,Number=R,Type=String,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_String_G,Number=G,Type=String,Description=\"generated vcf format field\">\n##FORMAT=<ID=format_String_.,Number=.,Type=String,Description=\"generated vcf format field\">\n#CHROM\tPOS\tID\tREF\tALT\tQUAL\tFILTER\tINFO\tFORMAT\tsample_0\tsample_1\tsample_2\nYAR028W\t509242864\t.\tA\t.\t224\tfilter_0\tinfo_Integer_1=-1867486109;info_Integer_2=1180908492,1041698939;info_Integer_A=-207506017;info_Integer_R=-1221871790,-1356802783;info_Integer_G=-496257857,2127853583,-1498117423;info_Integer_.=2082620030,-344161843,-1022296784,-1007334138;info_Float_1=68.286865;info_Float_2=-96.154594,-23.433853;info_Float_A=-48.782158;info_Float_R=-46.15216,-92.639305;info_Float_G=-7.5115204,74.78337,1.5983124;info_Float_.=26.825455;info_Flag_0;info_Character_1=i;info_Character_2=r,[;info_Character_A=g;info_Character_R=M,D;info_Character_G=h,w,\\;info_Character_.=C,G,p,];info_String_1=ZoXMT;info_String_2=gQouV,Gn`Jw;info_String_A=eVDDU;info_String_R=YytzA,ny[_P;info_String_G=Oshsq,bSjAd,bZcRF;info_String_.=rQ_[V,S^RtS,vzMeT,jonYV\tformat_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.\t-1552897203:1249370088,894744660:-1298826907:-1500526673,846767901:154354090,1292630937,-513388490:730433769,-1782228224,1193004039,1639963889:-31.463745:-74.13223,44.792007:-4.5392303:-42.586063,-20.249939:-19.714546,-48.754406,40.519638:-27.838158:L:J,L:n:u,P:t,f,`:r,^:aaSsw:svYGC,zkT\\W:k_sGD:gZcCc,]tIGE:bcnVW,JVaDB,nQSHY:[QBCg,L`Scx,xXYm`,NnOG[\t-1345745815:173280036,-939420073:-1365650667:679852521,1295053734:732715199,-819759668,-308523151:1942972144,-249711286,1737760149:-53.047443:-97.35165,-58.53014:93.27409:-89.49225,65.68997:62.677032,92.94722,32.79944:52.132156,-30.33149:z:R,v:G:G,X:B,g,q:[,a,B:w_Zxx:kAFA[,o`OId:JgjZD:StKau,vtaIh:wmmrI,gNXcb,hRd]Q:OgukS\t946791943:-2019035904,1055813342:-2045085244:-1401538285,878536766:731752434,1439145027,-966674455:-1096509554,-1513894259,1176983779,-199713084:51.48242:-93.36465,6.6719513:32.869843:-77.50437,-17.745377:38.63495,-9.558914,42.16661:-6.823944,-39.047478,48.595016,68.83052:w:O,m:A:i,Z:P,w,y:s:KBssX:JGMMK,`HVkg:oY`vk:xarZo,yTnQF:EntKU,mnaDW,uppug:FhYRx,BZHMq\n93\t2036067340\t.\tT\t.\t3\tfilter_2\tinfo_Integer_1=-945359064;info_Integer_2=2042408529,-281042636;info_Integer_A=1400195836;info_Integer_R=158409543,1664317966;info_Integer_G=-678096394,1218409815,-280169010;info_Integer_.=-429223473,-798239102,1447160136;info_Float_1=-6.572792;info_Float_2=-69.61241,4.734352;info_Float_A=-75.17469;info_Float_R=54.42581,-98.062325;info_Float_G=-23.31765,-19.276001,-94.52958;info_Float_.=-96.97473;info_Flag_0;info_Character_1=L;info_Character_2=_,`;info_Character_A=e;info_Character_R=b,j;info_Character_G=E,N,F;info_Character_.=`;info_String_1=pNSPd;info_String_2=^wz^t,ZVmq_;info_String_A=oBYJg;info_String_R=Q`oPn,^Z\\`b;info_String_G=la^yz,IWtrg,moGx];info_String_.=cWVPn,iuT_I,lSskB\tformat_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.\t-1281703324:1908114858,-1293150474:-1602964452:1056979575,-963728372:1402654398,-1718558894,95336780:-1647348167,1565404409,1392446648,-1451772547:-40.51242:-35.81727,89.454605:-6.7819138:11.500885,-47.349976:86.69888,-0.38061523,-49.81668:-85.17905:B:b,Z:v:x,F:j,H,G:`,Y,P,f:IFpIP:NevhO,TNvrC:wSEWb:LtHSU,iCnsI:sSMCB,y^pRI,Q\\eLD:RzYyz,V_szw\t1429610357:667106503,-347005078:-1450892043:187576632,-731059940:2111030852,810139033,-1298935060:312049871,870382568,-1387207741,1225417725:21.764038:-99.951675,-45.165207:39.073135:64.91058,77.78046:63.085556,-47.436356,-39.503193:-42.086887,93.22655:B:P,e:N:q,L:a,R,r:F,J:cjVaE:rHHlo,MTrco:Gyzgq:iA_WI,LkXIc:`ot_P,Zwl^\\,Uz^rc:ndZg_,IpyMn\t-298362881:554100151,-1638105763:-813444979:285964333,-159387058:-2103781190,-330220715,-544136330:-1391083738,-712907221:63.459488:48.598007,-64.342575:65.56694:-0.56829834,-94.79799:-28.613068,81.39114,68.24536:39.960556,73.48726:`:N,R:y:Q,G:D,w,G:`:RjrNl:hQqYM,wuBIP:QMGV`:vWunB,R\\Okz:KMcfE,GOhnT,sJUAE:tKWGL,KatQJ,Qxq\\g,^^xfH\nX\t2138516245\t.\tA\t.\t58\tfilter_0\tinfo_Integer_1=1885408291;info_Integer_2=922014433,1238890301;info_Integer_A=-474357293;info_Integer_R=-1290577166,1120909097;info_Integer_G=-186729156,1973040123,-422535277;info_Integer_.=-183630805,1867038567,1281678071;info_Float_1=-95.3896;info_Float_2=51.682953,-31.031967;info_Float_A=46.307205;info_Float_R=8.105705,35.940765;info_Float_G=-34.93447,72.228195,-83.08275;info_Float_.=-82.36182;info_Character_1=H;info_Character_2=o,R;info_Character_A=q;info_Character_R=Z,v;info_Character_G=z,y,];info_Character_.=m;info_String_1=EhXGl;info_String_2=\\R[nM,ljhET;info_String_A=zoPwT;info_String_R=hoTOP,mgWvu;info_String_G=BiQUb,oxchY,cBuGc;info_String_.=LjZa],OQjTt,_wcgT,tO\\lL\tformat_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.\t1581362196:-1447784025,-1525232815:-1035525905:-1979111486,-1258086770:428899970,1146438819,-1211106230:634110781:61.013245:11.870453,-10.159973:-17.56308:-89.45878,-27.66838:-34.221336,81.52295,87.46088:22.250458:T:G,D:Y:X,C:r,I,y:b,B,D:UVwho:[vJ`a,muhok:xubhH:L^ANq,ulNzZ:Mpgwy,XlyYC,qgVMS:zrlVo\t1082793408:-1252548594,195759635:-1056145565:1294511812,1376570218:-1300715418,-1097690924,-72191116:-416974155,1624764853,-68133459,-693202153:-81.10368:-62.194897,-10.530067:-69.0829:-94.7505,48.184113:-68.95244,86.32448,-70.56353:74.956985:]:d,Z:Q:F,m:e,W,J:^:PamSD:QW[XW,\\\\Hpl:aP_dw:Tkfvm,vkqDI:xIXno,EQsVH,lECpM:P_hBj,XI`Pk,lhoEn,nDGdq\t-1756403182:-1210584648,1067164580:-2026752630:1524204479,2063402043:-1671581241,1992411203,-255678314:387204102,-2048329797:29.60765:-70.24462,91.82048:-57.780792:-19.511703,87.46164:17.362617,-10.059616,-89.640594:-4.2216034,31.744385:n:v,T:E:N,p:],Y,R:p,A,W,i:yIqiC:w]gp_,[s]vD:hY\\Sm:ynkIV,^tOuG:kqHsi,EQDdb,nppEh:gQkWC,CgEHr\nNC_000015.10\t1204106469\t.\tc\t.\t163\tfilter_2\tinfo_Integer_1=1856695899;info_Integer_2=-1228532925,-1558813844;info_Integer_A=-1669649672;info_Integer_R=-2009180694,-858629871;info_Integer_G=-1571758790,-1808158134,-952576567;info_Integer_.=-839104859,-1425897419,1479780909;info_Float_1=86.42102;info_Float_2=47.610138,-21.43116;info_Float_A=22.063469;info_Float_R=31.69635,81.844086;info_Float_G=-0.43652344,34.970734,6.893921;info_Float_.=-70.91541,88.53403,11.178253,-25.09742;info_Character_1=w;info_Character_2=K,O;info_Character_A=R;info_Character_R=q,B;info_Character_G=P,P,[;info_Character_.=S,h,I;info_String_1=Z^Mr\\;info_String_2=Pozxs,[sGNN;info_String_A=uycmn;info_String_R=jXNUP,kaQaF;info_String_G=rhEZa,IB_Tj,XJMdW;info_String_.=MRIyw\tformat_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.\t-1127684345:-447879000,-1851298129:1367475938:1988967275,-439362504:-447904683,-1278415490,-1965808177:-1579759849,-1859788951:-31.910011:6.785797,-95.413086:19.286415:9.942863,-23.623634:31.06224,42.57071,92.734314:-51.402973,-25.126984,73.030045:J:K,Y:K:c,^:y,P,V:S,f,s:UwMXI:KZ]ZM,Dszwa:pRf\\B:VTcUU,OJHFc:gkajf,Ijt]R,ZCsdT:RPBHo,RSsDF,^wkt`,MKboQ\t-17276091:-1084847601,-962513528:1986061217:1121398096,539835016:398616881,544166366,-1410097944:1412514655:18.60447:86.08472,21.346832:12.448051:82.72441,61.860733:-29.544258,33.810944,17.074417:59.14827,-3.4019012:B:k,S:Y:K,b:I,J,\\:Q,x,t,j:ZBq\\n:QWJTe,YEfMY:iUtHu:jPdNd,e\\EWF:fR`mi,g_`q_,CHPa^:kUBhP,n\\^lc,zNX^D,Bir[s\t1513956177:842026642,704497487:1383022793:-798417759,1694579519:122079663,1485654311,1917949394:-1493128866,-669588433,-1130379312,-279135289:6.5678864:-39.898037,40.419724:-27.728035:27.380325,59.674316:-78.01762,-46.58332,82.15553:-9.7939,3.7038345,-10.703232:M:`,V:M:m,[:m,p,f:T,a,W:j\\c`P:mbjEz,GJcxU:gIaec:]IAHY,eyWTl:fKaF],eJYeM,oCeFY:lwAXv,GHCdL,yzGwQ,fB_YF\nNC_016845.1\t1745241132\t.\tc\t.\t178\tfilter_1\tinfo_Integer_1=1255642541;info_Integer_2=-495098950,-163997913;info_Integer_A=1186186199;info_Integer_R=-572056065,-813811802;info_Integer_G=1802548524,-1453024998,1578487959;info_Integer_.=135778623,772025838,1954620613,662130014;info_Float_1=-38.989304;info_Float_2=89.58476,9.665085;info_Float_A=90.821075;info_Float_R=-69.59674,45.46039;info_Float_G=-22.850967,40.54007,-72.7124;info_Float_.=85.93584,22.705269,-53.66709,-12.823059;info_Flag_0;info_Character_1=`;info_Character_2=_,e;info_Character_A=k;info_Character_R=u,a;info_Character_G=I,q,f;info_Character_.=p;info_String_1=mSQrS;info_String_2=lEvUS,zv[RB;info_String_A=\\lKRx;info_String_R=HP`gj,ftMtl;info_String_G=uGX\\V,D^xYp,\\]fYN;info_String_.=hvzef,hZ_x]\tformat_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.\t-1818220937:-1716370196,1471873252:1693568706:-1384353442,-531174370:290881760,-1611919520,-30861859:1820031599,1223028519,-241837959,-130516933:97.7139:-71.66517,-12.71994:-10.58564:25.678925,-49.043964:45.110657,-93.51485,27.87349:98.51344,89.07881,67.16724:L:z,B:d:r,m:^,r,Q:C,M:\\wEaP:Iq`oJ,tZfNB:cc[uV:_`pF`,wSolG:OBI\\a,L`htu,t^OEb:yBTff,Eql_^\t-519243625:1394920398,970547643:-1339598250:2069050503,1673139727:-383366852,819217239,-1582651946:394275479,1405093414:-88.27617:48.00322,-98.616264:-35.968445:-16.261383,64.94083:23.264336,55.233124,-12.705566:-31.018303,32.29892,88.60808:c:U,s:k:k,h:H,V,V:X,f,N:S^fl[:q[ODL,Tk[Kk:XkZnS:kkStd,H_Ns[:RRu[f,wNYIM,Skoky:KIrEa,Xxim^,nfhEz,mMSVs\t711611455:-246472977,1741690154:1085519110:-567213617,-400517020:1344286726,-1099251448,-600330030:-1239546736,247739344:4.053589:-22.763847,15.909172:47.2063:-13.576843,27.678421:8.278969,-50.282,57.042984:-70.92239,25.547836:e:s,u:c:j,A:E,],^:Q,V,t:WSfOh:IoRqu,HnRI`:MpI_G:XQKWn,sVHVf:s[UFc,tFryv,aJzJt:XVSVn,eOcXy\n";

    #[test]
    fn default() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Vcf::default();

        generator.header(&mut output, &mut rng)?;
        generator.records(&mut output, &mut rng, 5)?;

        assert_eq!(output, DEFAULT);

        Ok(())
    }

    #[test]
    fn create() -> error::Result<()> {
        let mut rng = crate::rand();

        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let temp_file = temp_path.join("tmp.vcf");

        let generator = Vcf::builder().build()?;

        generator.create(&temp_file, &mut rng, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, DEFAULT.to_vec());

        Ok(())
    }
}
