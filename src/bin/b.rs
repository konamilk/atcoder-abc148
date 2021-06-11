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
        n: usize,
        s: Chars,
        t: Chars
    };

    let mut ans = vec![];

    for i in 0..n{
        ans.push(s[i]);
        ans.push(t[i]);
    }

    println!("{}", ans.iter().collect::<String>())
}
