use std::fs;
use std::io;
use std::process;
use std::ffi;

use term_size;

use super::filetype;
use crate::parser;

const SEP: &str = " ";
const SEP_LEN: usize = SEP.len();


fn get_max_per_line(longest_name_len: usize) -> usize {
    if let Some((width, _)) = term_size::dimensions() {
        width / (longest_name_len + SEP_LEN)
    } else {
        eprintln!("Failed to get current term's size");
        process::exit(1);
    }
}
fn get_metrics(dir_entries: &Vec<filetype::Entry>) -> (usize, usize) {
    let mut total_len: usize = SEP_LEN * (dir_entries.len() - 1);
    let mut longest_name_len: usize = 0;
    for entry in dir_entries.iter() {
        let filename_len: usize = entry.name.len();
        total_len += filename_len;
        if longest_name_len < filename_len {
            longest_name_len = filename_len;
        }
    }
    (total_len, longest_name_len)
}

fn read_dirs_to_entry(read_dir: fs::ReadDir) -> (Vec<filetype::Entry>, Vec<io::Error>) {
    let mut errors: Vec<io::Error> = Vec::new();
    let mut entries: Vec<filetype::Entry> = Vec::new();

    for read_dir_entry in read_dir.into_iter() {
        match read_dir_entry {
            Ok(dir_entry) => {
                let entry: filetype::Entry = filetype::Entry::from(dir_entry);
                entries.push(entry);
            },
            Err(error) => errors.push(error),
        }
    }
    entries.sort();

    (entries, errors)
}

fn show_one_line(entries: Vec<filetype::Entry>, errors: Vec<io::Error>, config: &parser::Config) {
    for entry in entries {
        let formatted_name: ffi::OsString = entry.get_formatted_name(0, &config);
        if let Some(str_name) = formatted_name.to_str() {
            print!("{}", str_name);
        } else {
            print!("{}", formatted_name.to_string_lossy());
        }
        print!("{}", SEP);
    }
    for error in errors {
        println!("{}", error);
    }
}


/// Pretty prints out read_dirs 
pub fn read_dir(config: &parser::Config, read_dir: fs::ReadDir) {
    let (entries, errors): (Vec<filetype::Entry>, Vec<io::Error>) = read_dirs_to_entry(read_dir);
    let (total_len, longest_name_len): (usize, usize) = get_metrics(&entries);
    let max_per_line: usize = get_max_per_line(longest_name_len);

    println!("max: {}, len = {}", max_per_line, entries.len());

    if entries.len() < max_per_line {
        show_one_line(entries, errors, config);
    }

}