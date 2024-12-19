use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

// fn main() {
//     let mut v1 = Vec::new();
//     let mut v2 = Vec::new();
//     if let Ok(lines) = read_lines("./2.txt") {
//         // Consumes the iterator, returns an (Optional) String
//         for line in lines.flatten() {
//             let w: Vec<_> = line.split_whitespace().collect();
//             v1.push(w[0].parse::<i32>().unwrap());
//             v2.push(w[1].parse::<i32>().unwrap());
//         }
//     }
//     v1.sort();
//     v2.sort();

//     part1(&v1, &v2);
//     part2(&v1, &v2);
//     ()
// }

fn part1(v1: &[i32], v2: &[i32]) {
    let diff = zip(v1.iter(), v2.iter()).fold(0, |acc, (&a, &b)| acc + (a - b).abs());
    dbg!(diff);
}

fn part2(v1: &[i32], v2: &[i32]) {
    let score = v1.iter().copied().reduce(|score, a| {
        score
            + a * v2
                .iter()
                .copied()
                .reduce(|acc, x| if a == x { acc + 1 } else { acc })
                .unwrap()
    });
    dbg!(score);
}
