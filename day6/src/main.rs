use std::fs;

fn main() {
    let (p1, p2) = run("input");
    print!("{} {}", p1, p2);
}

fn run(inp: &str) -> (usize, usize) {
    let contents: String = fs::read_to_string(inp).expect("input file cannot be read");
    let one = contents.as_bytes().windows(4).position(|slice| (1..slice.len())
    .all(|i| !slice[i..].contains(&slice[i - 1]))).unwrap() + 4;

    let two = contents.as_bytes().windows(14).position(|slice| (1..slice.len())
    .all(|i| !slice[i..].contains(&slice[i - 1]))).unwrap() + 14;

    return (one, two);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(run("test"), (11, 26));
    }
}
