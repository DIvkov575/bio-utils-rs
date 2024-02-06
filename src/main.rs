use std::collections::HashMap;
use lazy_static::lazy_static;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;
use anyhow::Result;


type HashMapFnv<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

lazy_static!(

    static ref dna_rna_nucleotide_pairs: HashMapFnv<String, String> = HashMapFnv::from_iter([
         ("T".to_string(), "A".to_string()),
         ("A".to_string(), "U".to_string()),
         ("C".to_string(), "G".to_string()),
         ("G".to_string(), "C".to_string()),
         ("t".to_string(), "a".to_string()),
         ("a".to_string(), "u".to_string()),
         ("c".to_string(), "g".to_string()),
         ("g".to_string(), "c".to_string()),
    ]);
    static ref rna_aa_pairs: HashMapFnv<String, String> = HashMapFnv::form_iter([
        ("UUU".into(), "Phe".into()),
        ("UUC".into(), "Phe".into()),
        ("UUA".into(), "Leu".into()),
        ("UUG".into(), "Leu".into()),
        ("CUU".into(), "Leu".into()),
        ("CUC".into(), "Leu".into()),
        ("CUA".into(), "Leu".into()),
        ("CUG".into(), "Leu".into()),
        ("AUU".into(), "Ile".into()),
        ("AUC".into(), "Ile".into()),
        ("AUA".into(), "Ile".into()),
        ("AUG".into(), "Met".into()),  // or start
        ("GUU".into(), "Val".into()),
        ("GUC".into(), "Val".into()),
        ("GUA".into(), "Val".into()),
        ("GUG".into(), "Val".into()),
        ("UCU".into(), "Ser".into()),
        ("UCC".into(), "Ser".into()),
        ("UCA".into(), "Ser".into()),
        ("UCG".into(), "Ser".into()),
        ("CCU".into(), "Pro".into()),
        ("CCC".into(), "Pro".into()),
        ("CCA".into(), "Pro".into()),
        ("CCG".into(), "Pro".into()),
        ("ACU".into(), "Thr".into()),
        ("ACC".into(), "Thr".into()),
        ("ACA".into(), "Thr".into()),
        ("ACG".into(), "Thr".into()),
        ("GCU".into(), "Ala".into()),
        ("GCC".into(), "Ala".into()),
        ("GCA".into(), "Ala".into()),
        ("GCG".into(), "Ala".into()),
        ("UAU".into(), "Tyr".into()),
        ("UAC".into(), "Tyr".into()),
        ("UAA".into(), "Stop.into()"),  //stop
        ("UAG".into(), "Stop.into()"),  // stop
        ("CAU".into(), "His".into()),
        ("CAC".into(), "His".into()),
        ("CAA".into(), "Gln".into()),
        ("CAG".into(), "Gln".into()),
        ("AAU".into(), "Asn".into()),
        ("AAC".into(), "Asn".into()),
        ("AAA".into(), "Lys".into()),
        ("AAG".into(), "Lys".into()),
        ("GAU".into(), "Asp".into()),
        ("GAC".into(), "Asp".into()),
        ("GAA".into(), "Glu".into()),
        ("GAG".into(), "Glu".into()),
        ("UGU".into(), "Cys".into()),
        ("UGC".into(), "Cys".into()),
        ("UGA".into(), "Stop.into()"), // stop
        ("UGG".into(), "Trp".into()),
        ("CGU".into(), "Arg".into()),
        ("CGC".into(), "Arg".into()),
        ("CGA".into(), "Arg".into()),
        ("CGG".into(), "Arg".into()),
        ("AGU".into(), "Ser".into()),
        ("AGC".into(), "Ser".into()),
        ("AGA".into(), "Arg".into()),
        ("AGG".into(), "Arg".into()),
        ("GGU".into(), "Gly".into()),
        ("GGC".into(), "Gly".into()),
        ("GGA".into(), "Gly".into()),
        ("GGG".into(), "Gly".into()),
        ("uuu".into(), "Phe".into()),
        ("uuc".into(), "Phe".into()),
        ("uua".into(), "Leu".into()),
        ("uug".into(), "Leu".into()),
        ("cuu".into(), "Leu".into()),
        ("cuc".into(), "Leu".into()),
        ("cua".into(), "Leu".into()),
        ("cug".into(), "Leu".into()),
        ("auu".into(), "Ile".into()),
        ("auc".into(), "Ile".into()),
        ("aua".into(), "Ile".into()),
        ("aug".into(), "Met".into()),  // or start
        ("guu".into(), "Val".into()),
        ("guc".into(), "Val".into()),
        ("gua".into(), "Val".into()),
        ("gug".into(), "Val".into()),
        ("ucu".into(), "Ser".into()),
        ("ucc".into(), "Ser".into()),
        ("uca".into(), "Ser".into()),
        ("ucg".into(), "Ser".into()),
        ("ccu".into(), "Pro".into()),
        ("ccc".into(), "Pro".into()),
        ("cca".into(), "Pro".into()),
        ("ccg".into(), "Pro".into()),
        ("acu".into(), "Thr".into()),
        ("acc".into(), "Thr".into()),
        ("aca".into(), "Thr".into()),
        ("acg".into(), "Thr".into()),
        ("gcu".into(), "Ala".into()),
        ("gcc".into(), "Ala".into()),
        ("gca".into(), "Ala".into()),
        ("gcg".into(), "Ala".into()),
        ("uau".into(), "Tyr".into()),
        ("uac".into(), "Tyr".into()),
        ("uaa".into(), "Stop.into()"),  // stop
        ("uag".into(), "Stop.into()"),  // stop
        ("cau".into(), "His".into()),
        ("cac".into(), "His".into()),
        ("caa".into(), "Gln".into()),
        ("cag".into(), "Gln".into()),
        ("aau".into(), "Asn".into()),
        ("aac".into(), "Asn".into()),
        ("aaa".into(), "Lys".into()),
        ("aag".into(), "Lys".into()),
        ("gau".into(), "Asp".into()),
        ("gac".into(), "Asp".into()),
        ("gaa".into(), "Glu".into()),
        ("gag".into(), "Glu".into()),
        ("ugu".into(), "Cys".into()),
        ("ugc".into(), "Cys".into()),
        ("uga".into(), "Stop.into()"),  // stop
        ("ugg".into(), "Trp".into()),
        ("cgu".into(), "Arg".into()),
        ("cgc".into(), "Arg".into()),
        ("cga".into(), "Arg".into()),
        ("cgg".into(), "Arg".into()),
        ("agu".into(), "Ser".into()),
        ("agc".into(), "Ser".into()),
        ("aga".into(), "Arg".into()),
        ("agg".into(), "Arg".into()),
        ("ggu".into(), "Gly".into()),
        ("ggc".into(), "Gly".into()),
        ("gga".into(), "Gly".into()),
        ("ggg".into(), "Gly".into()),
    ]);
);

// def dna_to_pre_rna(dna_string: str) -> str:
// return dna_string.translate(dna_rna_nucleotides).__reversed__()
//
//
// def rna_to_AA(rna_string: str) -> list(str):
// pp_chain = []
// buf = ""
// for char in rna_string:
// buf += char
// if len(buf) == 3:
// pp_chain.append(buf.lower())
// buf = ""
// if len(buf) != 0:
// raise Exception("rna_string #nucleotides is not a multiple of 3")
//
// return pp_chain.__reversed__()


fn dna_to_pre_rna(dna_str: &str) -> String {
    dna_str.chars().par_iter()
}





// rna_to_AA = {
fn main() -> Result<()>{

    Ok(())
}