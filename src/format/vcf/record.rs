//! VCF header generation

/* std use */

/* crates use */
use rand::seq::SliceRandom as _;
use rand::Rng as _;

/* projet use */
use crate::error;
use crate::values;

use crate::values::Generate as _;
use crate::values::Get as _;

#[derive(derive_builder::Builder)]
#[builder(pattern = "owned")]
/// Struct to generate record
pub struct Record {
    /// Value use for chromosomes
    #[builder(default = "values::Chromosomes::Default")]
    contigs: values::Chromosomes,

    /// Possible position
    #[builder(default = "values::Integer::Position")]
    position: values::Integer,

    /// Alphabet use to variant id
    #[builder(default = "values::Alphabet::VcfDefault")]
    id: values::Alphabet,

    /// Length of id
    #[builder(default = "1")]
    id_len: usize,

    /// Id prefix
    #[builder(default = "b\"\".to_vec()")]
    id_prefix: Vec<u8>,

    /// Id suffix
    #[builder(default = "b\"\".to_vec()")]
    id_suffix: Vec<u8>,

    /// Alphabet use to reference sequence
    #[builder(default = "values::Nucleotides::Dna")]
    reference: values::Nucleotides,

    /// Alphabet use to reference sequence
    #[builder(default = "1")]
    reference_len: usize,

    /// Alphabet use to alternative sequence
    #[builder(default = "values::Nucleotides::DnaUpper")]
    alternative: values::Nucleotides,

    /// Alphabet use to alternative sequence
    #[builder(default = "1")]
    alternative_len: usize,

    /// Quality range
    #[builder(default = "values::Integer::Quality")]
    quality: values::Integer,

    /// filter range
    #[builder(default = "values::Integer::UserDefine(0..3)")]
    filter: values::Integer,

    /// filter prefix
    #[builder(default = "b\"filter_\".to_vec()")]
    filter_prefix: Vec<u8>,

    /// info prefix
    #[builder(default = "b\"info_\".to_vec()")]
    info_prefix: Vec<u8>,

    /// InfoType
    #[builder(default = "values::VcfInfoType::All")]
    info_type: values::VcfInfoType,

    /// InfoNumber
    #[builder(default = "values::VcfInfoNumber::All")]
    info_number: values::VcfInfoNumber,

    /// format prefix
    #[builder(default = "b\"format_\".to_vec()")]
    format_prefix: Vec<u8>,

    /// FormatType
    #[builder(default = "values::VcfFormatType::All")]
    format_type: values::VcfFormatType,

    /// FormatNumber
    #[builder(default = "values::VcfFormatNumber::All")]
    format_number: values::VcfFormatNumber,

    /// Number of sample
    #[builder(default = "3")]
    sample: usize,
}

impl Record {
    /// Create a RecordBuilder
    pub fn builder() -> RecordBuilder {
        RecordBuilder::default()
    }

    fn format<W>(&self, output: &mut W) -> error::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        for vcf_type in self.format_type.as_ref() {
            for vcf_number in self.format_number.as_ref() {
                output.write_all(&self.format_prefix)?;
                output.write_all(vcf_type)?;
                output.write_all(b"_")?;
                output.write_all(vcf_number)?;

                if Some(vcf_number) != self.format_number.as_ref().last()
                    || Some(vcf_type) != self.format_type.as_ref().last()
                {
                    output.write_all(b":")?;
                }
            }
        }

        Ok(())
    }

    fn sample<W>(&self, output: &mut W, rng: &mut rand::rngs::StdRng) -> error::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        for vcf_type in self.format_type.as_ref() {
            for vcf_number in self.format_number.as_ref() {
                output.write_all(&generate_value(
                    rng,
                    vcf_type,
                    vcf_number,
                    self.sample as u8,
                )?)?;
                if Some(vcf_number) != self.format_number.as_ref().last()
                    || Some(vcf_type) != self.format_type.as_ref().last()
                {
                    output.write_all(b":")?;
                }
            }
        }

        Ok(())
    }

    fn info<W>(&self, output: &mut W, rng: &mut rand::rngs::StdRng) -> error::Result<()>
    where
        W: std::io::Write + ?Sized,
    {
        for vcf_type in self.info_type.as_ref() {
            if vcf_type == b"Flag" {
                if rng.gen_bool(0.5) {
                    output.write_all(&self.info_prefix)?;
                    output.write_all(b"Flag_0;")?;
                }
            } else {
                for vcf_number in self.info_number.as_ref() {
                    output.write_all(&self.info_prefix)?;
                    output.write_all(vcf_type)?;
                    output.write_all(b"_")?;
                    output.write_all(vcf_number)?;
                    output.write_all(b"=")?;
                    output.write_all(&generate_value(
                        rng,
                        vcf_type,
                        vcf_number,
                        self.sample as u8,
                    )?)?;

                    if Some(vcf_number) != self.info_number.as_ref().last()
                        || Some(vcf_type) != self.info_type.as_ref().last()
                    {
                        output.write_all(b";")?;
                    }
                }
            }
        }

        Ok(())
    }

    /// Generate vcf record
    pub fn generate(
        &self,
        output: &mut dyn std::io::Write,
        rng: &mut rand::rngs::StdRng,
    ) -> error::Result<()> {
        // chromosomes
        output.write_all(
            self.contigs
                .as_ref()
                .choose(rng)
                .ok_or(error::create_unreachable!())?,
        )?;
        output.write_all(b"\t")?;

        // position
        output.write_all(&self.position.clone().get(rng))?;
        output.write_all(b"\t")?;

        // identifiant
        let id_len = self.id_prefix.len() + self.id_len + self.id_suffix.len();
        output.write_all(&self.id_prefix)?;
        output.write_all(&self.id.generate(rng, self.id_len)?)?;
        output.write_all(&self.id_suffix)?;
        if id_len == 0 {
            output.write_all(&[b'.'])?;
        }
        output.write_all(&[b'\t'])?;

        // reference
        output.write_all(&self.reference.generate(rng, self.reference_len)?)?;
        output.write_all(b"\t")?;

        // alternative
        let alt_len = rng.gen_range(0..self.alternative_len);
        if alt_len == 0 {
            output.write_all(b".")?;
        } else {
            output.write_all(&self.alternative.generate(rng, alt_len)?)?;
        }
        output.write_all(b"\t")?;

        // quality
        output.write_all(&self.quality.clone().get(rng))?;
        output.write_all(b"\t")?;

        // filter
        let nb_filters =
            <values::Integer as Into<core::ops::Range<i32>>>::into(self.filter.clone()).len();
        if nb_filters == 0 || rng.gen_bool(1.0 / nb_filters as f64) {
            output.write_all(b".")?;
        } else {
            output.write_all(&self.filter_prefix)?;
            output.write_all(rng.gen_range(0..nb_filters).to_string().as_bytes())?;
        }

        if (!self.info_type.as_ref().is_empty() && !self.info_number.as_ref().is_empty())
            && (!self.format_type.as_ref().is_empty() && !self.format_number.as_ref().is_empty()
                || self.sample != 0)
        {
            output.write_all(b"\t")?;
        }

        // info
        if !self.info_type.as_ref().is_empty() && !self.info_number.as_ref().is_empty() {
            self.info(output, rng)?;

            // check end of line
            if (!self.format_type.as_ref().is_empty() || !self.format_number.as_ref().is_empty())
                && self.sample != 0
            {
                output.write_all(b"\t")?;
            }
        }

        // format
        if !self.format_type.as_ref().is_empty()
            && !self.format_number.as_ref().is_empty()
            && self.sample != 0
        {
            self.format(output)?;
            output.write_all(b"\t")?;
        }

        // sample
        for s in 0..self.sample {
            self.sample(output, rng)?;
            if s != self.sample - 1 {
                output.write_all(b"\t")?;
            }
        }

        Ok(())
    }
}

fn generate_value(
    rng: &mut rand::rngs::StdRng,
    vcf_type: &[u8],
    vcf_number: &[u8],
    nb_samples: u8,
) -> error::Result<Vec<u8>> {
    match vcf_type {
        b"Integer" => match vcf_number {
            b"1" | b"A" => Ok(values::Integer::default().get(rng)),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(values::Integer::default().get(rng));
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(values::Integer::default().get(rng));
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(values::Integer::default().get(rng));
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            _ => Err(create_unreachable!()),
        },
        b"Float" => match vcf_number {
            b"1" | b"A" => Ok(values::Float::default().get(rng)),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(values::Float::default().get(rng));
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(values::Float::default().get(rng));
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(values::Float::default().get(rng));
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            _ => Err(create_unreachable!()),
        },
        b"Character" => match vcf_number {
            b"1" | b"A" => Ok(values::Alphabet::default().generate(rng, 1)?),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(values::Alphabet::default().generate(rng, 1)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(values::Alphabet::default().generate(rng, 1)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(values::Alphabet::default().generate(rng, 1)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            _ => Err(create_unreachable!()),
        },
        b"String" => match vcf_number {
            b"1" | b"A" => values::Alphabet::default().generate(rng, 10),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(values::Alphabet::default().generate(rng, 10)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(values::Alphabet::default().generate(rng, 10)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(values::Alphabet::default().generate(rng, 10)?);
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

#[cfg(test)]
mod tests {
    /* std use */

    /* project use */
    use super::*;

    const DEFAULT: &[u8] = b"YAR028W	509242864	.	A	.	224	filter_0	info_Integer_1=-1867486109;info_Integer_2=1180908492,1041698939;info_Integer_A=-207506017;info_Integer_R=-1221871790,-1356802783;info_Integer_G=-496257857,2127853583,-1498117423;info_Integer_.=2082620030,-344161843,-1022296784,-1007334138;info_Float_1=68.286865;info_Float_2=-96.154594,-23.433853;info_Float_A=-48.782158;info_Float_R=-46.15216,-92.639305;info_Float_G=-7.5115204,74.78337,1.5983124;info_Float_.=26.825455;info_Flag_0;info_Character_1=b;info_Character_2=L,^;info_Character_A=4;info_Character_R=&,a;info_Character_G=N,k,%;info_Character_.={;info_String_1=nSOJk4@lC,;info_String_2=jS/\\D&BI|t,Y!R:7saso?;info_String_A=d!\"JhX)qQp;info_String_R=LD?P=?w~A),5[[@lIC.kc;info_String_G=3/bSljA-eF,F9c\"303:t],TqU?Kssw+$;info_String_.=[MEs+_JX%Y	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	1331697702:-73747613,1645597043:-1553292372:-1685240233,-1820034465:300184414,394747854,1197504288:-512239285,-1414044731:-26.444412:12.577988,-87.76228:-3.4822464:-95.66553,55.56636:-43.384956,-35.16729,6.755356:-9.445259,-43.99848,-94.4432,-92.06316:_:{,z:F:H,i:T,6,j:[,+:2;2l`>IYzJ:v7\"4m{_>h~,<G_oKV#{ze:!)!m\'S9/0_:dJ?2UvwuUy,^7GYW`=N;7:|)8^vf>dg#,1c`ok8Kh$S,_+GbKm=pyh:b:}\"s1f#s/	-1699207592:-1247215950,-1253877200:-1343277579:188169583,-1589761063:-532454402,989628108,295511500:1300868485:-14.547348:10.005661,82.95245:46.642517:90.124435,12.111877:69.43762,-11.427376,58.87137:-3.6344528,56.566788:e:),I:j:t,i:U,&,v:m,<,_,/:4y*`HYz#-o:9zE|;./\"-M,;<;|nStAje:SF>o/R,iE#:/,\"$RTVc(7,*(Sn6>ZQPD:ho#H0_<Tl9,PWg*UP~Esp,,{OVYEkbvA:.?sf#3gn*R,]yhrn0?zU8,N0+\"VIx*d-	-1697963895:1138852593,181408155:-317412374:-1000659906,1329247534:628009109,-1501500099,-1741170910:-1510591037,-995737568,2069116675,-1117969497:25.04361:84.79895,44.54808:36.19725:-48.734688,-33.58867:-54.331757,84.5206,69.88823:87.006195:?:E,t:,:?,B:b,$,s:?,4,%,}:Pm=/<N[3&;:=H>}~CRs}y,o^H0T~a31`:EJblc1Z!bn:tm9=sB<\":\\,7oU=q*dDU(:-k5AqYk|~^,o\"SF3e.$lt,R9J~QqXY_R:jI\\,>VLD1@,O#Bvp?rT;+,+]M$Tdqh7g,Qa;ou#<4,C";

    const NO_SAMPLE: &[u8] = b"YAR028W	509242864	.	A	.	224	filter_0	info_Integer_1=-1867486109;info_Integer_2=1180908492,1041698939;info_Integer_A=-207506017;info_Integer_R=-1221871790,-1356802783;info_Integer_G=;info_Integer_.=2082620030,-344161843,-1022296784,-1007334138;info_Float_1=68.286865;info_Float_2=-96.154594,-23.433853;info_Float_A=-48.782158;info_Float_R=-46.15216,-92.639305;info_Float_G=;info_Float_.=1.5983124,-8.867523,77.741455,-86.29277;info_Flag_0;info_Character_1=5;info_Character_2=b,L;info_Character_A=^;info_Character_R=4,&;info_Character_G=;info_Character_.=%,{,n,S;info_String_1=OJk4@lC,jS;info_String_2=/\\D&BI|tY!,R:7saso?d!;info_String_A=\"JhX)qQpLD;info_String_R=?P=?w~A)5[,[@lIC.kc3/;info_String_G=;info_String_.=ljA-eFF9c\",303:t]TqU?,Kssw+$*[ME";

    const NO_INFO: &[u8] = b"YAR028W	509242864	.	A	.	224	filter_0format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	-1867486109:1180908492,1041698939:-207506017:-1221871790,-1356802783:-496257857,2127853583,-1498117423:2082620030,-344161843,-1022296784,-1007334138:68.286865:-96.154594,-23.433853:-48.782158:-46.15216,-92.639305:-7.5115204,74.78337,1.5983124:26.825455:5:b,L:^:4,&:a,N,k:{:nSOJk4@lC,:jS/\\D&BI|t,Y!R:7saso?:d!\"JhX)qQp:LD?P=?w~A),5[[@lIC.kc:3/bSljA-eF,F9c\"303:t],TqU?Kssw+$:[MEs+_JX%Y	1331697702:-73747613,1645597043:-1553292372:-1685240233,-1820034465:300184414,394747854,1197504288:-512239285,-1414044731:-26.444412:12.577988,-87.76228:-3.4822464:-95.66553,55.56636:-43.384956,-35.16729,6.755356:-9.445259,-43.99848,-94.4432,-92.06316:_:{,z:F:H,i:T,6,j:[,+:2;2l`>IYzJ:v7\"4m{_>h~,<G_oKV#{ze:!)!m\'S9/0_:dJ?2UvwuUy,^7GYW`=N;7:|)8^vf>dg#,1c`ok8Kh$S,_+GbKm=pyh:b:}\"s1f#s/	-1699207592:-1247215950,-1253877200:-1343277579:188169583,-1589761063:-532454402,989628108,295511500:1300868485:-14.547348:10.005661,82.95245:46.642517:90.124435,12.111877:69.43762,-11.427376,58.87137:-3.6344528,56.566788:e:),I:j:t,i:U,&,v:m,<,_,/:4y*`HYz#-o:9zE|;./\"-M,;<;|nStAje:SF>o/R,iE#:/,\"$RTVc(7,*(Sn6>ZQPD:ho#H0_<Tl9,PWg*UP~Esp,,{OVYEkbvA:.?sf#3gn*R,]yhrn0?zU8,N0+\"VIx*d-";

    const NO_INFO_SAMPLE: &[u8] = b"YAR028W	509242864	.	A	.	224	filter_0";

    const NO_INFO_SAMPLE_FILTER: &[u8] = b"YAR028W	509242864	.	A	.	224	.";

    #[test]
    fn default() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Record::builder().build().unwrap();

        generator.generate(&mut output, &mut rng)?;

        assert_eq!(output, DEFAULT);

        Ok(())
    }

    #[test]
    fn no_sample() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Record::builder().sample(0).build().unwrap();

        generator.generate(&mut output, &mut rng)?;

        assert_eq!(output, NO_SAMPLE);

        Ok(())
    }

    #[test]
    fn no_info() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Record::builder()
            .info_number(values::VcfInfoNumber::UserDefine(vec![]))
            .build()
            .unwrap();

        generator.generate(&mut output, &mut rng)?;

        assert_eq!(output, NO_INFO);

        Ok(())
    }

    #[test]
    fn no_info_sample() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Record::builder()
            .info_number(values::VcfInfoNumber::UserDefine(vec![]))
            .sample(0)
            .build()
            .unwrap();

        generator.generate(&mut output, &mut rng)?;

        assert_eq!(output, NO_INFO_SAMPLE);

        Ok(())
    }

    #[test]
    fn no_info_sample_filter() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Record::builder()
            .filter(values::Integer::UserDefine(0..0))
            .info_number(values::VcfInfoNumber::UserDefine(vec![]))
            .format_number(values::VcfFormatNumber::UserDefine(vec![]))
            .sample(0)
            .build()
            .unwrap();

        generator.generate(&mut output, &mut rng)?;

        assert_eq!(output, NO_INFO_SAMPLE_FILTER);

        Ok(())
    }
}
