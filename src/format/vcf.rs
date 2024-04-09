//! VCF generation

/* std use */

/* crates use */
use rand::seq::SliceRandom as _;
use rand::Rng as _;

/* projet use */
use crate::constants;
use crate::error;

/* module declaration */
pub mod header;

/// Generate prefix of vcf record
fn prefix_record<W>(output: &mut W, rng: &mut rand::rngs::StdRng) -> error::Result<()>
where
    W: std::io::Write,
{
    // chromosomes
    output.write_all(constants::CHROMOSOMES.choose(rng).unwrap())?;
    output.write_all(b"\t")?;

    // position
    output.write_all(
        &rng.gen_range(0..i32::MAX)
            .to_string()
            .bytes()
            .collect::<Vec<u8>>(),
    )?;

    // identifiant
    output.write_all(b"\t.\t")?;

    // reference
    output.write_all(&[*constants::NUCLEOTIDES.choose(rng).unwrap()])?;
    output.write_all(b"\t")?;

    // alternatif
    let alt_len = rng.gen_range(1..5);
    crate::sequence(output, rng, alt_len)?;
    output.write_all(b"\t")?;

    // quality
    output.write_all(
        &rng.gen_range(0..255)
            .to_string()
            .bytes()
            .collect::<Vec<u8>>(),
    )?;
    output.write_all(b"\t")?;

    Ok(())
}

/// Generate filter
fn filter<W>(output: &mut W, rng: &mut rand::rngs::StdRng, nb_filters: u8) -> error::Result<()>
where
    W: std::io::Write,
{
    if nb_filters == 0 || rng.gen_bool(1.0 / nb_filters as f64) {
        output.write_all(b".")?;
    } else {
        output.write_all(b"Filter_")?;
        output.write_all(rng.gen_range(0..nb_filters).to_string().as_bytes())?;
    }

    Ok(())
}

/// Generate integer in VCF range
fn generate_integer(rng: &mut rand::rngs::StdRng) -> i32 {
    rng.gen_range((i32::MIN + 7)..i32::MAX)
}

/// Generate float in VCF range
fn generate_float(rng: &mut rand::rngs::StdRng) -> f32 {
    rng.gen_range(-100.0..100.0)
}

/// Generate a string
fn generate_string(rng: &mut rand::rngs::StdRng, length: usize) -> error::Result<Vec<u8>> {
    (0..length)
        .map(|_| {
            constants::ASCII_VISIBLE
                .choose(rng)
                .cloned()
                .ok_or(create_unreachable!())
        })
        .collect::<error::Result<Vec<u8>>>()
}

fn generate_value(
    rng: &mut rand::rngs::StdRng,
    vcf_type: &[u8],
    vcf_number: &[u8],
    nb_samples: u8,
) -> error::Result<Vec<u8>> {
    match vcf_type {
        b"Integer" => match vcf_number {
            b"1" | b"A" => Ok(generate_integer(rng).to_string().as_bytes().to_vec()),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(generate_integer(rng).to_string().as_bytes());
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(generate_integer(rng).to_string().as_bytes());
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(generate_integer(rng).to_string().as_bytes());
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            _ => Err(create_unreachable!()),
        },
        b"Float" => match vcf_number {
            b"1" | b"A" => Ok(generate_float(rng).to_string().as_bytes().to_vec()),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(generate_float(rng).to_string().as_bytes());
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(generate_float(rng).to_string().as_bytes());
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(generate_float(rng).to_string().as_bytes());
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            _ => Err(create_unreachable!()),
        },
        b"Character" => match vcf_number {
            b"1" | b"A" => Ok(vec![constants::ASCII_VISIBLE
                .choose(rng)
                .cloned()
                .ok_or(create_unreachable!())?]),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.push(
                        constants::ASCII_VISIBLE
                            .choose(rng)
                            .cloned()
                            .ok_or(create_unreachable!())?,
                    );
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.push(
                        constants::ASCII_VISIBLE
                            .choose(rng)
                            .cloned()
                            .ok_or(create_unreachable!())?,
                    );
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.push(
                        constants::ASCII_VISIBLE
                            .choose(rng)
                            .cloned()
                            .ok_or(create_unreachable!())?,
                    );
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            _ => Err(create_unreachable!()),
        },
        b"String" => match vcf_number {
            b"1" | b"A" => generate_string(rng, 10),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(&generate_string(rng, 10)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(&generate_string(rng, 10)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(&generate_string(rng, 10)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            _ => Err(create_unreachable!()),
        },
        _ => Err(create_unreachable!()),
    }
}

/// Generate info
fn info<W>(output: &mut W, rng: &mut rand::rngs::StdRng, nb_samples: u8) -> error::Result<()>
where
    W: std::io::Write,
{
    for vcf_type in constants::VCF_INFO_TYPE {
        if vcf_type == b"Flag" {
            if rng.gen_bool(0.5) {
                output.write_all(b"Flag_0")?;
            }
        } else {
            for vcf_number in constants::VCF_INFO_NUMBER {
                output.write_all(b"info_")?;
                output.write_all(vcf_type)?;
                output.write_all(b"_")?;
                output.write_all(vcf_number)?;
                output.write_all(b"=")?;
                output.write_all(&generate_value(rng, vcf_type, vcf_number, nb_samples)?)?;

                if Some(&vcf_number) != constants::VCF_INFO_NUMBER.last()
                    || Some(&vcf_type) != constants::VCF_INFO_TYPE.last()
                {
                    output.write_all(b";")?;
                }
            }
        }
    }

    Ok(())
}

fn format<W>(output: &mut W) -> error::Result<()>
where
    W: std::io::Write,
{
    for vcf_type in constants::VCF_FORMAT_TYPE {
        for vcf_number in constants::VCF_FORMAT_NUMBER {
            output.write_all(b"format_")?;
            output.write_all(vcf_type)?;
            output.write_all(b"_")?;
            output.write_all(vcf_number)?;

            if Some(&vcf_number) != constants::VCF_INFO_NUMBER.last()
                || Some(&vcf_type) != constants::VCF_INFO_TYPE.last()
            {
                output.write_all(b":")?;
            }
        }
    }

    Ok(())
}

fn genotype<W>(output: &mut W, rng: &mut rand::rngs::StdRng, nb_samples: u8) -> error::Result<()>
where
    W: std::io::Write,
{
    for vcf_type in constants::VCF_FORMAT_TYPE {
        for vcf_number in constants::VCF_FORMAT_NUMBER {
            output.write_all(&generate_value(rng, vcf_type, vcf_number, nb_samples)?)?;
            if Some(&vcf_number) != constants::VCF_INFO_NUMBER.last()
                || Some(&vcf_type) != constants::VCF_INFO_TYPE.last()
            {
                output.write_all(b":")?;
            }
        }
    }

    Ok(())
}

/// Generate vcf record
fn record<W>(
    output: &mut W,
    rng: &mut rand::rngs::StdRng,
    nb_filters: Option<u8>,
    add_info: bool,
    nb_samples: Option<u8>,
) -> error::Result<()>
where
    W: std::io::Write,
{
    prefix_record(output, rng)?;

    // filter
    if let Some(n) = nb_filters {
        filter(output, rng, n)?;
    } else {
        output.write_all(b".")?;
    }

    // info
    if add_info {
        output.write_all(b"\t")?;
        info(output, rng, nb_samples.unwrap_or(0))?;
    }

    // format
    if let Some(n) = nb_samples {
        output.write_all(b"\t")?;
        format(output)?;

        for i in 0..n {
            genotype(output, rng, n)?;
            if i != n - 1 {
                output.write_all(b"\t")?;
            }
        }
    }

    Ok(())
}

/// Write multiple record
pub fn records<W>(
    output: &mut W,
    rng: &mut rand::rngs::StdRng,
    num_record: usize,
    nb_filters: Option<u8>,
    add_info: bool,
    nb_samples: Option<u8>,
) -> error::Result<()>
where
    W: std::io::Write,
{
    for _ in 0..num_record {
        record(output, rng, nb_filters, add_info, nb_samples)?;
        output.write_all(&[b'\n'])?;
    }

    Ok(())
}

/// Create a vcf file
pub fn create<P>(
    path: P,
    rng: &mut rand::rngs::StdRng,
    num_record: usize,
    nb_filters: Option<u8>,
    add_info: bool,
    nb_samples: Option<u8>,
) -> error::Result<()>
where
    P: std::convert::AsRef<std::path::Path>,
{
    let mut output = std::fs::File::create(&path)?;

    header::write(&mut output, nb_filters, add_info, nb_samples)?;
    records(
        &mut output,
        rng,
        num_record,
        nb_filters,
        add_info,
        nb_samples,
    )?;

    Ok(())
}

#[cfg(test)]
mod tests {
    /* std use */
    use std::io::Read as _;

    /* project use */
    use super::*;

    const TRUTH: &[u8] = b"##fileformat=VCFv4.3
##contig=<ID=chr1,length=2147483648,species=\"random\">
##contig=<ID=23,length=2147483648,species=\"random\">
##contig=<ID=93,length=2147483648,species=\"random\">
##contig=<ID=chrMT,length=2147483648,species=\"random\">
##contig=<ID=X,length=2147483648,species=\"random\">
##contig=<ID=NC_000015.10,length=2147483648,species=\"random\">
##contig=<ID=ENA|LT795502|LT795502.1,length=2147483648,species=\"random\">
##contig=<ID=NC_016845.1,length=2147483648,species=\"random\">
##contig=<ID=YAR028W,length=2147483648,species=\"random\">
##contig=<ID=1,length=2147483648,species=\"random\">
#CHROM	POS	ID	REF	ALT	QUAL	FILTER	INFO
YAR028W	509242864	.	a	ATg	6	.
chrMT	1592900903	.	A	CGCg	200	.
X	562593432	.	T	G	65	.
chr1	993087666	.	a	cAcg	122	.
1	1705884896	.	A	tGc	72	.
";

    const TRUTH_FILTER: &[u8] = b"YAR028W	509242864	.	a	ATg	6	Filter_0
NC_016845.1	969988815	.	C	GTTA	97	.
chr1	993087666	.	a	cAcg	122	Filter_0
23	2071684354	.	a	cT	204	.
23	1676586806	.	a	AG	248	Filter_1
";

    const TRUTH_INFO: &[u8] = b"YAR028W	509242864	.	a	ATg	6	Filter_0	info_Integer_1=-1867486102;info_Integer_2=1180908493,1041698941;info_Integer_A=-207506013;info_Integer_R=-1221871784,-1356802777;info_Integer_G=;info_Integer_.=2082620030,-344161839,-1022296779,-1007334133;info_Float_1=68.286865;info_Float_2=-96.154594,-23.433853;info_Float_A=-48.782158;info_Float_R=-46.15216,-92.639305;info_Float_G=;info_Float_.=1.5983124,-8.867523,77.741455,-86.29277;Flag_0info_Character_1=M;info_Character_2=i,r;info_Character_A=[;info_Character_R=g,M;info_Character_G=;info_Character_.=h;info_String_1=w\\voCGp]Zo;info_String_2=XMTgQouVGn,`JweVDDUYy;info_String_A=tzAny[_POs;info_String_R=hsqbSjAdbZ,cRFrrQ_[VS;info_String_G=;info_String_.=RtSvzMeTjo,nYVInjLIi`,oWogntTH\\Q
X	550245579	.	A	aaa	81	Filter_1	info_Integer_1=1609643078;info_Integer_2=1793595268,-322605342;info_Integer_A=-1688173181;info_Integer_R=-1978954124,-1707772923;info_Integer_G=;info_Integer_.=511785259,-125870099;info_Float_1=-23.018097;info_Float_2=46.32849,5.9644012;info_Float_A=74.879486;info_Float_R=-78.288414,-87.38592;info_Float_G=;info_Float_.=-90.66701,19.76609,62.01201;info_Character_1=I;info_Character_2=G,E;info_Character_A=b;info_Character_R=c,n;info_Character_G=;info_Character_.=W,J;info_String_1=VaDBnQSHYN;info_String_2=m[QBCgL`Sc,xxXYm`NnOG;info_String_A=[K`QKgogYx;info_String_R=ZuNAMyDqpg,ZliSmUzRvG;info_String_G=;info_String_.=XBgqxa[aBw
NC_000015.10	954002188	.	A	t	17	.	info_Integer_1=-1505124378;info_Integer_2=495082976,-1432749489;info_Integer_A=713077112;info_Integer_R=947225490,-250366933;info_Integer_G=;info_Integer_.=-761568963;info_Float_1=78.51788;info_Float_2=-62.416267,11.148003;info_Float_A=82.33461;info_Float_R=83.84645,78.85599;info_Float_G=;info_Float_.=-70.75128,35.715073,88.21924;info_Character_1=r;info_Character_2=I,g;info_Character_A=N;info_Character_R=X,c;info_Character_G=;info_Character_.=h,R,d;info_String_1=]QWyFOgukS;info_String_2=jBlBKigqzn,OIm[gGXi[j;info_String_A=\\RlwOmAiZP;info_String_R=wyAsKBssXJ,GMMK`HVkgo;info_String_G=;info_String_.=vkxarZoyTn,QFEntKUmna,DWuppugILw
93	215948308	.	G	AG	34	Filter_0	info_Integer_1=1924651034;info_Integer_2=-473081454,2013874853;info_Integer_A=-937759504;info_Integer_R=-1524377977,-1501512929;info_Integer_G=;info_Integer_.=-1586974393;info_Float_1=80.58081;info_Float_2=-5.483078,-43.724228;info_Float_A=-41.917133;info_Float_R=65.731735,-44.021725;info_Float_G=;info_Float_.=7.376503,77.50084,-31.576347,56.736618;info_Character_1=`;info_Character_2=X,S;info_Character_A=q;info_Character_R=\\,I;info_Character_G=;info_Character_.=H,m,A;info_String_1=WXBIAyCL_`;info_String_2=ebjENFE`pN,SPd^wz^tZV;info_String_A=mq_oBYJgQ`;info_String_R=oPn^Z\\`bla,^yzIWtrgmo;info_String_G=;info_String_.=x]WbcWVPni
1	733873271	.	a	gACT	210	Filter_1	info_Integer_1=1879698450;info_Integer_2=1870114592,1109294152;info_Integer_A=-1647348160;info_Integer_R=1565404410,1392446649;info_Integer_G=;info_Integer_.=-769169762,1921023442;info_Float_1=-6.7819138;info_Float_2=11.500885,-47.349976;info_Float_A=86.69888;info_Float_R=-0.38061523,-49.81668;info_Float_G=;info_Float_.=-85.17905;info_Character_1=b;info_Character_2=Z,v;info_Character_A=x;info_Character_R=F,j;info_Character_G=;info_Character_.=m;info_String_1=`YPfIFpIPN;info_String_2=evhOTNvrCw,SEWbLtHSUi;info_String_A=CnsIsSMCBy;info_String_R=^pRIQ\\eLD],QRzYyzV_sz;info_String_G=;info_String_.=gYJ`TzLKhH,WZiobiKndA,PiptpPRUQy,BPeNqLaRrP
";

    const TRUTH_SAMPLES: &[u8] = b"YAR028W	509242864	.	a	ATg	6	.	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.-686621067:1038318162,-1892038410:-1867486102:1180908493,1041698941:-207506013:2082620030,-344161839,-1022296779,-1007334133:68.286865:-96.154594,-23.433853:-48.782158:-46.15216,-92.639305:-7.5115204:1.5983124,-8.867523,77.741455,-86.29277:e:L,M:i:r,[:g:h:w\\voCGp]Zo:XMTgQouVGn,`JweVDDUYy:tzAny[_POs:hsqbSjAdbZ,cRFrrQ_[VS:^RtSvzMeTj:nYVInjLIi`,oWogntTH\\Q,XXOiALJLnu,Ptf`Sr^aaS
X	1881454133	.	g	A	26	.	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.2139428471:993348561,-689633277:511785259:-125870099,-494309874:994897076:1608024901,-1681230419,-1876597949:32.911133:-11.732056,18.387054:-90.66701:19.76609,62.01201:-3.4341583:-84.75196:b:c,n:V:W,J:V:D,B,n:QSHYNm[QBC:gL`ScxxXYm,`NnOG[K`QK:gogYxZuNAM:yDqpgZliSm,UzRvGGXBgq:xa[aBw_Zxx:FA[o`OIdJg
NC_016845.1	948558355	.	A	ggg	141	.	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.-1519372091:766975666,1894493800:1136886858:1132040702,1543188175:-1513714723:328933865,746721907,-865393810:23.349594:-3.3177109,-43.643593:-21.609734:-51.812767,94.04256:-81.78232:44.08844,-94.0187:l:B,K:i:g,q:z:O,I,m,[:gGXi[j\\Rlw:OmAiZPwyAs,KBssXJGMMK:`HVkgoY`vk:xarZoyTnQF,EntKUmnaDW:uppugILwOF:RxBZHMqOWy,QIIAHu\\QQq,QyZ`tTnZk`
X	674622273	.	C	t	2	.	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.-500742528:-413948498,-2030006843:-1499608731:-1632847816,-2082516466:2048055005:-1289521416:4.3382645:9.063217,25.310585:-44.940376:14.317032,41.66844:-84.75847:-83.61602:`:p,N:S:P,d:^:t,Z,V:mq_oBYJgQ`:oPn^Z\\`bla,^yzIWtrgmo:Gx]WbcWVPn:iuT_IlSskB,LwLHlF_fww:GspJRS\\aPw:]JGEBbZvxF,jHGm`YPfIF
YAR028W	318446305	.	T	gA	238	.	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.-770454388:-1786087909,-486130761:313139324:-1328850310,1676663853:-1621135315:-625454964,841201143:-92.52112:57.182693,74.828125:-69.70703:72.491745,-35.8011:-58.521343:-93.79444:y:^,p:R:I,Q:\\:L,D,]:QRzYyzV_sz:wqgYJ`TzLK,hHWZiobiKn:dAPiptpPRU:QyBPeNqLaR,rPFJcjVaEr:HHloMTrcoG:iA_WILkXIc,`ot_PZwl^\\,Uz^rcVndZg,_IpyMneGSa
";

    #[test]
    fn record_() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        record(&mut output, &mut rng, None, false, None)?;

        assert_eq!(output, TRUTH[628..657]);

        Ok(())
    }

    #[test]
    fn record_filter() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        record(&mut output, &mut rng, Some(2), false, None)?;

        assert_eq!(output, TRUTH_FILTER[..36]);

        Ok(())
    }

    #[test]
    fn record_info() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        record(&mut output, &mut rng, Some(2), true, None)?;

        assert_eq!(output, TRUTH_INFO[..732]);

        Ok(())
    }

    #[test]
    fn record_sample() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        record(&mut output, &mut rng, None, false, Some(1))?;

        assert_eq!(output, TRUTH_SAMPLES[..807]);

        Ok(())
    }

    #[test]
    fn records_() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        records(&mut output, &mut rng, 5, None, false, None)?;

        assert_eq!(output, TRUTH[628..]);

        Ok(())
    }

    #[test]
    fn records_filter() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        records(&mut output, &mut rng, 5, Some(2), false, None)?;

        assert_eq!(output, TRUTH_FILTER);

        Ok(())
    }

    #[test]
    fn records_info() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        records(&mut output, &mut rng, 5, Some(2), true, None)?;

        assert_eq!(output, TRUTH_INFO);

        Ok(())
    }

    #[test]
    fn records_sample() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        records(&mut output, &mut rng, 5, None, false, Some(1))?;

        assert_eq!(output, TRUTH_SAMPLES);

        Ok(())
    }

    #[test]
    fn create_() -> error::Result<()> {
        let mut rng = crate::rand();

        let temp_dir = tempfile::tempdir()?;
        let temp_path = temp_dir.path();

        let temp_file = temp_path.join("tmp.vcf");

        create(&temp_file, &mut rng, 5, None, false, None)?;

        let mut data = Vec::new();
        let mut input = std::fs::File::open(&temp_file)?;
        input.read_to_end(&mut data)?;

        assert_eq!(data, TRUTH);

        Ok(())
    }
}
