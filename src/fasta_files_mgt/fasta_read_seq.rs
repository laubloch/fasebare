// src/fasta_files_mgt/fasta_read_seq.rs :

pub mod fasta_read_seq {

    use std::fs::File;
    use std::io::Read;

    use crate::fasta_files_mgt::fasta_open_read::fasta_open_read;

    pub fn fasta_read_seq(config: fasta_open_read::Config) {
        let mut f1 = File::open(config.filename1).expect("fichier non trouvé");
        let mut f2 = File::open(config.filename2).expect("fichier non trouvé");

        let mut seq1 = String::new();
        f1.read_to_string(&mut seq1)
            .expect("Incident de lecture du fichier");
        
        let mut seq2 = String::new();
        f2.read_to_string(&mut seq2)
            .expect("Incident de lecture du fichier");
        
        println!("Avec texte :\n{} \n{}", seq1, seq2);
    }
}
