#![feature(autodiff)]
use std::autodiff::*;

use foo::df1_lib;

// f(x) = x * x, f'(x) = 2.0 * x
// bar therefore returns (x * x, 2.0 * x)
#[autodiff_reverse(bar, Active, Active)]
fn foo(x: f32) -> f32 { x * x }

fn main() {
    dbg!(&df1_lib(3.14, 1.0));
    assert_eq!(bar(3.0, 1.0), (9.0, 6.0));
    assert_eq!(bar(4.0, 1.0), (16.0, 8.0));
    dbg!(&bar(4.0, 1.0));
}
