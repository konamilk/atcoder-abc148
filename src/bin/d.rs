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
    let source = AutoSource::from("10
    3 1 4 1 5 9 2 6 5 3");
    input!{
        // from source,
        n: usize,
        a: [usize; n]
    };


    let mut flg = false;
    let mut ans = 0;

    let mut i = 1;
    for j in 0..n {
        if a[j] == i {
            i += 1;
            flg = true

        }
        else {
            ans += 1;
        }
    }

    if flg {
        println!("{}", ans)
    }
    else {
        println!("-1")
    }

}
