use std::fs;

fn main() {
    let (p1, p2) = run("input");
    print!("p1 {}\np2 {}", p1, p2);
}

fn run(inp:&str) -> (i32, i32) {
    let contents: String = fs::read_to_string(inp).expect("input file cannot be read");

    let spl = contents.split("\r\n\r\n");

    let mut cal = spl
        .map(|s| {
            s.split("\r\n")
                .map(|x| x.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();
    cal.sort();
    let p1 = cal[cal.len() - 1];
    let p2 = cal[cal.len() - 3..cal.len()].iter().sum::<i32>();
    return (p1, p2)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(run("test"), (24000, 45000));
    }
}