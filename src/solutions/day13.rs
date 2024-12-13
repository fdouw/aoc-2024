use good_lp::{constraint, default_solver, variables, Solution, SolverModel};
use regex::Regex;

pub fn solve(input: String, _verbose: bool) -> (String, String) {
    let pattern =
        r"(?m)Button A: X\+(\d+), Y\+(\d+)\nButton B: X\+(\d+), Y\+(\d+)\nPrize: X=(\d+), Y=(\d+)";
    let re = Regex::new(&pattern).unwrap();
    let mut scores = 0;
    let mut scores2 = 0;
    for (_, [ax, ay, bx, by, x, y]) in re.captures_iter(&input).map(|c| c.extract()) {
        let ax: u64 = ax.parse().unwrap();
        let ay: u64 = ay.parse().unwrap();
        let bx: u64 = bx.parse().unwrap();
        let by: u64 = by.parse().unwrap();
        let x: u64 = x.parse().unwrap();
        let y: u64 = y.parse().unwrap();

        // Part 1
        for i in 0.. {
            if x < bx * i || y < by * i {
                break;
            }
            if (x - bx * i) % ax == 0 && (y - by * i) % ay == 0 {
                let j = (x - bx * i) / ax;
                if j == (y - by * i) / ay && i <= 100 && j <= 100 {
                    // It seems there is 1 score at most
                    scores += i + 3 * j;
                    break;
                }
            }
        }

        // Part 2
        // let x = 10000000000000 + x;
        // let y = 10000000000000 + y;
        // let mut min_score = u64::MAX;
        // for i in 0.. {
        //     if x < bx * i || y < by * i {
        //         break;
        //     }
        //     if (x - bx * i) % ax == 0 && (y - by * i) % ay == 0 {
        //         let j = (x - bx * i) / ax;
        //         if j == (y - by * i) / ay {
        //             min_score = min_score.min(i + 3 * j);
        //         }
        //     }
        // }
        // if min_score < u64::MAX {
        //     scores2 += min_score;
        // }

        // // Part 2
        // let ax = ax as f64;
        // let bx = bx as f64;
        // let x = x as f64;
        // let ay = ay as f64;
        // let by = by as f64;
        // let y = y as f64;
        // variables! {
        //     problem: 0<=a; 0<= b;
        // }
        // let solution = problem
        //     .minimise(3 * a + b)
        //     .using(default_solver)
        //     .with(constraint!(a * ax + b * bx == x))
        //     .with(constraint!(a * ay + b * by == y))
        //     .with(constraint!(a <= 100))
        //     .with(constraint!(b <= 100))
        //     .solve();
        // match solution {
        //     Ok(s) => {
        //         println!("Found solution: {}, {}", s.value(a), s.value(b));
        //         scores2 += 3 * s.value(a) as u64 + s.value(b) as u64;
        //     }
        //     Err(_) => { /* Could not solve, but that happens */ }
        // }
    }

    (scores.to_string(), scores2.to_string())
}
