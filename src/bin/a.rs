#[allow(unused_imports)]
use proconio::marker::{Bytes, Chars, Usize1};
use proconio::{fastout, input};
#[fastout]
fn main() {
    input! {
        n: isize,
    }
    if n >= 30 {
        println!("Yes");
    } else {
        println!("No");
    }
}
