mod maps;
use maps::*;

use std::thread::JoinHandle;
use anyhow::Result;
use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use rayon::prelude::*;


fn main() -> Result<()> {
    // let mut rnas = vec![
    //     "U U G U U C G U U G U G G U G C G C G G C U U U U U C G C U G G C".to_string().replace(" ", ""),
    //     "U U U G U G G U U U U U G C G G U G C C G C U G G U U U C C U U U ".to_string().replace(" ", ""),
    //     "G C U G U U U G G U U C C G C U U C G C C U G U G C G U G G U U C ".to_string().replace(" ", ""),
    //     "G U C G U U G U U U G C G U G C G U U C U G U G U G G G U U U G U".to_string().replace(" ", ""),
    // ];
    // for rna in &mut rnas {
    //     let mut aa = rna_to_aa(rna);
    //     for a in aa {
    //         print!("{}, ", a)
    //     }
    //     println!("");
    //
    // }


    let dna = "G G G T A C A T A T C A C T G A A C C G T G G A A T T ".to_string().replace(" ", "");
    let mut rna = dna_to_rna(vec![dna]);
    println!("{}", rna);
    let aa = rna_to_aa(&mut rna);
    println!("{}", rna);
    for a in aa { print!("{}, ", a)}



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