#![feature(iter_array_chunks)]

use std::fs;

fn main() {
    let (p1, p2) = run("input");
    print!("{} {}", p1, p2);
}

fn run(inp: &str) -> (i32, i32) {
    let contents: String = fs::read_to_string(inp).expect("input file cannot be read");
    let one = contents.split("\r\n").map(|s| {
        let (s1, s2) = s.split_at(s.len() / 2);
        let c = s1.chars().filter(|c| s2.contains(*c)).next().unwrap();
        match c {
            'a'..='z' => return c as i32 - 'a' as i32 + 1,
            'A'..='Z' => return c as i32 - 'A' as i32 + 27,
            _ => unreachable!(),
        };
    });

    let two = contents.lines().array_chunks::<3>().map(|[s1, s2, s3]| {
        let c = s1.chars().into_iter().filter(|c| s2.contains(*c) && s3.contains(*c)).next().unwrap();
        match c {
            'a'..='z' => return c as i32 - 'a' as i32 + 1,
            'A'..='Z' => return c as i32 - 'A' as i32 + 27,
            _ => unreachable!(),
        };
    });
    return (one.sum(), two.sum());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(run("test"), (157, 70));
    }
}
