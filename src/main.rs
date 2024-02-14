mod maps;
use maps::*;

use std::thread::JoinHandle;
use anyhow::Result;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;


fn main() -> Result<()> {


    // let dna = "TAC AAA GTA GAG CAA CTG AAA GTC CAA TGA TAT CGA CTC TAT AAT GAT TAA TAA TAC TCC".to_string().replace(" ", "");
    // let mut rna = dna_to_rna(vec![dna]);
    // println!("{}", rna);
    // let mut rna = "AUG,UCU,AGU,AGG,AUU,ACC,AGG,GAA,GAC".to_string().replace()
    // let mut rna = "AUG,UCU,AGU,AGG,AUU,ACC,AGG,GAA,GAC ".to_string().replace(",", "");

    // let aa = rna_to_aa(&mut rna);
    // println!("{}", rna);
    // for a in aa { print!("{}, ", a)}

    let mut a = "AUG UUU CAU CUC GUU GAC UUU CAG GUU ACU UAGC AGA GAU AUU ACU AAU UAU UAU GAG".replace(" ", "");
    print(&rna_to_aa(&mut a));




    Ok(())
}

fn print(codons: &[String]) {
    for codon in codons {
        print!("{} ", codon);
    }

}

fn rna_to_aa(slice: &mut str) -> Vec<String> {
    // observes/matches every 3 nucleotides
    // therefore: don't include spaces
    let mut aa: Vec<String> = Vec::with_capacity(slice.len()/3 + 1);
    for i in (0..(slice.len()-2)).step_by(3) {
        aa.push(rna_aa_pairs.get(&slice[i..=i+2]).unwrap().clone());
    }
    return aa
}


fn dna_to_rna(slices: Vec<String>) -> String {
    let mut rna = String::from("");

    let mut threads: Vec<JoinHandle<String>> = Vec::new();
    for slice in slices {
        let handle = std::thread::spawn(move || {
            slice.chars()
                .map(|char|
                    dna_rna_nucleotide_pairs
                        .get(&char.to_string())
                        .unwrap()
                        .chars()
                        .collect::<Vec<_>>()[0])
                .collect()
        });
        threads.push(handle);
    }
    for handle in threads { rna += &handle.join().unwrap(); }


    rna
}


fn get_slices(input: &mut str, n: usize) -> Vec<&mut str> {
    let input_len = input.len();
    let mut slices: Vec<&mut str> = Vec::new();

    let mut main_block: &mut str = input;
    let mut branch: &mut str;

    for _ in 0..(n-1) {
        (branch, main_block) = main_block.split_at_mut(input_len /n);
        slices.push(branch);
    }
    slices.push(main_block);

    slices
}