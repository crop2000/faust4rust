/* ------------------------------------------------------------
author: "Grame"
copyright: "(c)GRAME 2006"
license: "BSD"
name: "matrix"
version: "1.0"
Code generated with Faust 2.76.0 (https://faust.grame.fr)
Compilation options: -a ./bench_template.rs -lang rust -ct 1 -cn Matrix -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
------------------------------------------------------------ */
mod flag {
    #![allow(clippy::all)]
    #![allow(unused_parens)]
    #![allow(non_snake_case)]
    #![allow(non_camel_case_types)]
    #![allow(dead_code)]
    #![allow(unused_variables)]
    #![allow(unused_mut)]
    #![allow(non_upper_case_globals)]

    use std::{
        borrow,
        slice::{from_raw_parts, from_raw_parts_mut},
    };

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
                .zip(outputs3);
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

        pub fn compute_inpl<OutType: borrow::BorrowMut<[FaustFloat]>>(
            &mut self,
            count: usize,
            mut ios: impl borrow::BorrowMut<[OutType]>,
        ) {
            let [ios0, ios1, ios2, ios3] = ios.borrow_mut() else {
                panic!("wrong number of ios");
            };
            let ios0 = &mut ios0.borrow_mut()[..count];
            let ios1 = &mut ios1.borrow_mut()[..count];
            let ios2 = &mut ios2.borrow_mut()[..count];
            let ios3 = &mut ios3.borrow_mut()[..count];
            let ios0 = ios0.iter_mut();
            let ios1 = ios1.iter_mut();
            let ios2 = ios2.iter_mut();
            let ios3 = ios3.iter_mut();
            let zipped_iterators = ios0.zip(ios1).zip(ios2).zip(ios3);
            for ((((io0), io1), io2), io3) in zipped_iterators {
                *io0 = *io0 * 0.9;
                *io1 = *io1 * 0.9;
                *io2 = *io2 * 0.9;
                *io3 = *io3 * 0.9;
            }
        }

        pub fn compute_new_ref<InType, OutType>(
            &mut self,
            count: usize,
            inputs: &(impl AsRef<[InType]> + ?Sized),
            outputs: &mut (impl AsMut<[OutType]> + ?Sized),
        ) where
            InType: AsRef<[FaustFloat]>,
            OutType: AsMut<[FaustFloat]>,
        {
            let [inputs0, inputs1, inputs2, inputs3] = inputs.as_ref() else {
                panic!("wrong number of inputs");
            };
            let [outputs0, outputs1, outputs2, outputs3] = outputs.as_mut() else {
                panic!("wrong number of outputs");
            };
            let inputs0 = inputs0.as_ref()[..count].iter();
            let inputs1 = inputs1.as_ref()[..count].iter();
            let inputs2 = inputs2.as_ref()[..count].iter();
            let inputs3 = inputs3.as_ref()[..count].iter();
            let outputs0 = outputs0.as_mut()[..count].iter_mut();
            let outputs1 = outputs1.as_mut()[..count].iter_mut();
            let outputs2 = outputs2.as_mut()[..count].iter_mut();
            let outputs3 = outputs3.as_mut()[..count].iter_mut();
            // let outputs0 = outputs0.iter_mut();
            // let outputs1 = outputs1.iter_mut();
            // let outputs2 = outputs2.iter_mut();
            // let outputs3 = outputs3.iter_mut();
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(inputs2)
                .zip(inputs3)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0;
                *output1 = *input1;
                *output2 = *input2;
                *output3 = *input3;
            }
        }

        pub fn compute_new_ref2<InType, OutType>(
            &mut self,
            count: usize,
            inputs: &(impl AsRef<[InType]> + ?Sized),
            mut outputs: &mut (impl AsMut<[OutType]> + ?Sized),
        ) where
            InType: AsRef<[FaustFloat]>,
            OutType: AsMut<[FaustFloat]>,
        {
            let [inputs0, inputs1, inputs2, inputs3] = inputs.as_ref() else {
                panic!("wrong number of inputs");
            };
            let [outputs0, outputs1, outputs2, outputs3] = outputs.as_mut() else {
                panic!("wrong number of outputs");
            };
            let inputs0 = inputs0.as_ref()[..count].iter();
            let inputs1 = inputs1.as_ref()[..count].iter();
            let inputs2 = inputs2.as_ref()[..count].iter();
            let inputs3 = inputs3.as_ref()[..count].iter();
            let outputs0 = outputs0.as_mut()[..count].iter_mut();
            let outputs1 = outputs1.as_mut()[..count].iter_mut();
            let outputs2 = outputs2.as_mut()[..count].iter_mut();
            let outputs3 = outputs3.as_mut()[..count].iter_mut();
            // let outputs0 = outputs0.iter_mut();
            // let outputs1 = outputs1.iter_mut();
            // let outputs2 = outputs2.iter_mut();
            // let outputs3 = outputs3.iter_mut();
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(inputs2)
                .zip(inputs3)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0;
                *output1 = *input1;
                *output2 = *input2;
                *output3 = *input3;
            }
        }

        pub fn compute_new_borrow2<InType, OutType>(
            &mut self,
            count: usize,
            inputs: &(impl borrow::Borrow<[InType]> + ?Sized),
            mut outputs: &mut (impl borrow::BorrowMut<[OutType]> + ?Sized),
        ) where
            InType: borrow::Borrow<[FaustFloat]>,
            OutType: borrow::BorrowMut<[FaustFloat]>,
        {
            let [inputs0, inputs1, inputs2, inputs3] = inputs.borrow() else {
                panic!("wrong number of inputs");
            };
            let [outputs0, outputs1, outputs2, outputs3] = outputs.borrow_mut() else {
                panic!("wrong number of outputs");
            };
            let inputs0 = inputs0.borrow()[..count].iter();
            let inputs1 = inputs1.borrow()[..count].iter();
            let inputs2 = inputs2.borrow()[..count].iter();
            let inputs3 = inputs3.borrow()[..count].iter();
            let outputs0 = outputs0.borrow_mut()[..count].iter_mut();
            let outputs1 = outputs1.borrow_mut()[..count].iter_mut();
            let outputs2 = outputs2.borrow_mut()[..count].iter_mut();
            let outputs3 = outputs3.borrow_mut()[..count].iter_mut();
            // let outputs0 = outputs0.iter_mut();
            // let outputs1 = outputs1.iter_mut();
            // let outputs2 = outputs2.iter_mut();
            // let outputs3 = outputs3.iter_mut();
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(inputs2)
                .zip(inputs3)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0;
                *output1 = *input1;
                *output2 = *input2;
                *output3 = *input3;
            }
        }

        pub fn compute_new_borrow<InType, OutType>(
            &mut self,
            count: usize,
            inputs: impl borrow::Borrow<[InType]>,
            mut outputs: impl borrow::BorrowMut<[OutType]>,
        ) where
            InType: borrow::Borrow<[FaustFloat]>,
            OutType: borrow::BorrowMut<[FaustFloat]>,
        {
            let [inputs0, inputs1, inputs2, inputs3] = inputs.borrow() else {
                panic!("wrong number of inputs");
            };
            let [outputs0, outputs1, outputs2, outputs3] = outputs.borrow_mut() else {
                panic!("wrong number of outputs");
            };
            let inputs0 = inputs0.borrow()[..count].iter();
            let inputs1 = inputs1.borrow()[..count].iter();
            let inputs2 = inputs2.borrow()[..count].iter();
            let inputs3 = inputs3.borrow()[..count].iter();
            let outputs0 = outputs0.borrow_mut()[..count].iter_mut();
            let outputs1 = outputs1.borrow_mut()[..count].iter_mut();
            let outputs2 = outputs2.borrow_mut()[..count].iter_mut();
            let outputs3 = outputs3.borrow_mut()[..count].iter_mut();
            let zipped_iterators = inputs0
                .zip(inputs1)
                .zip(inputs2)
                .zip(inputs3)
                .zip(outputs0)
                .zip(outputs1)
                .zip(outputs2)
                .zip(outputs3);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0;
                *output1 = *input1;
                *output2 = *input2;
                *output3 = *input3;
            }
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
                .zip(outputs3);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0;
                *output1 = *input1;
                *output2 = *input2;
                *output3 = *input3;
            }
        }

        pub fn compute_ptr(
            &mut self,
            count: usize,
            inputs: *mut *mut FaustFloat,
            outputs: *mut *mut FaustFloat,
        ) {
            let (inputs0, inputs1, inputs2, inputs3, outputs0, outputs1, outputs2, outputs3) = unsafe {
                let inputs0: &[f32] = from_raw_parts(*inputs.wrapping_add(0), count);
                let inputs1: &[f32] = from_raw_parts(*inputs.wrapping_add(1), count);
                let inputs2: &[f32] = from_raw_parts(*inputs.wrapping_add(2), count);
                let inputs3: &[f32] = from_raw_parts(*inputs.wrapping_add(3), count);
                let outputs0: &mut [f32] = from_raw_parts_mut(*outputs.wrapping_add(0), count);
                let outputs1: &mut [f32] = from_raw_parts_mut(*outputs.wrapping_add(1), count);
                let outputs2: &mut [f32] = from_raw_parts_mut(*outputs.wrapping_add(2), count);
                let outputs3: &mut [f32] = from_raw_parts_mut(*outputs.wrapping_add(3), count);
                (
                    inputs0, inputs1, inputs2, inputs3, outputs0, outputs1, outputs2, outputs3,
                )
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
                .zip(outputs3);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0 * 0.99;
                *output1 = *input1 * 0.99;
                *output2 = *input2 * 0.99;
                *output3 = *input3 * 0.99;
            }
        }

        // // this one failed because of duplicate borrow at first borrow of output0
        // pub fn compute_new<IT: std::borrow::Borrow<[f32]>, OT: std::borrow::BorrowMut<[f32]>>(
        //     &mut self,
        //     count: usize,
        //     inputs: impl std::borrow::Borrow<[Rc<RefCell<IT>>]>,
        //     mut outputs: impl std::borrow::BorrowMut<[Rc<RefCell<OT>>]>,
        // ) {
        //     let [inputs0, inputs1, inputs2, inputs3] = inputs.borrow() else {
        //         panic!("wrong number of inputs");
        //     };
        //     let [outputs0, outputs1, outputs2, outputs3] = outputs.borrow_mut() else {
        //         panic!("wrong number of outputs");
        //     };
        //     let inputs0 = inputs0.borrow();
        //     let inputs0 = inputs0.borrow()[..count].iter();
        //     let inputs1 = inputs1.borrow();
        //     let inputs1 = inputs1.borrow()[..count].iter();
        //     let inputs2 = inputs2.borrow();
        //     let inputs2 = inputs2.borrow()[..count].iter();
        //     let inputs3 = inputs3.borrow();
        //     let inputs3 = inputs3.borrow()[..count].iter();
        //     let mut outputs0 = outputs0.borrow_mut();
        //     let outputs0 = outputs0.borrow_mut()[..count].iter_mut();
        //     let mut outputs1 = outputs1.borrow_mut();
        //     let outputs1 = outputs1.borrow_mut()[..count].iter_mut();
        //     let mut outputs2 = outputs2.borrow_mut();
        //     let outputs2 = outputs2.borrow_mut()[..count].iter_mut();
        //     let mut binding = outputs3.borrow_mut();
        //     let outputs3 = binding.borrow_mut()[..count].iter_mut();
        //     let zipped_iterators = inputs0
        //         .zip(inputs1)
        //         .zip(inputs2)
        //         .zip(inputs3)
        //         .zip(outputs0)
        //         .zip(outputs1)
        //         .zip(outputs2)
        //         .zip(outputs3);
        //     for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
        //         zipped_iterators
        //     {
        //         *output0 = *input0;
        //         *output1 = *input1;
        //         *output2 = *input2;
        //         *output3 = *input3;
        //     }
        // }

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
                .zip(outputs3);
            for (((((((input0, input1), input2), input3), output0), output1), output2), output3) in
                zipped_iterators
            {
                *output0 = *input0 * 0.99;
                *output1 = *input1 * 0.99;
                *output2 = *input2 * 0.99;
                *output3 = *input3 * 0.99;
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

pub use flag::Matrix;

use criterion::{black_box, measurement::WallTime, BenchmarkGroup};
use std::{
    slice::{from_raw_parts, from_raw_parts_mut},
    sync::Mutex,
    thread::sleep,
    time::Duration,
};

pub fn run_benchmark(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    // Prepare buffers
    let mut lin_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    let mut lout_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    (0..flag::FAUST_INPUTS).for_each(|c| {
        (0..*buffer_size).for_each(|j| {
            lin_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let ibb = lin_buffer
        .iter()
        .map(|buffer| buffer.as_slice())
        .collect::<Vec<&[flag::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = lout_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    c.bench_function("flag_Matrix", |b| {
        b.iter(|| ldsp.compute(black_box(*buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_old(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    // Prepare buffers
    let mut lin_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    let mut lout_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    (0..flag::FAUST_INPUTS).for_each(|c| {
        (0..*buffer_size).for_each(|j| {
            lin_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let ibb = lin_buffer
        .iter()
        .map(|buffer| buffer.as_slice())
        .collect::<Vec<&[flag::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = lout_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    c.bench_function("flag_Matrix_old", |b| {
        b.iter(|| ldsp.compute_old(black_box(*buffer_size), black_box(ib), black_box(ob)))
    });
}

use lazy_static::lazy_static;

lazy_static! {
    static ref sample_rate: i32 = 48000;
    static ref buffer_size: usize = 1023;
    static ref dsp: Mutex<Matrix> = {
        let mut ldsp = Matrix::new();
        ldsp.init(*sample_rate);
        Mutex::new(ldsp)
    };
    static ref in_buffer: Mutex<Vec<Vec<f32>>> =
        Mutex::new(vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS]);
    static ref out_buffer: Mutex<Vec<Vec<f32>>> =
        Mutex::new(vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS]);
}

pub fn iai_benchmark_old() {
    // Prepare buffers

    // Set input buffers to fixed impulse responses

    // Compute

    let lib = in_buffer.lock().unwrap();
    let mut lob = out_buffer.lock().unwrap();
    let ibb = lib
        .iter()
        .map(|buffer| buffer.as_slice())
        .collect::<Vec<&[flag::FaustFloat]>>();
    let ib = ibb.as_slice();

    let mut obb = lob
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    // Generation constants

    dsp.lock()
        .unwrap()
        .compute_old(black_box(*buffer_size), black_box(ib), black_box(ob))
}

// pub fn run_benchmark_new(c: &mut BenchmarkGroup<WallTime>) {
//     // Generation constants
//     let sample_rate = 48000;
//     let buffer_size = 1024;

//     let mut dsp = Matrix::new();
//     dsp.init(sample_rate);

//     let flag::FAUST_INPUTS = flag::FAUST_INPUTS;
//     let flag::FAUST_OUTPUTS = flag::FAUST_INPUTS;

//     let mut buffers = vec![vec![0.0; buffer_size]; flag::FAUST_INPUTS + flag::FAUST_OUTPUTS];
//     // Prepare buffers
//     // let mut in_buffer = vec![vec![0.0; buffer_size]; flag::FAUST_INPUTS];
//     // let mut out_buffer = vec![vec![0.0; buffer_size]; flag::FAUST_OUTPUTS];

//     // Set input buffers to fixed impulse responses
//     buffers.iter_mut().for_each(|c| {
//         c.iter_mut().enumerate().for_each(|(i, j)| {
//             *j = if i == 0 { 1.0 } else { 0.0 };
//         });
//     });

//     // Compute

//     let bb = buffers
//         .iter_mut()
//         .map(|buffer| Rc::new(RefCell::new(buffer.as_mut_slice())))
//         .collect::<Vec<Rc<RefCell<&mut [flag::FaustFloat]>>>>();

//     // i am using on purpose here the same buffers a
//     let ib: Vec<Rc<RefCell<&mut [f32]>>> = bb.iter().take(flag::FAUST_INPUTS).cloned().collect();
//     let mut ob: Vec<Rc<RefCell<&mut [f32]>>> = bb.iter().take(flag::FAUST_OUTPUTS).cloned().collect();

//     c.bench_function("flag_Matrix_new", |b| {
//         b.iter(|| {
//             dsp.compute_new(
//                 black_box(buffer_size),
//                 black_box(ib.as_slice()),
//                 black_box(ob.as_mut_slice()),
//             )
//         })
//     });
// }

pub fn run_benchmark_inpl(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    let mut buffers = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS + flag::FAUST_OUTPUTS];
    // Prepare buffers
    // let mut in_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    // let mut out_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    buffers.iter_mut().for_each(|c| {
        c.iter_mut().enumerate().for_each(|(i, j)| {
            *j = if i == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let mut bb = buffers
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();

    // i am using on purpose here the same buffers a
    // let mut ib: Vec<*mut f32> = bb.iter().take(flag::FAUST_INPUTS).cloned().collect();
    // let mut ob: Vec<*mut f32> = bb.iter().take(flag::FAUST_OUTPUTS).cloned().collect();

    c.bench_function("flag_Matrix_inpl", |b| {
        b.iter(|| {
            let (_ib, ob) = bb.split_at_mut(flag::FAUST_INPUTS);
            ldsp.compute_inpl(black_box(*buffer_size), black_box(ob))
        })
    });
}

pub fn run_benchmark_new_borrow(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    let mut buffers = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS + flag::FAUST_OUTPUTS];
    // Prepare buffers
    // let mut in_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    // let mut out_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    buffers.iter_mut().for_each(|c| {
        c.iter_mut().enumerate().for_each(|(i, j)| {
            *j = if i == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let mut bb = buffers
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();

    // i am using on purpose here the same buffers a
    // let mut ib: Vec<*mut f32> = bb.iter().take(flag::FAUST_INPUTS).cloned().collect();
    // let mut ob: Vec<*mut f32> = bb.iter().take(flag::FAUST_OUTPUTS).cloned().collect();

    c.bench_function("flag_Matrix_new_borrow", |b| {
        b.iter(|| {
            let (ib, ob) = bb.split_at_mut(flag::FAUST_INPUTS);
            ldsp.compute_new_borrow(black_box(*buffer_size), black_box(ib), black_box(ob))
        })
    });
}

pub fn run_benchmark_new_borrow2(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    let mut buffers = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS + flag::FAUST_OUTPUTS];
    // Prepare buffers
    // let mut in_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    // let mut out_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    buffers.iter_mut().for_each(|c| {
        c.iter_mut().enumerate().for_each(|(i, j)| {
            *j = if i == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let mut bb = buffers
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();

    // i am using on purpose here the same buffers a
    // let mut ib: Vec<*mut f32> = bb.iter().take(flag::FAUST_INPUTS).cloned().collect();
    // let mut ob: Vec<*mut f32> = bb.iter().take(flag::FAUST_OUTPUTS).cloned().collect();

    c.bench_function("flag_Matrix_new_borrow2", |b| {
        b.iter(|| {
            let (ib, ob) = bb.split_at_mut(flag::FAUST_INPUTS);
            ldsp.compute_new_borrow2(black_box(*buffer_size), black_box(ib), black_box(ob))
        })
    });
}

pub fn run_benchmark_new_ref(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    let mut buffers = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS + flag::FAUST_OUTPUTS];
    // Prepare buffers
    // let mut in_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    // let mut out_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    buffers.iter_mut().for_each(|c| {
        c.iter_mut().enumerate().for_each(|(i, j)| {
            *j = if i == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let mut bb = buffers
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();

    // i am using on purpose here the same buffers a
    // let mut ib: Vec<*mut f32> = bb.iter().take(flag::FAUST_INPUTS).cloned().collect();
    // let mut ob: Vec<*mut f32> = bb.iter().take(flag::FAUST_OUTPUTS).cloned().collect();

    c.bench_function("flag_Matrix_new_ref", |b| {
        b.iter(|| {
            let (ib, ob) = bb.split_at_mut(flag::FAUST_INPUTS);
            ldsp.compute_new_ref(black_box(*buffer_size), black_box(ib), black_box(ob))
        })
    });
}

pub fn run_benchmark_new_ptr(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    let mut buffers = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS + flag::FAUST_OUTPUTS];
    // Prepare buffers
    // let mut in_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    // let mut out_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    buffers.iter_mut().for_each(|c| {
        c.iter_mut().enumerate().for_each(|(i, j)| {
            *j = if i == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let mut bb = buffers
        .iter_mut()
        .map(|buffer| buffer.as_mut_ptr())
        .collect::<Vec<*mut flag::FaustFloat>>();

    // i am using on purpose here the same buffers a
    // let mut ib: Vec<*mut f32> = bb.iter().take(flag::FAUST_INPUTS).cloned().collect();
    // let mut ob: Vec<*mut f32> = bb.iter().take(flag::FAUST_OUTPUTS).cloned().collect();

    let (ib, ob) = bb.split_at_mut(1);
    c.bench_function("flag_Matrix_new_ptr", |b| {
        b.iter(|| {
            ldsp.compute_ptr(
                black_box(*buffer_size),
                black_box(ib.as_mut_ptr()),
                black_box(ob.as_mut_ptr()),
            )
        })
    });
}

pub fn run_benchmark_fixed_same_buffer(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp: Matrix = Matrix::new();
    ldsp.init(*sample_rate);

    let mut lin_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    let mut lout_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    let mut buffers = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS + flag::FAUST_OUTPUTS];
    // Prepare buffers
    // let mut in_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    // let mut out_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    buffers.iter_mut().for_each(|c| {
        c.iter_mut().enumerate().for_each(|(i, j)| {
            *j = if i == 0 { 1.0 } else { 0.0 };
        });
    });

    // Set input buffers to fixed impulse responses
    (0..flag::FAUST_INPUTS).for_each(|c| {
        (0..*buffer_size).for_each(|j| {
            lin_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    let ibb: Vec<&[f32]> = buffers
        .iter_mut()
        .take(flag::FAUST_INPUTS)
        .map(|b| unsafe { from_raw_parts(b.as_slice().as_ptr(), *buffer_size) })
        .collect();
    let _ib = ibb.as_slice();

    let mut obb: Vec<&mut [f32]> = buffers
        .iter_mut()
        .take(flag::FAUST_OUTPUTS)
        .map(|b| unsafe { from_raw_parts_mut(b.as_mut_slice().as_mut_ptr(), *buffer_size) })
        .collect();
    let ob = obb.as_mut_slice();

    // let mut buffers = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS + flag::FAUST_OUTPUTS];
    let ibb3: Vec<&[f32]> = buffers
        .iter_mut()
        .take(flag::FAUST_INPUTS)
        .map(|b| unsafe { from_raw_parts(b.as_slice().as_ptr(), *buffer_size) })
        .collect();
    let ib3 = ibb3.as_slice();

    let mut obb3: Vec<&mut [f32]> = buffers
        .iter_mut()
        .take(flag::FAUST_OUTPUTS)
        .map(|b| unsafe { from_raw_parts_mut(b.as_mut_slice().as_mut_ptr(), *buffer_size) })
        .collect();
    let _ob3 = obb3.as_mut_slice();

    let ibb2 = lin_buffer
        .iter()
        .map(|buffer| buffer.as_slice())
        .collect::<Vec<&[flag::FaustFloat]>>();

    let _ib2 = ibb2.as_slice();

    let mut obb2 = lout_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();
    let _ob2 = obb2.as_mut_slice();

    sleep(Duration::from_secs(1));
    c.bench_function("flag_Matrix_fixed_same_buffer", |b| {
        b.iter(|| ldsp.compute_fixed(black_box(*buffer_size), black_box(ib3), black_box(ob)))
    });
    // c.bench_function("flag_Matrix_fixed2", |b| {
    //     b.iter(|| ldsp.compute_fixed(black_box(*buffer_size), black_box(ib), black_box(ob3)))
    // });
}

pub fn run_benchmark_fixed(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    // Prepare buffers
    let mut lin_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    let mut lout_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    (0..flag::FAUST_INPUTS).for_each(|c| {
        (0..*buffer_size).for_each(|j| {
            lin_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let ibb = lin_buffer
        .iter()
        .map(|buffer| buffer.as_slice())
        .collect::<Vec<&[flag::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = lout_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    c.bench_function("flag_Matrix_fixed", |b| {
        b.iter(|| ldsp.compute_fixed(black_box(*buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_arr(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants

    let mut ldsp = Matrix::new();
    ldsp.init(*sample_rate);

    // Prepare buffers
    let mut lin_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_INPUTS];
    let mut lout_buffer = vec![vec![0.0; *buffer_size]; flag::FAUST_OUTPUTS];

    // Set input buffers to fixed impulse responses
    (0..flag::FAUST_INPUTS).for_each(|c| {
        (0..*buffer_size).for_each(|j| {
            lin_buffer[c][j] = if j == 0 { 1.0 } else { 0.0 };
        });
    });

    // Compute

    let ibb = lin_buffer
        .iter()
        .map(|buffer| buffer.as_slice())
        .collect::<Vec<&[flag::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = lout_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    let input_array: &[&[f32]; flag::FAUST_INPUTS] = ib
        .split_at(flag::FAUST_INPUTS)
        .0
        .try_into()
        .expect("too few input buffers");
    let mut output_array: &mut [&mut [f32]; flag::FAUST_OUTPUTS] = ob
        .split_at_mut(flag::FAUST_OUTPUTS)
        .0
        .try_into()
        .expect("too few output buffers");

    c.bench_with_input("flag_Matrix_arr", &mut output_array, |b, output_array| {
        // let (input_array, output_array): (&[&[f32]; 1], &mut [&mut [f32]; 1]) = *inputs;
        b.iter(|| {
            ldsp.compute_arrays(
                black_box(*buffer_size),
                black_box(input_array),
                black_box(output_array),
            );
        })
    });
}
