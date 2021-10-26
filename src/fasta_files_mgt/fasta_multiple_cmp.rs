// src/fasta_files_mgt/fasta_multiple_cmp.rs :

// https://linuxfr.org/forums/programmationautre/posts/rust-lire-des-donnees-de-type-i8-depuis-un-fichier
// https://www.it-swarm-fr.com/fr/file-io/quelle-est-la-maniere-de-facto-de-lire-et-decrire-des-fichiers-dans-rust-1.x/1054845808/
// https://docs.rs/simple-matrix/0.1.2/simple_matrix/

pub mod fasta_multiple_cmp {

    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io;
    use std::io::Read;
    use std::io::{prelude::*, BufReader};
    use std::io::Lines;
    use std::fs::Metadata;
    use std::str;

    use crate::sequences_matrix::build_sequences_matrix::build_sequences_matrix::print_seq;
    use crate::sequences_matrix::build_sequences_matrix::build_sequences_matrix::build_matrix;
    
    pub struct Config {
	pub query_filename: String,
	pub bank_filename: String,
	pub match_bonus: f32,
	pub gap_penalty: f32
    }
    
    impl Config {
	pub fn new(args: &[String]) -> Config {
	    if args.len() < 5 {
		panic!("pas assez d'arguments");
	    }
	    let query_filename = args[1].clone();
	    let bank_filename = args[2].clone();
	    let match_bonus: f32 = args[3].parse()
		.expect("Ce n'est pas un nombre !");
	    let gap_penalty: f32 = args[4].parse()
		.expect("Ce n'est pas un nombre !");
		
	    Config {query_filename, bank_filename, match_bonus, gap_penalty}
	}
    }
	
    pub fn get_filenames() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args);
	
	println!("Alignement de {} avec {} \n", config.query_filename, config.bank_filename);
	
        let f_query = fasta_open_file(config.query_filename);
        let f_bank = fasta_open_file(config.bank_filename);

	read_sequences(f_query,
		       f_bank,
		       config.match_bonus,
		       config.gap_penalty);

    }

    fn fasta_open_file(filename: String) -> (File) {
        let f = File::open(filename).expect("Fichier non trouvé !");
        f
    }

    fn get_sequence<B: BufRead>(count: &mut u8, ident: &mut String, lines: &mut Lines<B>)
				   -> (String, Vec<u8>) {
	let mut sequence: (String, Vec<u8>) = (String::new(), vec![]);
	let mut sequence_nuc: Vec<u8> = vec![];
	
	for line in lines {
	    let the_line = line.unwrap();
	    if the_line.len() > 0 {
		let first = &the_line[0..1];
		match first {
		    first if first == ">" => {
			if *count == 0 {
			    *ident = the_line.clone();
			    *count += 1;
			} else {
			    sequence = (ident.to_string(), sequence_nuc.clone());
			    println!("Numéro : {}", count);
			    *ident = the_line.clone();
			    sequence_nuc = vec![];
			    *count += 1;
			    return sequence;
			}
		    }
		    first if first != ">" => {
			sequence_nuc.extend(the_line.as_bytes())}
		    &_ => {}
		}
	    }
	}
	sequence = (ident.to_string(), sequence_nuc.clone());
	println!("Numéro : {}", count);
	sequence
    }


//    fn read_sequences(f: File) {
//	let fb = BufReader::new(&f);
//	let mut lines = fb.lines();
//	let mut count: u8 = 0;
//	let mut ident = String::new();
//	loop {
//	    let sequence = get_sequence(&mut count, &mut ident, &mut lines);
//	    if sequence.1.len() == 0 {
//		break} else {
//		print_seq(&sequence);
//	    }
//	}
//  }

    fn read_sequences(f_query: File,
		      f_bank: File,
		      match_bonus: f32,
		      gap_penalty: f32) {
	let fq = BufReader::new(&f_query);
	let mut fq_iter = fq.lines();
	let mut count: u8 = 0;
	let mut ident = String::new();
	let query_sequence = get_sequence(&mut count, &mut ident, &mut fq_iter);
	print_seq(&query_sequence);

	let fb = BufReader::new(&f_bank);
	let mut fb_iter = fb.lines();
	count = 0;
	loop {
	    let bank_sequence = get_sequence(&mut count, &mut ident, &mut fb_iter);
	    if bank_sequence.1.len() == 0 {
		break} else {
//		print_seq(&bank_sequence);
		build_matrix(&query_sequence,
			     &bank_sequence,
			     match_bonus,
			     gap_penalty);
	    }
	}
    }
}

