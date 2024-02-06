mod maps;
use maps::*;

use regex::{Captures, Regex};
use std::collections::HashMap;
use lazy_static::lazy_static;
use std::hash::BuildHasherDefault;
use fnv::FnvHasher;
use anyhow::Result;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;




// fn dna_to_pre_rna(dna_str: &str) {
//     dna_str.chars().collect::<Vec<_>>().par_iter_mut().map(|i|
//         *i = dna_rna_nucleotide_pairs.get(&i.to_string()).unwrap().to_ascii_lowercase().chars().collect::<Vec<char>>()[0]
//     );
//     // return dna_str;
// }


fn main() -> Result<()> {
    let mut a = "atgcatatgc".to_string();
    let slices = get_slices(&mut a, 3);

    let mut output_buf = "".to_string();
    for slice in slices { output_buf += &replace(slice); }

    print!("{:?}", output_buf);


    Ok(())
}


fn replace(slice: &mut str) -> String {
    slice
        .chars()
        .map(|char|
            dna_rna_nucleotide_pairs
                .get(&char.to_string())
                .unwrap()
                .chars()
                .collect::<Vec<_>>()[0])
        .collect()
}



fn get_slices(input: &mut str, n: usize) -> Vec<&mut str> {
    let input_len = input.len();
    let n = 3usize;
    let mut slices: Vec<&mut str> = Vec::new();

    let mut main_block: &mut str = input;
    let mut branch: &mut str;

    for _ in 0..(n-1) {
        (branch, main_block) = (main_block).split_at_mut(input_len /n);
        slices.push(branch);
    }
    slices.push(main_block);

    slices
}