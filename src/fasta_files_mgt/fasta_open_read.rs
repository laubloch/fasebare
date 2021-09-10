// src/fasta_files_mgt/fasta_open_read.rs :

// https://linuxfr.org/forums/programmationautre/posts/rust-lire-des-donnees-de-type-i8-depuis-un-fichier
// https://www.it-swarm-fr.com/fr/file-io/quelle-est-la-maniere-de-facto-de-lire-et-decrire-des-fichiers-dans-rust-1.x/1054845808/
// https://docs.rs/simple-matrix/0.1.2/simple_matrix/

pub mod fasta_open_read {

    use std::env;
    use std::fs;
    use std::fs::File;
    use std::io::Read;
    use std::fs::Metadata;
    use std::str;

    use crate::sequences_matrix::build_sequences_matrix::build_sequences_matrix::build_matrix;
    
    pub struct Config {
	pub filename1: String,
	pub filename2: String,
	pub match_bonus: f32,
	pub gap_penalty: f32
    }
    
    impl Config {
	pub fn new(args: &[String]) -> Config {
	    if args.len() < 5 {
		panic!("pas assez d'arguments");
	    }
	    let filename1 = args[1].clone();
	    let filename2 = args[2].clone();
	    let match_bonus: f32 = args[3].parse()
		.expect("Ce n'est pas un nombre !");
	    let gap_penalty: f32 = args[4].parse()
		.expect("Ce n'est pas un nombre !");
		
	    Config {filename1, filename2, match_bonus, gap_penalty}
	}
    }
	
    pub fn get_filenames() {
	let args: Vec<String> = env::args().collect();
	let config = Config::new(&args);
	
	println!("Alignement de {} avec {} \n", config.filename1, config.filename2);
	
        let mut data1 = fasta_open_file(config.filename1);
        let mut data2 = fasta_open_file(config.filename2);

        let sequences = read_seq(&mut data1, &mut data2);
	build_matrix(sequences.0,
		     sequences.1,
		     config.match_bonus,
		     config.gap_penalty);
    }

    fn fasta_open_file(filename: String) -> (File, Metadata) {
        let metadata = fs::metadata(&filename).expect("Fichier non trouvé.");
        let f = File::open(filename).expect("Fichier non trouvé !");
        (f, metadata)
    }

// "read_seq" lit chacun des deux fichiers au format FASTA et ses métadonnées.
    fn read_seq(data1: &mut (File, Metadata),
		data2: &mut (File, Metadata))
		-> ((String, Vec<u8>), (String, Vec<u8>)) {
        let mut seq1 = vec![0; data1.1.len() as usize];
        let mut seq2 = vec![0; data2.1.len() as usize];
        data1.0.read(&mut seq1).expect("débordement de buffer");
        data2.0.read(&mut seq2).expect("débordement de buffer");
	
	let seq1_str = str::from_utf8(&seq1).unwrap().to_string();
        let seq2_str = str::from_utf8(&seq2).unwrap().to_string();
	
	println!("1er caractère : {:?}", &seq1[0]);
	println!("1er caractère : {:?}", &seq1_str.bytes().nth(0));
	let sequence1 = split_seq(seq1);
	let sequence2 = split_seq(seq2);

	(sequence1, sequence2)
    }

// D'un fichier au format FASTA, extraire d'une part la première ligne, de commentaire,
// identifiée par le caractère ">" en première position, dont le texte documentera la
// séquence sous le nom "ident", d'autre part les lignes suivantes, qui contiennent la
// séquence proprement dite des nucléotides (ou des acides aminés), sous le nom
// "sequence_nuc". Après traitement de "sequence_nuc" par la fonction "remove_LF",
// la fonction "split_seq" renvoie le tuple "(ident, sequence_clean)".
    fn split_seq(fasta_sequence: Vec<u8>) -> (String, Vec<u8>) {
	    let mut ident: String = "abcd".to_string();
	    let mut sequence_nuc: Vec<u8> = vec![3, 4, 5];
	    if fasta_sequence[0] == 62 {
		let index = fasta_sequence.iter().position(|x| *x == 10).unwrap();
		println!("Position du premier LF : {}", index);
		ident = str::from_utf8(fasta_sequence.split_at(index).0).unwrap().to_string();
		sequence_nuc = (fasta_sequence.split_at(index).1).to_vec();
	    }
	let sequence_clean: Vec<u8> = remove_LF(&sequence_nuc);
	(ident, sequence_clean)
    }

// Cette fonction "remove_LF" reçoit les lignes de nucléotides ou d'acides aminés
// et en retire les caractères LF (10 selon le code Ascii) pour former une seule ligne.
    fn remove_LF(sequence_nuc: &Vec<u8>) -> Vec<u8> {
	let mut sequence_clean: Vec<u8> = (&sequence_nuc).to_vec();
	for i in (0..sequence_clean.len()).rev() {
            if sequence_clean[i] == 10 {
		sequence_clean.remove(i);
            }
	}
	sequence_clean
    }
}
