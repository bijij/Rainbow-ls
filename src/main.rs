/// Rainbow-ls listing files with a lot of colours
/// Copyright (C) 2020 - Saphielle Akiyama
///
/// This program is free software: you can redistribute it and/or modify
/// it under the terms of the GNU General Public License as published by
/// the Free Software Foundation, either version 3 of the License, or
/// (at your option) any later version.
///
/// This program is distributed in the hope that it will be useful,
/// but WITHOUT ANY WARRANTY; without even the implied warranty of
/// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
/// GNU General Public License for more details.
///
/// You should have received a copy of the GNU General Public License
/// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use std::env;
use std::path;

mod filetype;
mod display;
mod parse;

fn get_metrics(dir_entries: &Vec<filetype::Entry>) -> (usize, usize) {
    let mut total_length: usize = filetype::SEP_LEN * (dir_entries.len() - 1);
    let mut longest_name: usize = 0;
    for entry in dir_entries.iter() {
        let fn_len: usize = entry.file_name.len();
        total_length += fn_len;
        if longest_name < fn_len {
            longest_name = fn_len;
        }
    }
    (total_length, longest_name)
}

fn display_dir(dir: path::PathBuf, multiple_calls: bool) {

    if multiple_calls {
        if let Some(filename) = dir.file_name() {
            println!("{}", filename.to_str().unwrap_or("Unknown filename"));
        } else {
            println!("Unknown filename");
        }
    }
    
    let mut dir_entries: Vec<filetype::Entry> = dir
        .read_dir()
        .expect("Failed to read dir")
        .filter_map(Result::ok)
        .map(filetype::Entry::from_read_dir)
        .collect();

    dir_entries.sort();

    let term_width: usize = {
        if let Some(t_size) = term_size::dimensions() {
            t_size.0
        } else {
            panic!("Failed to get term's size")
        }
    };

    let (total_length, longest_name_length): (usize, usize) = get_metrics(&dir_entries);

    if total_length <= term_width {
        display::one_line(&dir_entries);
    } else {
        display::multiline(&mut dir_entries, longest_name_length, term_width);
    }

    if multiple_calls {
        println!("");
    }


}


#[allow(unreachable_code)]
fn main() {
    let config: parse::Config = parse::parse_args();

    let ok_dirs = config.ok_directories;
    let multiple_calls = ok_dirs.len() != 1;

    for dir in ok_dirs {
        display_dir(dir, multiple_calls);
    }



    

    
}
