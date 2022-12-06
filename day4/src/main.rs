use std::fs;

fn main() {
    let (p1, p2) = run("input");
    print!("{} {}", p1, p2);
}

fn run(inp: &str) -> (usize, usize) {
    let contents: String = fs::read_to_string(inp).expect("input file cannot be read");
    let one = contents.lines().map(|line| 
        line.split(&[',', '-'][..] )
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>())
        .filter(|n| (n[0] <= n[2] && n[1] >= n[3]) || (n[2] <= n[0] && n[3] >= n[1]));

    let two = contents.lines().map(|line| 
        line.split(&[',', '-'][..] )
        .map(|s| s.parse::<u32>().unwrap())
        .collect::<Vec<_>>())
        .filter(|n| (n[0] <= n[3] && n[1] >= n[2]));

    return (one.count() , two.count());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(run("test"), (2, 4));
    }
}
