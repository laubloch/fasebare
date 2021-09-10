// src/main.rs :

extern crate needleman_wunsch;

// use needleman_wunsch::fasta_files_mgt::fasta_open_read::fasta_open_read::get_filenames;
use needleman_wunsch::fasta_files_mgt::fasta_multiple_cmp::fasta_multiple_cmp::get_filenames;

fn main() {
    get_filenames();
}

