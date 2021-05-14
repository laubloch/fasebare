// src/fasta_files_mgt/fasta_open_read.rs :

pub mod fasta_open_read {

    use crate::fasta_files_mgt::fasta_read_seq::fasta_read_seq;

    use std::env;

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
	
	println!("Alignement de {}", config.filename1);
	println!("avec {}", config.filename2);
	
	println!("{:?}", args);

        fasta_read_seq::fasta_read_seq(config);
    }
}
