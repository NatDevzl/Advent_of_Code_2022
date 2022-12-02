use std::fs;

fn main() {
    let (p1, p2) = run("input");
    print!("{} {}", p1, p2);
}

fn run(inp: &str) -> (i32, i32) {
    // let mut outcome = 0;
    let contents: String = fs::read_to_string(inp).expect("input file cannot be read");
    let one = contents.split("\n").map(|s| { 
        let opp = s.chars().nth(0).unwrap() as i32 - 'A' as i32;
        let me = s.chars().nth(2).unwrap() as i32 - 'X' as i32;

        match me - opp {
            0 => return me + 3 + 1,
            1 | -2 => return me + 6 + 1,
            2 | -1 => return me + 1,
            _ => unreachable!(),
        }
    });


    let two = contents.split("\n").map(|s| { 
        let opp = s.chars().nth(0).unwrap() as i32 - 'A' as i32;
        let me = (opp + s.chars().nth(2).unwrap() as i32 - 'Y' as i32).rem_euclid(3);

        match me - opp {
            0 => return me + 3 + 1,
            1 | -2 => return me + 6 + 1,
            2 | -1 => return me + 1,
            _ => unreachable!(),
        }
    });

    return (one.sum(), two.sum());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_add() {
        assert_eq!(run("test"), (15, 12));
    }
}
