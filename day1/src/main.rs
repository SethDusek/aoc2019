use std::io::prelude::*;
use std::io::stdin;

fn calc_fuel(mass: f64) -> f64 {
    (mass / 3.0).floor() - 2.0
}
fn calc_fuelr(mut mass: i64) -> i64 {
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
    let mut tfuel = 0;
    buf.lines()
        .map(str::parse)
        .map(Result::ok)
        .filter_map(|p| p)
        .map(calc_fuelr)
        .for_each(|f| tfuel += f);
    let t2 = std::time::Instant::now()-t1;
    dbg!(t2);
    println!("{}", tfuel);
}
