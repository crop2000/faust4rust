mod <<moduleName>> {
#![allow(clippy::all)]
#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

use faust_types::*;
<<includeIntrinsic>>
<<includeclass>>

}

pub use <<moduleName>>::<<structName>>;

use std::hint::black_box;
use criterion::{
     measurement::WallTime, BenchmarkGroup,
};

pub fn run_benchmark(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = <<structName>>::new();
    dsp.init(sample_rate);

    let num_inputs = <<moduleName>>::FAUST_INPUTS;
    let num_outputs = <<moduleName>>::FAUST_INPUTS;

    // Prepare buffers
    let mut in_buffer = vec![vec![0.0; buffer_size]; num_inputs];
    let mut out_buffer = vec![vec![0.0; buffer_size]; num_outputs];

    // Set input buffers to fixed impulse responses
    (0..num_inputs).for_each(|c| {
        (0..buffer_size).for_each(|j| {
            in_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let ibb = in_buffer
    .iter()
    .map(|buffer| buffer.as_slice())
    .collect::<Vec<&[<<moduleName>>::FaustFloat]>>();
     
    let ib = 
        ibb.as_slice();
    
        let mut obb = 
        out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [<<moduleName>>::FaustFloat]>>();
        let ob = obb 
            .as_mut_slice();
    

    c.bench_function("<<moduleName>>_<<structName>>", |b| {
        b.iter(|| dsp.compute(black_box(buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_old(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = <<structName>>::new();
    dsp.init(sample_rate);

    let num_inputs = <<moduleName>>::FAUST_INPUTS;
    let num_outputs = <<moduleName>>::FAUST_INPUTS;

    // Prepare buffers
    let mut in_buffer = vec![vec![0.0; buffer_size]; num_inputs];
    let mut out_buffer = vec![vec![0.0; buffer_size]; num_outputs];

    // Set input buffers to fixed impulse responses
    (0..num_inputs).for_each(|c| {
        (0..buffer_size).for_each(|j| {
            in_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let ibb = in_buffer
    .iter()
    .map(|buffer| buffer.as_slice())
    .collect::<Vec<&[<<moduleName>>::FaustFloat]>>();
     
    let ib = 
        ibb.as_slice();
    
        let mut obb = 
        out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [<<moduleName>>::FaustFloat]>>();
        let ob = obb 
            .as_mut_slice();
    

    c.bench_function("<<moduleName>>_<<structName>>_old", |b| {
        b.iter(|| dsp.compute_old(black_box(buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_fixed(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = <<structName>>::new();
    dsp.init(sample_rate);

    let num_inputs = <<moduleName>>::FAUST_INPUTS;
    let num_outputs = <<moduleName>>::FAUST_INPUTS;

    // Prepare buffers
    let mut in_buffer = vec![vec![0.0; buffer_size]; num_inputs];
    let mut out_buffer = vec![vec![0.0; buffer_size]; num_outputs];

    // Set input buffers to fixed impulse responses
    (0..num_inputs).for_each(|c| {
        (0..buffer_size).for_each(|j| {
            in_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let ibb = in_buffer
    .iter()
    .map(|buffer| buffer.as_slice())
    .collect::<Vec<&[<<moduleName>>::FaustFloat]>>();
     
    let ib = 
        ibb.as_slice();
    
        let mut obb = 
        out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [<<moduleName>>::FaustFloat]>>();
        let ob = obb 
            .as_mut_slice();
    

    c.bench_function("<<moduleName>>_<<structName>>_fixed", |b| {
        b.iter(|| dsp.compute_fixed(black_box(buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_arr(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = <<structName>>::new();
    dsp.init(sample_rate);

    let num_inputs = <<moduleName>>::FAUST_INPUTS;
    let num_outputs = <<moduleName>>::FAUST_INPUTS;

    // Prepare buffers
    let mut in_buffer = vec![vec![0.0; buffer_size]; num_inputs];
    let mut out_buffer = vec![vec![0.0; buffer_size]; num_outputs];

    // Set input buffers to fixed impulse responses
    (0..num_inputs).for_each(|c| {
        (0..buffer_size).for_each(|j| {
            in_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let ibb = in_buffer
        .iter()
        .map(|buffer| buffer.as_slice())
        .collect::<Vec<&[<<moduleName>>::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [<<moduleName>>::FaustFloat]>>();
    let ob = obb.as_mut_slice();



    let input_array: &[&[f32]; <<moduleName>>::FAUST_INPUTS] = ib.split_at(<<moduleName>>::FAUST_INPUTS).0.try_into().expect("too few input buffers");
    let mut output_array: &mut [&mut [f32]; <<moduleName>>::FAUST_OUTPUTS] = ob
        .split_at_mut(<<moduleName>>::FAUST_OUTPUTS)
        .0
        .try_into()
        .expect("too few output buffers");

    c.bench_with_input("<<moduleName>>_<<structName>>_arr", &mut output_array, |b, output_array| {
        // let (input_array, output_array): (&[&[f32]; 1], &mut [&mut [f32]; 1]) = *inputs;
        b.iter(|| {
            dsp.compute_arrays(
                black_box(buffer_size),
                black_box(input_array),
                black_box(output_array),
            );
        })
    });

}
