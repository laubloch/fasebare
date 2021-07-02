// src/fasta_files_mgt/fasta_open_read.rs :

// https://linuxfr.org/forums/programmationautre/posts/rust-lire-des-donnees-de-type-i8-depuis-un-fichier
// https://www.it-swarm-fr.com/fr/file-io/quelle-est-la-maniere-de-facto-de-lire-et-decrire-des-fichiers-dans-rust-1.x/1054845808/

pub mod fasta_open_read {

    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::fs::Metadata;
//    use std::u8;
    use std::str;
    use std::slice;

    use crate::sequences_matrix::build_sequences_matrix::build_sequences_matrix;
    
    pub struct Config {
	pub filename1: String,
	pub filename2: String,
    }
    
    impl Config {
	pub fn new(args: &[String]) -> Config {
	    if args.len() < 3 {
		panic!("pas assez d'arguments");
	    }
	    let filename1 = args[1].clone();
	    let filename2 = args[2].clone();
	    
	    Config { filename1, filename2 }
	}
    }

    pub fn get_filenames() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args);
	
	println!("Alignement de {} avec {} \n", config.filename1, config.filename2);
	
        let mut data1 = fasta_open_file(config.filename1);
        let mut data2 = fasta_open_file(config.filename2);

        read_seq(&mut data1, &mut data2);
    }

    fn fasta_open_file(filename: String) -> (File, Metadata) {
        let metadata = fs::metadata(&filename).expect("Fichier non trouvé.");
        let filelength = metadata.len();

        let f = File::open(filename).expect("Fichier non trouvé !");
        (f, metadata)
    }

        fn read_seq(data1: &mut (File, Metadata), data2: &mut (File, Metadata)) {
            let mut seq1 = vec![0; data1.1.len() as usize];
            let mut seq2 = vec![0; data2.1.len() as usize];
            data1.0.read(&mut seq1).expect("débordement de buffer");
            data2.0.read(&mut seq2).expect("débordement de buffer");

             let seq1_str = unsafe {
	        str::from_utf8_unchecked(&seq1)
	    };
            let seq2_str = unsafe {
	        str::from_utf8_unchecked(&seq2)
	    };
	    
	    println!("1er caractère : {:?}", &seq1[0]);
	    println!("1er caractère : {:?}", &seq1_str.bytes().nth(0));
	    let sequence1 = split_seq(seq1);
	    let sequence2 = split_seq(seq2);
	    print_seq(sequence1);
	    print_seq(sequence2);
	}

    fn split_seq(fasta_sequence: Vec<u8>) -> (Vec<u8>, Vec<u8>) {
	    let mut ident: Vec<u8> = vec![0, 1, 2];
	    let mut sequence_nuc: Vec<u8> = vec![3, 4, 5];
	    if fasta_sequence[0] == 62 {
		let index = fasta_sequence.iter().position(|x| *x == 10).unwrap();
		println!("Position du premier LF : {}", index);
		ident = (fasta_sequence.split_at(index).0).to_vec();
		sequence_nuc = (fasta_sequence.split_at(index).1).to_vec();
	    }
	let sequence_clean: Vec<u8> = remove_LF(&sequence_nuc);
	(ident, sequence_clean)
    }

    fn remove_LF(sequence_nuc: &Vec<u8>) -> Vec<u8> {
	let mut sequence_clean: Vec<u8> = (&sequence_nuc).to_vec();
	for i in (0..sequence_clean.len()).rev() {
            if sequence_clean[i] == 10 {
		sequence_clean.remove(i);
            }
	}
	sequence_clean
    }
    
    fn print_seq(sequence: (Vec<u8>, Vec<u8>)) {
		println!("Ident : {:?}", sequence.0);
		println!("Séquence : {:?}", sequence.1);
		let ident_str = unsafe {
	            str::from_utf8_unchecked(&sequence.0)
		};
		let sequence_str = unsafe {
	            str::from_utf8_unchecked(&sequence.1)
		};
		println!("Ident : {}", &ident_str);
		println!("Séquence : {}", &sequence_str);
    }
	       
}


