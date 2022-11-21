use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn main() {
    let f = File::open("input.txt").expect("Could not open file");
    let reader = BufReader::new(f);
    let depths = reader
            .lines()
            .map(|i| i.unwrap())
            .map(|i| u32::from_str(&i).expect("Could not parse string to u32"))
            .collect::<Vec<u32>>();

    let increases = depths.iter().zip(depths.iter().skip(1)).filter(|(a,b)|  b > a).count();
    println!("Number of increases {increases}");
    // // Keep a count of the increases
    // let mut increases = 0;
    // let mut previous = depths[0];
    // for i in 1..depths.len() {
    //     if depths[i] > previous {
    //         increases += 1;
    //     }
    //     previous = depths[i];
    // }
    // println!("Number of increases {increases}");
}
// Number of increases 1583