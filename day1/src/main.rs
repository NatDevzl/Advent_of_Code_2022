use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

fn main() {
    let mut max = 0;
    let mut sum = 0;
    let mut vec = Vec::new();
    let file = File::open("input.txt").expect("File not found");
    let buf_reader = BufReader::new(file);

    for line in buf_reader.lines() {
        let string = line.unwrap();
        if string.is_empty() {
            // add to vector
            vec.push(sum);
            sum = 0;
        }
        else {
            sum += string.parse::<i32>().unwrap(); 
        }
    }
    vec.push(sum);
    vec.sort();
    // sort vector en pak top 3
    println!("part 1 {}",vec[vec.len() - 1]);
    println!("part 2 {}",vec[vec.len() - 3 .. vec.len()].iter().sum::<i32>());
}