mod maps;
use maps::*;

use std::thread::JoinHandle;
use anyhow::Result;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;


fn main() -> Result<()> {
    // let mut dna = " U U U G U G G U U U U U G C G G U G C C G C U G G U U U C C U U U ".to_string();
    let mut dna = " T A C T T G C T T C A C C T C G C G C C G T T A T T G C G T C C G ".to_string();
    dna = dna.replace(" ", "");

    let mut slices = get_slices(&mut dna, 3).iter().map(|x| x.to_string()).collect();
    let rna = dna_to_rna(slices);

    // rna_to_aa(&mut rna);


    Ok(())
}


fn rna_to_aa(slice: &mut str) -> Vec<String> {
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