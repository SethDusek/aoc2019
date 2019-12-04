use std::io::prelude::*;
use std::io::stdin;

fn calc_fuelr(mut mass: i32) -> i32 {
    let mut f = 0;
    loop {
        mass = (mass / 3) - 2;
        if mass <= 0 {
            break;
        }
        f+=mass;
    }
    f
}

fn main() {
    let t1 = std::time::Instant::now();
    let mut buf = String::new();
    std::fs::File::open("input.txt").unwrap().read_to_string(&mut buf).unwrap();
    let tfuel: i32 = buf.lines()
        .map(str::parse)
        .filter_map(Result::ok)
        .map(calc_fuelr)
        .sum();
    dbg!(std::time::Instant::now()-t1);
    println!("{}", tfuel);
}
