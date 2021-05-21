// src/fasta_files_mgt/fasta_read_seq.rs :

pub mod fasta_read_seq {

    use std::fs::File;
    use std::io::{self, BufRead};
    use std::path::Path;

    use crate::fasta_files_mgt::fasta_open_read::fasta_open_read;

    pub fn fasta_read_seq(config: fasta_open_read::Config) {
        let f1 = config.filename1;
        let f2 = config.filename2;

	println!("{}\n{}\n", les_lignes(&f1).0, les_lignes(&f1).1);
	println!("{}\n{}\n", les_lignes(&f2).0, les_lignes(&f2).1);
    }

    fn les_lignes(f: &String) -> (String, String) {
	let mut sequence = String::new();
	let mut ident = String::new();
	if let Ok(lines) = read_lines(f) {
	    for line in lines {
		if let Ok(texte) = line {
		    if texte.starts_with(">") {
			ident.push_str(&texte);
		    } else {
			sequence.push_str(&texte);
		    }
		}
	    }
	}
	(ident, sequence)
    }

    fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
    where P: AsRef<Path>, {
	let file = File::open(filename)?;
	Ok(io::BufReader::new(file).lines())
    }
}
