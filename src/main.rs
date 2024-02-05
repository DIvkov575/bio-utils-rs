use std::collections::HashMap;
use lazy_static::lazy_static;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;


    type HashMapFnv<K, V> = HashMap<K, V, BuildHasherDefault<FnvHasher>>;

lazy_static!(

    static ref a: HashMapFnv = HashMapFnv::from_iter([
         ("T", "A"),
         ("A", "U"),
         ("C", "G"),
         ("G", "C"),
         ("t", "a"),
         ("a", "u"),
         ("c", "g"),
         ("g", "c"),
    ]);
    static ref b: HashMapFnv = HashMapFnv::form_iter([
        ("UUU", "Phe"),
        ("UUC", "Phe"),
        ("UUA", "Leu"),
        ("UUG", "Leu"),
        ("CUU", "Leu"),
        ("CUC", "Leu"),
        ("CUA", "Leu"),
        ("CUG", "Leu"),
        ("AUU", "Ile"),
        ("AUC", "Ile"),
        ("AUA", "Ile"),
        ("AUG", "Met"),  // or start
        ("GUU", "Val"),
        ("GUC", "Val"),
        ("GUA", "Val"),
        ("GUG", "Val"),
        ("UCU", "Ser"),
        ("UCC", "Ser"),
        ("UCA", "Ser"),
        ("UCG", "Ser"),
        ("CCU", "Pro"),
        ("CCC", "Pro"),
        ("CCA", "Pro"),
        ("CCG", "Pro"),
        ("ACU", "Thr"),
        ("ACC", "Thr"),
        ("ACA", "Thr"),
        ("ACG", "Thr"),
        ("GCU", "Ala"),
        ("GCC", "Ala"),
        ("GCA", "Ala"),
        ("GCG", "Ala"),
        ("UAU", "Tyr"),
        ("UAC", "Tyr"),
        ("UAA", "Stop"),  //stop
        ("UAG", "Stop"),  // stop
        ("CAU", "His"),
        ("CAC", "His"),
        ("CAA", "Gln"),
        ("CAG", "Gln"),
        ("AAU", "Asn"),
        ("AAC", "Asn"),
        ("AAA", "Lys"),
        ("AAG", "Lys"),
        ("GAU", "Asp"),
        ("GAC", "Asp"),
        ("GAA", "Glu"),
        ("GAG", "Glu"),
        ("UGU", "Cys"),
        ("UGC", "Cys"),
        ("UGA", "Stop"), // stop
        ("UGG", "Trp"),
        ("CGU", "Arg"),
        ("CGC", "Arg"),
        ("CGA", "Arg"),
        ("CGG", "Arg"),
        ("AGU", "Ser"),
        ("AGC", "Ser"),
        ("AGA", "Arg"),
        ("AGG", "Arg"),
        ("GGU", "Gly"),
        ("GGC", "Gly"),
        ("GGA", "Gly"),
        ("GGG", "Gly"),
        ("uuu", "Phe"),
        ("uuc", "Phe"),
        ("uua", "Leu"),
        ("uug", "Leu"),
        ("cuu", "Leu"),
        ("cuc", "Leu"),
        ("cua", "Leu"),
        ("cug", "Leu"),
        ("auu", "Ile"),
        ("auc", "Ile"),
        ("aua", "Ile"),
        ("aug", "Met"),  // or start
        ("guu", "Val"),
        ("guc", "Val"),
        ("gua", "Val"),
        ("gug", "Val"),
        ("ucu", "Ser"),
        ("ucc", "Ser"),
        ("uca", "Ser"),
        ("ucg", "Ser"),
        ("ccu", "Pro"),
        ("ccc", "Pro"),
        ("cca", "Pro"),
        ("ccg", "Pro"),
        ("acu", "Thr"),
        ("acc", "Thr"),
        ("aca", "Thr"),
        ("acg", "Thr"),
        ("gcu", "Ala"),
        ("gcc", "Ala"),
        ("gca", "Ala"),
        ("gcg", "Ala"),
        ("uau", "Tyr"),
        ("uac", "Tyr"),
        ("uaa", "Stop"),  // stop
        ("uag", "Stop"),  // stop
        ("cau", "His"),
        ("cac", "His"),
        ("caa", "Gln"),
        ("cag", "Gln"),
        ("aau", "Asn"),
        ("aac", "Asn"),
        ("aaa", "Lys"),
        ("aag", "Lys"),
        ("gau", "Asp"),
        ("gac", "Asp"),
        ("gaa", "Glu"),
        ("gag", "Glu"),
        ("ugu", "Cys"),
        ("ugc", "Cys"),
        ("uga", "Stop"),  // stop
        ("ugg", "Trp"),
        ("cgu", "Arg"),
        ("cgc", "Arg"),
        ("cga", "Arg"),
        ("cgg", "Arg"),
        ("agu", "Ser"),
        ("agc", "Ser"),
        ("aga", "Arg"),
        ("agg", "Arg"),
        ("ggu", "Gly"),
        ("ggc", "Gly"),
        ("gga", "Gly"),
        ("ggg", "Gly"),
    ]);
    }


// rna_to_AA = {
fn main() -> Result<()>{






}