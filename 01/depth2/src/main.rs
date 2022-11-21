use std::fs::File;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

fn load_depths(path: &str) -> Vec<u32> {
    let f = File::open(path).expect("Could not open file");
    let reader = BufReader::new(f);
    reader
        .lines()
        .map(|i| i.unwrap())
        .map(|i| u32::from_str(&i).expect("Could not parse string to u32"))
        .collect::<Vec<u32>>()
}

fn increases(depths: Vec<u32>) -> usize {
    depths.iter().zip(depths.iter().skip(1)).filter(|(a,b)|  b > a).count()
}

fn increases_over_range(depths: Vec<u32>) -> usize {
    let average_depths = depths.iter()
        .zip(depths.iter().skip(1))
        .zip(depths.iter().skip(2))
        .map(|((a,b),c)| a + b +c )
        .collect::<Vec<u32>>();

    // println!("{:?}", average_depths);
    increases(average_depths)
}

fn main() {
    let depths = load_depths("input.txt");
    let increases = increases_over_range(depths);
    println!("Number of increases {increases}");
}
// Number of increases 1627

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_increases_over_range_1() {
        let depths = load_depths("input2.txt");
        let increases = increases_over_range(depths);
        assert_eq!(increases, 5);
    }
}