//! VCF header generation

/* std use */

/* crates use */
use rand::seq::SliceRandom as _;
use rand::Rng as _;

/* projet use */
use crate::constants;
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
            output.write_all(b".")?;
        }
        output.write_all(b"\t")?;

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
        }
        // check end of line
        if (!self.format_type.as_ref().is_empty() || !self.format_number.as_ref().is_empty())
            && self.sample != 0
        {
            output.write_all(b"\t")?;
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
            b"1" | b"A" => Ok(values::Alphabet::A2z.generate(rng, 1)?),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(values::Alphabet::A2z.generate(rng, 1)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(values::Alphabet::A2z.generate(rng, 1)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(values::Alphabet::A2z.generate(rng, 1)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            _ => Err(create_unreachable!()),
        },
        b"String" => match vcf_number {
            b"1" | b"A" => values::Alphabet::A2z.generate(rng, constants::VCF_STRING_LENGTH),
            b"2" | b"R" => {
                let mut ret = Vec::new();
                for _ in 0..2 {
                    ret.extend(values::Alphabet::A2z.generate(rng, constants::VCF_STRING_LENGTH)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"G" => {
                let mut ret = Vec::new();
                for _ in 0..nb_samples {
                    ret.extend(values::Alphabet::A2z.generate(rng, constants::VCF_STRING_LENGTH)?);
                    ret.push(b',');
                }
                ret.pop();

                Ok(ret)
            }
            b"." => {
                let mut ret = Vec::new();
                for _ in 0..rng.gen_range(1..5) {
                    ret.extend(values::Alphabet::A2z.generate(rng, constants::VCF_STRING_LENGTH)?);
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

    const DEFAULT: &[u8] = b"YAR028W	509242864	.	A	.	224	filter_0	info_Integer_1=-1867486109;info_Integer_2=1180908492,1041698939;info_Integer_A=-207506017;info_Integer_R=-1221871790,-1356802783;info_Integer_G=-496257857,2127853583,-1498117423;info_Integer_.=2082620030,-344161843,-1022296784,-1007334138;info_Float_1=68.286865;info_Float_2=-96.154594,-23.433853;info_Float_A=-48.782158;info_Float_R=-46.15216,-92.639305;info_Float_G=-7.5115204,74.78337,1.5983124;info_Float_.=26.825455;info_Flag_0;info_Character_1=i;info_Character_2=r,[;info_Character_A=g;info_Character_R=M,D;info_Character_G=h,w,\\;info_Character_.=C,G,p,];info_String_1=ZoXMT;info_String_2=gQouV,Gn`Jw;info_String_A=eVDDU;info_String_R=YytzA,ny[_P;info_String_G=Oshsq,bSjAd,bZcRF;info_String_.=rQ_[V,S^RtS,vzMeT,jonYV	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	-1552897203:1249370088,894744660:-1298826907:-1500526673,846767901:154354090,1292630937,-513388490:730433769,-1782228224,1193004039,1639963889:-31.463745:-74.13223,44.792007:-4.5392303:-42.586063,-20.249939:-19.714546,-48.754406,40.519638:-27.838158:L:J,L:n:u,P:t,f,`:r,^:aaSsw:svYGC,zkT\\W:k_sGD:gZcCc,]tIGE:bcnVW,JVaDB,nQSHY:[QBCg,L`Scx,xXYm`,NnOG[	-1345745815:173280036,-939420073:-1365650667:679852521,1295053734:732715199,-819759668,-308523151:1942972144,-249711286,1737760149:-53.047443:-97.35165,-58.53014:93.27409:-89.49225,65.68997:62.677032,92.94722,32.79944:52.132156,-30.33149:z:R,v:G:G,X:B,g,q:[,a,B:w_Zxx:kAFA[,o`OId:JgjZD:StKau,vtaIh:wmmrI,gNXcb,hRd]Q:OgukS	946791943:-2019035904,1055813342:-2045085244:-1401538285,878536766:731752434,1439145027,-966674455:-1096509554,-1513894259,1176983779,-199713084:51.48242:-93.36465,6.6719513:32.869843:-77.50437,-17.745377:38.63495,-9.558914,42.16661:-6.823944,-39.047478,48.595016,68.83052:w:O,m:A:i,Z:P,w,y:s:KBssX:JGMMK,`HVkg:oY`vk:xarZo,yTnQF:EntKU,mnaDW,uppug:FhYRx,BZHMq";

    const NO_SAMPLE: &[u8] = b"YAR028W	509242864	.	A	.	224	filter_0	info_Integer_1=-1867486109;info_Integer_2=1180908492,1041698939;info_Integer_A=-207506017;info_Integer_R=-1221871790,-1356802783;info_Integer_G=;info_Integer_.=2082620030,-344161843,-1022296784,-1007334138;info_Float_1=68.286865;info_Float_2=-96.154594,-23.433853;info_Float_A=-48.782158;info_Float_R=-46.15216,-92.639305;info_Float_G=;info_Float_.=1.5983124,-8.867523,77.741455,-86.29277;info_Flag_0;info_Character_1=M;info_Character_2=i,r;info_Character_A=[;info_Character_R=g,M;info_Character_G=;info_Character_.=h;info_String_1=w\\voC;info_String_2=Gp]Zo,XMTgQ;info_String_A=ouVGn;info_String_R=`JweV,DDUYy;info_String_G=;info_String_.=zAny[,_POsh,sqbSj";

    const NO_INFO: &[u8] = b"YAR028W	509242864	.	A	.	224	filter_0	format_Integer_1:format_Integer_2:format_Integer_A:format_Integer_R:format_Integer_G:format_Integer_.:format_Float_1:format_Float_2:format_Float_A:format_Float_R:format_Float_G:format_Float_.:format_Character_1:format_Character_2:format_Character_A:format_Character_R:format_Character_G:format_Character_.:format_String_1:format_String_2:format_String_A:format_String_R:format_String_G:format_String_.	-1867486109:1180908492,1041698939:-207506017:-1221871790,-1356802783:-496257857,2127853583,-1498117423:2082620030,-344161843,-1022296784,-1007334138:68.286865:-96.154594,-23.433853:-48.782158:-46.15216,-92.639305:-7.5115204,74.78337,1.5983124:26.825455:L:M,i:r:[,g:M,D,h:C,G,p,]:ZoXMT:gQouV,Gn`Jw:eVDDU:YytzA,ny[_P:Oshsq,bSjAd,bZcRF:rQ_[V,S^RtS,vzMeT,jonYV	-1552897203:1249370088,894744660:-1298826907:-1500526673,846767901:154354090,1292630937,-513388490:730433769,-1782228224,1193004039,1639963889:-31.463745:-74.13223,44.792007:-4.5392303:-42.586063,-20.249939:-19.714546,-48.754406,40.519638:-27.838158:L:J,L:n:u,P:t,f,`:r,^:aaSsw:svYGC,zkT\\W:k_sGD:gZcCc,]tIGE:bcnVW,JVaDB,nQSHY:[QBCg,L`Scx,xXYm`,NnOG[	-1345745815:173280036,-939420073:-1365650667:679852521,1295053734:732715199,-819759668,-308523151:1942972144,-249711286,1737760149:-53.047443:-97.35165,-58.53014:93.27409:-89.49225,65.68997:62.677032,92.94722,32.79944:52.132156,-30.33149:z:R,v:G:G,X:B,g,q:[,a,B:w_Zxx:kAFA[,o`OId:JgjZD:StKau,vtaIh:wmmrI,gNXcb,hRd]Q:OgukS";

    const NO_INFO_SAMPLE: &[u8] = b"YAR028W	509242864	.	A	.	224	filter_0";

    const NO_INFO_SAMPLE_FILTER: &[u8] = b"YAR028W	509242864	.	A	.	224	.";

    const SET_ID: &[u8] = b"YAR028W	509242864	id_i_Pdz!	A	.	224	.";

    const ID_0: &[u8] = b"YAR028W	509242864	.	a	.	114	.";

    const LARGE_ALT: &[u8] = b"YAR028W\t509242864\t.\ta\tACA\t86\t.";

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

    #[test]
    fn set_id() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Record::builder()
            .filter(values::Integer::UserDefine(0..0))
            .info_number(values::VcfInfoNumber::UserDefine(vec![]))
            .format_number(values::VcfFormatNumber::UserDefine(vec![]))
            .sample(0)
            .id_len(5)
            .id(values::Alphabet::A2z)
            .id_prefix(b"id_".to_vec())
            .id_suffix(b"!".to_vec())
            .build()
            .unwrap();

        generator.generate(&mut output, &mut rng)?;

        assert_eq!(output, SET_ID);

        Ok(())
    }

    #[test]
    fn id_0() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Record::builder()
            .filter(values::Integer::UserDefine(0..0))
            .info_number(values::VcfInfoNumber::UserDefine(vec![]))
            .format_number(values::VcfFormatNumber::UserDefine(vec![]))
            .sample(0)
            .id_len(0)
            .id(values::Alphabet::A2z)
            .build()
            .unwrap();

        generator.generate(&mut output, &mut rng)?;

        assert_eq!(output, ID_0);

        Ok(())
    }

    #[test]
    fn large_alt() -> error::Result<()> {
        let mut output = Vec::new();
        let mut rng = crate::rand();

        let generator = Record::builder()
            .filter(values::Integer::UserDefine(0..0))
            .info_number(values::VcfInfoNumber::UserDefine(vec![]))
            .format_number(values::VcfFormatNumber::UserDefine(vec![]))
            .alternative_len(6)
            .sample(0)
            .id_len(0)
            .id(values::Alphabet::A2z)
            .build()
            .unwrap();

        generator.generate(&mut output, &mut rng)?;

        assert_eq!(output, LARGE_ALT);

        Ok(())
    }
}
