use std::fs::File;
use std::io::{self, BufRead};
use std::iter::zip;
use std::path::Path;

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
