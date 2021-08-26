// src/sequences_matrix/build_sequences_matrix.rs :

// This module was inspired by Vincent Esche's Seal crate,
// but simplified and much more basic, without mmap and so on.
// For pedagogic use.

pub mod build_sequences_matrix {

    use simple_matrix::Matrix;
    use std::str;
    use std::char;
    
    pub fn build_matrix(sequence1: (String, Vec<u8>),
			sequence2: (String, Vec<u8>),
			match_bonus: f32,
			gap_penalty: f32) {
//			-> Matrix<f32> {
	print_seq(&sequence1);
	print_seq(&sequence2);

	let l_seq1: usize = (sequence1.1).len();
	let l_seq2: usize = (sequence2.1).len();

	println!("Longueur première séquence : {} ", l_seq1);
	println!("Longueur seconde séquence : {} ", l_seq2);
	
	let mut the_mat: Matrix::<f32> = Matrix::new(l_seq2+1, l_seq1+1);

	init_matrix(&mut the_mat, l_seq2+1, l_seq1+1, 0.0);

	nw_matrix(&mut the_mat, l_seq2+1, l_seq1+1, match_bonus, gap_penalty, &sequence1.1, &sequence2.1);

	print_vector(&sequence1.1);
	    
	print_matrix(&the_mat, &sequence2.1, l_seq2+1, l_seq1+1);
    }

    pub fn nw_matrix(the_mat: &mut Matrix::<f32>,
		     lin: usize,
		     col: usize,
		     match_bonus: f32,
		     gap_penalty: f32,
		     seq1: &Vec<u8>,
		     seq2: &Vec<u8>) {
	for j in 1..col {
	    the_mat.set(0, j, gap_penalty * j as f32) ;
	}
	let mut score: f32 = 0.0;
	for i in 1..lin {
	    the_mat.set(i, 0, gap_penalty * i as f32) ;
	    for j in 1..col {
		if seq1[j-1] == seq2[i-1] {
		    score = match_bonus} else {
		    score = 0.0}
		the_mat.set(i, j, max3(the_mat.get(i-1,j-1).unwrap()
				       + score,
				       the_mat.get(i-1,j).unwrap()
				       + gap_penalty,
				       the_mat.get(i,j-1).unwrap()
				       + gap_penalty));
	    }
	}
    }

    pub fn max3(v1: f32, v2: f32, v3: f32) -> f32 {
	let tmp = f32::max(v2, v3);
	if v1 > tmp {
	    return v1 } else {
	    return tmp };
    }
    
    pub fn init_matrix(the_mat: &mut Matrix::<f32>, lin: usize, col: usize, val: f32) {
	for i in 0..lin {
	    for j in 0..col {
		the_mat.set(i, j, val) ;
	    }
	}
    }

// "print_seq" affiche une séquence selon différents formats.
    pub fn print_seq(sequence: &(String, Vec<u8>)) {
		println!("Ident : {:?}", sequence.0);
		println!("Séquence : {:?}", sequence.1);
		let sequence_str = str::from_utf8(&sequence.1).unwrap().to_string();
		println!("Séquence : {}", &sequence_str);
    }

    pub fn print_vector(the_vec: &Vec<u8>) {
	let vec_str = str::from_utf8(the_vec).unwrap().to_string();
	print!("{} ", "   ");
	for c in vec_str.chars() {
	    print!("{} ", c);
	}
	print!("{}", "   \n");
    }
    
    pub fn print_matrix(the_mat: &Matrix::<f32>, seq2: &Vec<u8>, lin: usize, col: usize) {
	for i in 0..lin {
	    if i > 0 {print!("{} ", char::from(seq2[i-1]))} else
	    {print!("{} ", " ")};
	    for j in 0..col {
		print!("{} ", the_mat.get(i, j).unwrap());
	    }
	    print!("{}", "\n")
	}
    }
}
