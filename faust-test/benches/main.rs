#![allow(static_mut_refs)]
pub mod flag;
// pub mod flagnoarr;
// pub mod flagnoarrt;
// pub mod flagt;
pub mod flagrt;
// pub mod flagvecvs32;
// pub mod flagvecvs4;
// pub mod flagvecvs64;

use criterion::{criterion_group, criterion_main, measurement::WallTime, Criterion};

fn bench_flags(c: &mut Criterion) {
    let a: Vec<for<'a> fn(&'a mut criterion::BenchmarkGroup<WallTime>)> = vec![
        // flag::comb_delay2_edited_static::run_benchmark,
        // flag::comb_delay2::run_benchmark,
        // flag::matrix_diy::run_benchmark_inpl,
        flag::matrix_diy::run_benchmark_new_ref,
        flag::matrix_diy::run_benchmark_new_borrow,
        flag::matrix_diy::run_benchmark_new_borrow2,
        // flag::matrix_diy::run_benchmark_new_ptr,
        // flag::matrix_diy::run_benchmark_fixed_same_buffer,
        // flag::matrix_diy::run_benchmark,
        // flag::matrix_diy::run_benchmark_fixed,
        // flag::matrix_diy::run_benchmark_arr,
        flag::matrix_diy::run_benchmark_old,
        // flagrt::matrix::run_benchmark_arr,
        // flagrt::matrix::run_benchmark,
        // flagnoarr::APF::run_benchmark,
        // flagnoarrt::APF::run_benchmark,
        // flagt::APF::run_benchmark,
        // flagvecvs4::APF::run_benchmark,
        // flagvecvs32::APF::run_benchmark,
        // flagvecvs64::APF::run_benchmark,
    ];

    let mut group = c.benchmark_group("Faust_flags");
    for fp in a.iter() {
        fp(&mut group)
    }
    group.finish();
}

criterion_group!(benches, bench_flags);
criterion_main!(benches);

// use iai::{black_box, main};
// fn iai_old() {
//     black_box(flag::matrix::iai_benchmark_old());
// }
// mod iai_wrappers {
//     pub fn iai_old() {
//         let _ = iai::black_box(super::iai_old());
//     }
// }
// fn main() {
//     let benchmarks: &[&(&'static str, fn())] = &[&(stringify!(iai_old), iai_wrappers::iai_old)];
//     iai::runner(benchmarks);
// }
