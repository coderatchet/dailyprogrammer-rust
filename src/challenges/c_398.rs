use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};

use regex::Regex;

//
// fn matrix_sum(matrix: &Vec<Vec<u32>>) -> Option<u32> {
// }

fn split_keep<'a>(r: &Regex, text: &'a str) -> Vec<&'a str> {
    let mut result = Vec::new();
    let mut last = 0;
    for (index, matched) in text.match_indices(r) {
        if last != index {
            result.push(&text[last..index]);
        }
        result.push(matched);
        last = index + matched.len();
    }
    if last < text.len() {
        result.push(&text[last..]);
    }
    result
}

fn load_matrix(file: &File) -> io::Result<Vec<Vec<u32>>> {
    let reader = BufReader::new(file);
    let mut vec: Vec<Vec<u32>> = Vec::new();
    for line in reader.lines() {
        let mut inner_vec: Vec<u32> = Vec::new();
        let regex = Regex::new(r"[,. ]+").expect("invalid regex");
        let words = line?;
        for word in split_keep(&regex, &words) {
            inner_vec.push(word.parse().expect("invalid number"));
        }
        vec.push(inner_vec);
    }
    Ok(vec)
}

#[cfg(test)]
mod tests {
    use std::fs::File;

    use super::*;

    #[test]
    fn matrix_is_loaded() {
        let five_by_five =
            File::open("resources/test/c_398_5_5.txt.rs").expect("could not read test file");
        let vec = load_matrix(&five_by_five);
        match vec {
            Ok(vec) => {
                assert_eq!(vec.len(), 5);
                for row in vec.iter() {
                    assert_eq!(row.len(), 5);
                    for num in row.iter() {
                        print!("{} ", num);
                    }
                    println!();
                }
            }
            Err(e) => {
                eprintln!("{:?}", e);
            }
        }
    }
}
