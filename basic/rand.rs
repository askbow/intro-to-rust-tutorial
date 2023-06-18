pub fn _rand() -> usize {
    42
}

pub fn rand(seed: usize) -> usize {
    let mut x = seed; // mutable
    /*
    pseudo-random number
    doi:10.18637/jss.v008.i14
    */
    x ^= x << 21;
    x ^= x >> 35;
    x ^= x << 4;
    // last value is the return:
    x
}

fn main() {
    // format with a string literal
    println!("{}", rand(42));
}