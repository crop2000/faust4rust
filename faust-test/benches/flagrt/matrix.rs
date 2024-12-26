/* ------------------------------------------------------------
author: "Grame"
copyright: "(c)GRAME 2006"
license: "BSD"
name: "matrix"
version: "1.0"
Code generated with Faust 2.76.0 (https://faust.grame.fr)
Compilation options: -a ./bench_template.rs -lang rust -ct 1 -cn Matrix -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */
mod flagrt {
    #![allow(clippy::all)]
    #![allow(unused_parens)]
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(non_upper_case_globals)]

    use faust_types::*;

    pub type FaustFloat = F32;
    mod ffi {
        use std::os::raw::c_float;
        // Conditionally compile the link attribute only on non-Windows platforms
        #[cfg_attr(not(target_os = "windows"), link(name = "m"))]
        extern "C" {
            pub fn remainderf(from: c_float, to: c_float) -> c_float;
            pub fn rintf(val: c_float) -> c_float;
        }
    }
    fn remainder_f32(from: f32, to: f32) -> f32 {
        unsafe { ffi::remainderf(from, to) }
    }
    fn rint_f32(val: f32) -> f32 {
        unsafe { ffi::rintf(val) }
    }

    pub const FAUST_INPUTS: usize = 4;
    pub const FAUST_OUTPUTS: usize = 4;
    pub const FAUST_ACTIVES: usize = 0;
    pub const FAUST_PASSIVES: usize = 0;

    #[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
    #[repr(C)]
    pub struct Matrix {
        fSampleRate: i32,
    }

    impl Matrix {
        pub fn new() -> Matrix {
            Matrix { fSampleRate: 0 }
        }
        pub fn metadata(&self, m: &mut dyn Meta) {
            m.declare("author", r"Grame");
            m.declare("compile_options", r"-a ./bench_template.rs -lang rust -ct 1 -cn Matrix -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
            m.declare("copyright", r"(c)GRAME 2006");
            m.declare("filename", r"matrix.dsp");
            m.declare("license", r"BSD");
            m.declare("math.lib/author", r"GRAME");
            m.declare("math.lib/copyright", r"GRAME");
            m.declare("math.lib/deprecated", r"This library is deprecated and is not maintained anymore. It will be removed in August 2017.");
            m.declare("math.lib/license", r"LGPL with exception");
            m.declare("math.lib/name", r"Math Library");
            m.declare("math.lib/version", r"1.0");
            m.declare("music.lib/author", r"GRAME");
            m.declare("music.lib/copyright", r"GRAME");
            m.declare("music.lib/license", r"LGPL with exception");
            m.declare("music.lib/name", r"Music Library");
            m.declare("music.lib/version", r"1.0");
            m.declare("name", r"matrix");
            m.declare("version", r"1.0");
        }

        fn get_sample_rate(&self) -> i32 {
            self.fSampleRate as i32
        }

        pub fn class_init(sample_rate: i32) {}
        pub fn instance_reset_params(&mut self) {}
        pub fn instance_clear(&mut self) {}
        pub fn instance_constants(&mut self, sample_rate: i32) {
            self.fSampleRate = sample_rate;
        }
        pub fn instance_init(&mut self, sample_rate: i32) {
            self.instance_constants(sample_rate);
            self.instance_reset_params();
            self.instance_clear();
        }
        pub fn init(&mut self, sample_rate: i32) {
            Matrix::class_init(sample_rate);
            self.instance_init(sample_rate);
        }

        pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
            Self::build_user_interface_static(ui_interface);
        }

        pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
            ui_interface.open_vertical_box("matrix");
            ui_interface.close_box();
        }

        pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
            match param.0 {
                _ => None,
            }
        }

        pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
            match param.0 {
                _ => {}
            }
        }

        pub fn compute_arrays(
            &mut self,
            count: usize,
            inputs: &[&[FaustFloat]; 4],
            outputs: &mut [&mut [FaustFloat]; 4],
        ) {
            let [inputs0, inputs1, inputs2, inputs3] = inputs;
            let [outputs0, outputs1, outputs2, outputs3] = outputs;
            let inputs0 = inputs0.iter();
            let inputs1 = inputs1.iter();
            let inputs2 = inputs2.iter();
            let inputs3 = inputs3.iter();
            let outputs0 = outputs0.iter_mut();
            let outputs1 = outputs1.iter_mut();
            let outputs2 = outputs2.iter_mut();
            let outputs3 = outputs3.iter_mut();
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(inputs2)
                .zip(inputs3)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3)
                .take(count);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0;
                *output1 = *input1;
                *output2 = *input2;
                *output3 = *input3;
            }
        }

        pub fn compute(
            &mut self,
            count: usize,
            inputs: &[&[FaustFloat]],
            outputs: &mut [&mut [FaustFloat]],
        ) {
            let [inputs0, inputs1, inputs2, inputs3] = inputs else {
                panic!("wrong number of inputs");
            };
            let [outputs0, outputs1, outputs2, outputs3] = outputs else {
                panic!("wrong number of outputs");
            };
            let inputs0 = &inputs0[..count];
            let inputs1 = &inputs1[..count];
            let inputs2 = &inputs2[..count];
            let inputs3 = &inputs3[..count];
            let outputs0 = &mut outputs0[..count];
            let outputs1 = &mut outputs1[..count];
            let outputs2 = &mut outputs2[..count];
            let outputs3 = &mut outputs3[..count];
            let input_array = [inputs0, inputs1, inputs2, inputs3];
            let mut output_array = [outputs0, outputs1, outputs2, outputs3];
            self.compute_arrays(count, &input_array, &mut output_array);
        }

        pub fn compute_old(
            &mut self,
            count: usize,
            inputs: &[&[FaustFloat]],
            outputs: &mut [&mut [FaustFloat]],
        ) {
            let [inputs0, inputs1, inputs2, inputs3] = inputs else {
                panic!("wrong number of inputs");
            };
            let [outputs0, outputs1, outputs2, outputs3] = outputs else {
                panic!("wrong number of outputs");
            };
            let inputs0 = &inputs0[..count];
            let inputs1 = &inputs1[..count];
            let inputs2 = &inputs2[..count];
            let inputs3 = &inputs3[..count];
            let outputs0 = &mut outputs0[..count];
            let outputs1 = &mut outputs1[..count];
            let outputs2 = &mut outputs2[..count];
            let outputs3 = &mut outputs3[..count];
            let inputs0 = inputs0.iter();
            let inputs1 = inputs1.iter();
            let inputs2 = inputs2.iter();
            let inputs3 = inputs3.iter();
            let outputs0 = outputs0.iter_mut();
            let outputs1 = outputs1.iter_mut();
            let outputs2 = outputs2.iter_mut();
            let outputs3 = outputs3.iter_mut();
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(inputs2)
                .zip(inputs3)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3)
                .take(count);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0;
                *output1 = *input1;
                *output2 = *input2;
                *output3 = *input3;
            }
        }

        pub fn compute_fixed(
            &mut self,
            count: usize,
            inputs: &[&[FaustFloat]],
            outputs: &mut [&mut [FaustFloat]],
        ) {
            let [inputs0, inputs1, inputs2, inputs3] = inputs else {
                panic!("wrong number of inputs");
            };
            let [outputs0, outputs1, outputs2, outputs3] = outputs else {
                panic!("wrong number of outputs");
            };

            let inputs0 = inputs0.iter();
            let inputs1 = inputs1.iter();
            let inputs2 = inputs2.iter();
            let inputs3 = inputs3.iter();

            let outputs0 = outputs0.iter_mut();
            let outputs1 = outputs1.iter_mut();
            let outputs2 = outputs2.iter_mut();
            let outputs3 = outputs3.iter_mut();
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(inputs2)
                .zip(inputs3)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3)
                .take(count);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0;
                *output1 = *input1;
                *output2 = *input2;
                *output3 = *input3;
            }
        }
    }

    impl FaustDsp for Matrix {
        type T = FaustFloat;
        fn new() -> Self
        where
            Self: Sized,
        {
            Self::new()
        }
        fn metadata(&self, m: &mut dyn Meta) {
            self.metadata(m)
        }
        fn get_sample_rate(&self) -> i32 {
            self.get_sample_rate()
        }
        fn get_num_inputs(&self) -> i32 {
            FAUST_INPUTS as i32
        }
        fn get_num_outputs(&self) -> i32 {
            FAUST_OUTPUTS as i32
        }
        fn class_init(sample_rate: i32)
        where
            Self: Sized,
        {
            Self::class_init(sample_rate);
        }
        fn instance_reset_params(&mut self) {
            self.instance_reset_params()
        }
        fn instance_clear(&mut self) {
            self.instance_clear()
        }
        fn instance_constants(&mut self, sample_rate: i32) {
            self.instance_constants(sample_rate)
        }
        fn instance_init(&mut self, sample_rate: i32) {
            self.instance_init(sample_rate)
        }
        fn init(&mut self, sample_rate: i32) {
            self.init(sample_rate)
        }
        fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>) {
            self.build_user_interface(ui_interface)
        }
        fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
        where
            Self: Sized,
        {
            Self::build_user_interface_static(ui_interface);
        }
        fn get_param(&self, param: ParamIndex) -> Option<Self::T> {
            self.get_param(param)
        }
        fn set_param(&mut self, param: ParamIndex, value: Self::T) {
            self.set_param(param, value)
        }
        fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]) {
            self.compute(count as usize, inputs, outputs)
        }
    }
}

pub use flagrt::Matrix;

use criterion::{measurement::WallTime, BenchmarkGroup};
use std::hint::black_box;

pub fn run_benchmark(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = Matrix::new();
    dsp.init(sample_rate);

    let num_inputs = flagrt::FAUST_INPUTS;
    let num_outputs = flagrt::FAUST_INPUTS;

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
        .collect::<Vec<&[flagrt::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flagrt::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    c.bench_function("flagrt_Matrix", |b| {
        b.iter(|| dsp.compute(black_box(buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_old(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = Matrix::new();
    dsp.init(sample_rate);

    let num_inputs = flagrt::FAUST_INPUTS;
    let num_outputs = flagrt::FAUST_INPUTS;

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
        .collect::<Vec<&[flagrt::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flagrt::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    c.bench_function("flagrt_Matrix_old", |b| {
        b.iter(|| dsp.compute_old(black_box(buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_fixed(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = Matrix::new();
    dsp.init(sample_rate);

    let num_inputs = flagrt::FAUST_INPUTS;
    let num_outputs = flagrt::FAUST_INPUTS;

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
        .collect::<Vec<&[flagrt::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flagrt::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    c.bench_function("flagrt_Matrix_fixed", |b| {
        b.iter(|| dsp.compute_fixed(black_box(buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_arr(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = Matrix::new();
    dsp.init(sample_rate);

    let num_inputs = flagrt::FAUST_INPUTS;
    let num_outputs = flagrt::FAUST_INPUTS;

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
        .collect::<Vec<&[flagrt::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flagrt::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    let input_array: &[&[f32]; flagrt::FAUST_INPUTS] = ib
        .split_at(flagrt::FAUST_INPUTS)
        .0
        .try_into()
        .expect("too few input buffers");
    let mut output_array: &mut [&mut [f32]; flagrt::FAUST_OUTPUTS] = ob
        .split_at_mut(flagrt::FAUST_OUTPUTS)
        .0
        .try_into()
        .expect("too few output buffers");

    c.bench_with_input("flagrt_Matrix_arr", &mut output_array, |b, output_array| {
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
