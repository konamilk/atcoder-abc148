use proconio::input;
#[allow(unused_imports)]
use proconio::source::auto::AutoSource;
#[allow(unused_imports)]
use proconio::marker::{Chars, Bytes};
#[allow(unused_imports)]
use num::integer::{sqrt, gcd, lcm};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};

fn main() {
    // let source = AutoSource::from("0");
    input!{
        // from source,
        a: usize,
        b: usize
    };

    let mut v = vec![true;3];
    v[a-1] = false;
    v[b-1] = false;

    for (i, &x) in v.iter().enumerate() {
        if x{
            println!("{}", i+1)
        }
    }
}
