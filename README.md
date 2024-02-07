bunch of simple utils for doing bio hw
I experimented w/ concurrency (multiple mutable refrences to non-overlapping slices of original string ->  each is processed concurrently)

- converting dna to rna (string)
- converting rna codons to amino acids (string)

maps.rs
 - contains mapping between dna and rna nucleotides
 - contains mapping between rna codons and amino acids
  
main.rs
 - contains spliter function
 - dna to rna
 - rna to amino acids


