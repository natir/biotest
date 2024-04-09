//! VCF generation

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
    #[builder(default = "header::Header::builder().build().unwrap()")]
    header: header::Header,

    /// Structure to define record
    #[builder(default = "record::Record::builder().build().unwrap()")]
    record: record::Record,
}

impl Vcf {
    /// Create a VcfBuilder
    pub fn builder() -> VcfBuilder {
        VcfBuilder::default()
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

    const DEFAULT: &[u8] = b"##fileformat=VCFv4.3
##contig=<ID=chr1,length=4294967295,species=\"random\">
##contig=<ID=23,length=4294967295,species=\"random\">
##contig=<ID=93,length=4294967295,species=\"random\">
##contig=<ID=chrMT,length=4294967295,species=\"random\">
##contig=<ID=X,length=4294967295,species=\"random\">
##contig=<ID=NC_000015.10,length=4294967295,species=\"random\">
##contig=<ID=ENA|LT795502|LT795502.1,length=4294967295,species=\"random\">
##contig=<ID=NC_016845.1,length=4294967295,species=\"random\">
##contig=<ID=YAR028W,length=4294967295,species=\"random\">
##contig=<ID=1,length=4294967295,species=\"random\">
##FILTER=<ID=filter_0,Description=\"generated vcf filter field\">
##FILTER=<ID=filter_1,Description=\"generated vcf filter field\">
##FILTER=<ID=filter_2,Description=\"generated vcf filter field\">
##INFO=<ID=info_Integer_1,Number=1,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_2,Number=2,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_A,Number=A,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_R,Number=R,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_G,Number=G,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Integer_.,Number=.,Type=Integer,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_1,Number=1,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_2,Number=2,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_A,Number=A,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_R,Number=R,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_G,Number=G,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Float_.,Number=.,Type=Float,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Flag_0,Number=0,Description=\"generated vcf info field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_1,Number=1,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_2,Number=2,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_A,Number=A,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_R,Number=R,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_G,Number=G,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_Character_.,Number=.,Type=Character,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_1,Number=1,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_2,Number=2,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_A,Number=A,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_R,Number=R,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_G,Number=G,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##INFO=<ID=info_String_.,Number=.,Type=String,Description=\"generated vcf format field\",Source=\"biotest\",Version=\"0.1.0\">
##FORMAT=<ID=format_Integer_1,Number=1,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_2,Number=2,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_A,Number=A,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_R,Number=R,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_G,Number=G,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Integer_.,Number=.,Type=Integer,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_1,Number=1,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_2,Number=2,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_A,Number=A,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_R,Number=R,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_G,Number=G,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Float_.,Number=.,Type=Float,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_1,Number=1,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_2,Number=2,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_A,Number=A,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_R,Number=R,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_G,Number=G,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_Character_.,Number=.,Type=Character,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_1,Number=1,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_2,Number=2,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_A,Number=A,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_R,Number=R,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_G,Number=G,Type=String,Description=\"generated vcf format field\">
##FORMAT=<ID=format_String_.,Number=.,Type=String,Description=\"generated vcf format field\">
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO	FORMAT	sample_0	sample_1	sample_2
YAR028W	509242864	.	A	.	224	filter_0	info_Integer_1=-1867486109;info_Integer_2=1180908492,1041698939;info_Integer_A=-207506017;info_Integer_R=-1221871790,-1356802783;info_Integer_G=-496257857,2127853583,-1498117423;info_Integer_.=2082620030,-344161843,-1022296784,-1007334138;info_Float_1=68.286865;info_Float_2=-96.154594,-23.433853;info_Float_A=-48.782158;info_Float_R=-46.15216,-92.639305;info_Float_G=-7.5115204,74.78337,1.5983124;info_Float_.=26.825455;info_Flag_0;info_Character_1=b;info_Character_2=L,^;info_Character_A=4;info_Character_R=&,a;info_Character_G=N,k,%;info_Character_.={;info_String_1=nSOJk4@lC,;info_String_2=jS/\\D&BI|t,Y!R:7saso?;info_String_A=d!\"JhX)qQp;info_String_R=LD?P=?w~A),5[[@lIC.kc;info_String_G=3/bSljA-eF,F9c\"303:t],TqU?Kssw+$;info_String_.=[MEs+_JX%Y	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	1331697702:-73747613,1645597043:-1553292372:-1685240233,-1820034465:300184414,394747854,1197504288:-512239285,-1414044731:-26.444412:12.577988,-87.76228:-3.4822464:-95.66553,55.56636:-43.384956,-35.16729,6.755356:-9.445259,-43.99848,-94.4432,-92.06316:_:{,z:F:H,i:T,6,j:[,+:2;2l`>IYzJ:v7\"4m{_>h~,<G_oKV#{ze:!)!m\'S9/0_:dJ?2UvwuUy,^7GYW`=N;7:|)8^vf>dg#,1c`ok8Kh$S,_+GbKm=pyh:b:}\"s1f#s/	-1699207592:-1247215950,-1253877200:-1343277579:188169583,-1589761063:-532454402,989628108,295511500:1300868485:-14.547348:10.005661,82.95245:46.642517:90.124435,12.111877:69.43762,-11.427376,58.87137:-3.6344528,56.566788:e:),I:j:t,i:U,&,v:m,<,_,/:4y*`HYz#-o:9zE|;./\"-M,;<;|nStAje:SF>o/R,iE#:/,\"$RTVc(7,*(Sn6>ZQPD:ho#H0_<Tl9,PWg*UP~Esp,,{OVYEkbvA:.?sf#3gn*R,]yhrn0?zU8,N0+\"VIx*d-	-1697963895:1138852593,181408155:-317412374:-1000659906,1329247534:628009109,-1501500099,-1741170910:-1510591037,-995737568,2069116675,-1117969497:25.04361:84.79895,44.54808:36.19725:-48.734688,-33.58867:-54.331757,84.5206,69.88823:87.006195:?:E,t:,:?,B:b,$,s:?,4,%,}:Pm=/<N[3&;:=H>}~CRs}y,o^H0T~a31`:EJblc1Z!bn:tm9=sB<\":\\,7oU=q*dDU(:-k5AqYk|~^,o\"SF3e.$lt,R9J~QqXY_R:jI\\,>VLD1@,O#Bvp?rT;+,+]M$Tdqh7g,Qa;ou#<4,C
NC_000015.10	1990771979	.	g	.	76	.	info_Integer_1=985125610;info_Integer_2=-2076159385,2095020886;info_Integer_A=-1486927124;info_Integer_R=-1366888910,1109498274;info_Integer_G=-1220195155,375528302,622622151;info_Integer_.=-1851270635,-1683478441,-1058618036;info_Float_1=36.333466;info_Float_2=57.98674,-34.06732;info_Float_A=73.12625;info_Float_R=-68.685196,-28.807327;info_Float_G=-97.25389,-83.217316,53.371017;info_Float_.=-59.54485;info_Character_1=t;info_Character_2=;,/;info_Character_A=z;info_Character_R=o,_;info_Character_G=P,P,z;info_Character_.=:;info_String_1=76\'~ydkEh{;info_String_2=Xl#hA?\")Rw,,l=ox~|&ia;info_String_A=G,gM=1K57g;info_String_R=d`(@9@akA8,:h_D#Nc;Wl;info_String_G={HXu,f7kdJ,UO;dRyY_@8,Mh3r0.9$4Y;info_String_.=5d#lUKvy]|,%Z,&HF$q.},GKTV\"\'>Dya,lKw0U`lz9V	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	776188052:-1578532767,-1302325449:52657598:-2135958655,-1135523634:1449106916,1753648080,1075637484:676542150,1902606622,2019226502,-439308040:50.887344:94.68472,-16.335388:-91.662834:66.09871,58.31633:32.06732,-26.503494,-79.61113:-88.184525,98.6815:q:g,C:k:g,4:T,l,n:s,N,K,/:#f/x`\\2u\"s:Z;*F0(P9Ui,?\';EKG-mgV:9Ry@]wiwe&:/{.Gkl(;rC,-%m53k:R`#:dF.Teg4k(j,jo)5g#q}+{,JcAX#{yXK9:A(6mOH<hSn,!E|.b$yN_n,RKrO6xaHM?,i|k.s8v,eo	-1588922597:1583267046,831043256:-1845708418:-906269054,-1882153225:491525986,340896502,-594200150:1373957466,1343650241,-1818387709,771445599:-56.410862:-26.67012,32.126236:-42.344856:45.14885,-23.34468:-91.05513,-93.02018,33.876923:34.392242:-:q,Q:U:!,_:@,P,r:b,x,5:+$=-(;Q=px:fZv`SDm.yU,Dby17::KR?:`/IP4ql~rK:r6v}ij}G7B,9eVU*qa^(U:RcG0,5\'ZEI,%5/|7F\'{FF,4<-\'AS#YT^:Dr12Qc:?CW,>^rDy5/NJ\',ryU[[n^]MC	-737734461:-1039503775,-224718148:395331087:-609396723,-643192338:-1099264898,-1414310378,-1613799467:409401431:-45.04497:31.736618,45.791763:11.747528:43.78209,-20.865295:30.616302,-71.162605,42.38507:-13.728645,-92.6682:r:[,@:t:l,*:<,9,#:l,r,\':)PyfWkO8{h:X10^xZmBXN,e>H2W.k0Mo:<zuJ\"o>M<k:<0A\\I5IbB-,u9Z6Z\\M(E*:]ThbTo$-UQ,ofB#a9jMQS,grY}7F\'#qL:qb_n>uRpJ|,Ol/A7S=l+:,vEQK5SD5iL,i)Fn]ZdXT:
NC_016845.1	161805126	.	C	.	0	filter_0	info_Integer_1=2025181494;info_Integer_2=-457162078,-677257609;info_Integer_A=1064522814;info_Integer_R=655262959,-1345573638;info_Integer_G=244104587,-1751589973,-33143890;info_Integer_.=-1440412853,-331423089,568438530;info_Float_1=-31.163742;info_Float_2=-57.342316,59.797592;info_Float_A=-0.16572571;info_Float_R=-90.35363,25.20082;info_Float_G=-82.30049,85.982254,-14.1403885;info_Float_.=49.404236,88.34059,-99.591736,-19.608734;info_Flag_0;info_Character_1=,;info_Character_2=%,Z;info_Character_A=3;info_Character_R=|,};info_Character_G=,,y,<;info_Character_.=R;info_String_1=I*gmU*f%z/;info_String_2=H-On}TkELC,>w0ri^=zTz;info_String_A=/eEcrxZ7FT;info_String_R=\\fuU.o&mik,><qfh(wB?~;info_String_G=xL=2<z,>*:,_]5tgv+GDN,&PmOH6Ba}[;info_String_.={O(*pu1DV,,OGpJM.JK\\e,$]`H]X~o4~	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	-2031554200:465234222,1514255955:1150635418:66804676,1494669369:-948127707,-773241792,-1952938182:285409541:-45.340466:-71.99507,65.908295:9.153915:60.76036,-67.940445:-17.430664,78.5948,-11.59816:17.45729:X:L,u:C:n,*:T,?,l:8:#/MV3Sat`u:t)Q(1^:\"@],\'(ogRPDne2:}tG-?X&f!?:HnJ`VA_yBe,eCC|uQF]?Q:^gLpL8&`fK,2fFfJjfat[,-7sL<Ly7I.:keo1(FciPk,]a(~i5>Cr_	-246472977:1741690154,1085519110:-567213617:-400517020,1344286726:-1099251448,-600330030,-954898033:87050511,-488849812,341647170:47.2063:-13.576843,27.678421:8.278969:-50.282,57.042984:-28.196312,-70.92239,25.547836:79.772995,19.348503,-7.1992874,44.35666:\":N,Q:X:;,D:t,?,]:6,o,u,,:=.m/R+G1Er:C-D^sLB*Yu,*q|xV~0t_::GC?j8YG|B):}h>X4K*.aM,DSRm~Z?#2o:@=&8uI2%*b,CYdPF`%psa,Hb_#E*#\"$3:jPX1c/$CT6,s$$<(x_k)@	1779253445:-2008333700,1469319739:1386427454:40699918,-1626132079:266038065,1328345211,1048991265:1788306263,396523608:47.059006:7.5892487,36.29811:18.503044:59.088684,84.92091:87.00018,42.422363,88.077545:22.039795,-20.849182,63.43016,-62.645317:S:$,O:r:g,#:L,<,B:t,k,[:5[?bN4K;QM:E|\\QN{;4T7,M.+e_*Wha]:8!6k?>)t,8:7iX\\q}q#dv,\"~GXa>*V</:5&Qr{]GwT1,2^5pr?V}B4,k46wBdIg2::\'Ia/M3\"Sbh,MC>-nVxp-C,k#bh1:RmsT,tr])/6I0,/
93	1628481200	.	g	.	243	.	info_Integer_1=-1745451465;info_Integer_2=-1416442745,1351086165;info_Integer_A=1149927617;info_Integer_R=-1257660465,570311405;info_Integer_G=-235852620,248766754,-897953129;info_Integer_.=-282537957,-648221198;info_Float_1=-13.898972;info_Float_2=69.033844,-81.27475;info_Float_A=-75.5985;info_Float_R=-30.964996,-65.281845;info_Float_G=45.846268,-76.65327,-3.310463;info_Float_.=-50.443413;info_Character_1=@;info_Character_2=G,j;info_Character_A=~;info_Character_R=0,q;info_Character_G=/,t,S;info_Character_.=G,y,a,];info_String_1=cH\"Mt+!DW7;info_String_2=m>WYSDke/$,V6GA=($ETH;info_String_A=PKC>kG\\k4G;info_String_R=6o?URXH\'iP,;Iq*U.qFP/;info_String_G=!|${+J)km6,s8l=qoJY*l,_%8P$8.LgD;info_String_.=0b|DhucyPM	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	2126175016:1129636117,-2081418625:-234859576:812882666,857277679:-2041946338,-2042220346,-1448296019:-207192413,-575858759,361961114,1833951153:11.249496:-53.691505,-56.08103:51.13185:50.876236,7.7953796:91.50241,55.230377,28.686188:56.792877,-72.2456,-41.596485,-67.26294:^:T,d:~:k,|:b,K,~:-,s,R,i:p-\'~#SArsU:mrZR2hqbm\',caSQKrBat\":<[atyOUHv<:Z0#^\'G}!aH,\'PaGe>zRWT:y5,irDj8bn,JQ5#m^B}VK,]\\Ro!?,H-X:joQ[_8K?5w,Pwh8$m\\G=&,\"/[QE%X~ei	326698944:1667434880,1502079066:-1796451115:-434319537,-693963678:-1428668240,-1567382331,1120991771:61362482,902786551,-229680185:42.882538:-37.351513,-13.369278:96.234604:-57.21159,69.30682:-44.12949,41.216522,-10.266998:-60.65533:g:a,k:(:\\,b:`,Q,7:5,;:+>PIuzu-_i:RLP,KRY69w,\'iXG~{U=g@:MB-{4AL20+:QsU|rsc|{-,2i|QeM-5Vp:R#4B*M!\'tY,)0l-=>mPI},[j^RcKOVi,:2O<:dz&%=],fH45+,xhyq,1\\\\vh]r<J\'	1419940183:-1548885431,1188147259:-1061203998:1600049196,-4212261:479487908,570443195,1624025678:-483195379:92.25282:-51.60575,-44.06626:-95.549416:-63.43329,75.52333:89.147766,62.908432,35.18953:-28.882408,68.80641,6.029129,-47.455452:c:O,z:^:Y,\":D,Z,>:%,e:0LQfZXW\\&|:1F\"Mac]*M},tl\\Q(+\'c!l:.4._EWsV48:0d~35QC?P%,b{sE1`)}J\\:$B#6Zh80;!,=7@3M\'up79,&\\2QIy@r\\d:v#k#[w/k~k,rBiNb2w7E3,nUsxH~g&t3
chr1	54211728	.	T	.	106	filter_0	info_Integer_1=1869758145;info_Integer_2=-1415931557,-1291733190;info_Integer_A=1197283695;info_Integer_R=-99011153,184993022;info_Integer_G=-304006920,107027867,1129332211;info_Integer_.=-743591286,1298751868,-1870036205;info_Float_1=-15.466354;info_Float_2=13.643524,31.201965;info_Float_A=-83.97765;info_Float_R=87.02226,45.305634;info_Float_G=66.3873,-8.753494,-24.258568;info_Float_.=-49.719143,4.4375916,-30.54676,-71.69261;info_Character_1=];info_Character_2=3,X;info_Character_A=?;info_Character_R=5,w;info_Character_G=y,f,Y;info_Character_.=6,h,l;info_String_1=:rpZGtX=cz;info_String_2=|L9vu,vSS0,ejWTi\\b*=);info_String_A=nEIh3H8zPu;info_String_R=d>uvMW\\BUc,])jO/rmL\'q;info_String_G=9JpHhY;r3\",#[[]ro.^+2,RU!msic)?-;info_String_.=7*9[Wp{,Zv,f3:i[eyT|#,aT9x>/Dl6z	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	-137429:-487188436,-696650746:-100643027:-1565615532,889622702:1413417352,1177023616,-753719288:-1842319770,490162555:57.16078:-57.2166,-34.35414:3.175949:90.32216,25.335693:-79.8594,-14.332291,-70.959496:3.832718:6:7,@:x:~,<:),>,g:F:Yj4SNTN@u_:\\$/G)|ce_=,#Fg(}a*&Rj:xkJf>]vdHG:SJqIu0tpm{,>u/X+j3d?L:\\u<L,h(+Kq,Q)Yi[KbGXM,2Ic4dNn8:d:j/?%_r(u%O,V+PcIWwrVj,0i0wT,c~s3,1tL{D;qQPc	-2101103516:1188925016,-942868546:272539360:-1647165337,-667287924:2042398819,-1530689155,2044961168:2133315971,1067580168:24.64869:-18.59977,-77.258606:7.3624115:84.90216,-50.00353:-94.99128,58.709763,-99.85483:-7.3968887,69.65532,18.196487,51.884888:>:t,4:}:9,B:v,P,5:?,j,<:/W)#\'O</vR:Ve%KtPW>8a,+?K:JBJu?;:v0JyHoR?|&:)SLKg@t{D7,i*6AC4fqFW:#:\\l#fHcST,}_/\\wMOWV*,79Z0g+okE|:A.}jkn&*v?,shPW,{\'.h~	1079991824:-519580031,1032149541:1419545689:961959574,190899660:-1627312707,-1641157633,-100165881:1893933525,1446753029,-761397440:50.09677:-78.02365,71.46156:56.636353:81.90201,-30.47879:-37.61213,98.139435,31.63826:95.535065,20.02246:=:>,d:g:u,c:f,;,@:E,V,N:4|Zdg|Fm5h:AH]$Z,mbN8,;nHC5v&cl$:\'FAw\"KX74/:8/>e6TA&9r,^]Nv/jaalR:W(?v[m*Ri+,Q1k+H)]tuC,E)Y8!wx/Zb:%j=j0`x[A6
";
    #[test]
    fn default() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Vcf::builder().build().unwrap();

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

        let generator = Vcf::builder().build().unwrap();

        generator.create(&temp_file, &mut rng, 5)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, DEFAULT.to_vec());

        Ok(())
    }
}
