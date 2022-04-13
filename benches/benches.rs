#![feature(test)]

extern crate test;
use thunk_simple::*;

#[bench]
fn thunk_nesting_sum_1000(b: &mut test::Bencher) {
    b.iter(|| {
        test::black_box((0..1000).fold(Thunk::<'_,u32>::new_const(0),|prev_thunk: Thunk<u32>,curr_u32|
            prev_thunk.map(move |prev_val| curr_u32 + prev_val))
        .unwrap())
    })
}
#[bench]
fn thunk_nesting_void_1000(b: &mut test::Bencher) {
    b.iter(|| {
        test::black_box((0..1000).fold(Thunk::<'_,()>::new_const(()),|prev_thunk: Thunk<()>,_|
            prev_thunk.map(|prev_val| prev_val))
        .unwrap())
    })
}