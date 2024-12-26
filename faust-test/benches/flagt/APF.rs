/* ------------------------------------------------------------
name: "APF"
Code generated with Faust 2.76.0 (https://faust.grame.fr)
Compilation options: -a ./bench_template.rs -lang rust -ct 1 -cn Apf -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0
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

    use faust_types::*;

    pub type FaustFloat = F32;
    use std::convert::TryInto;
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

    pub const FAUST_INPUTS: usize = 1;
    pub const FAUST_OUTPUTS: usize = 1;
    pub const FAUST_ACTIVES: usize = 2;
    pub const FAUST_PASSIVES: usize = 0;

    #[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
    #[repr(C)]
    pub struct Apf {
        fHslider0: F32,
        fSampleRate: i32,
        fConst0: F32,
        fHslider1: F32,
        fRec0: [F32; 3],
    }

    impl Apf {
        pub fn new() -> Apf {
            Apf {
                fHslider0: 0.0,
                fSampleRate: 0,
                fConst0: 0.0,
                fHslider1: 0.0,
                fRec0: [0.0; 3],
            }
        }
        pub fn metadata(&self, m: &mut dyn Meta) {
            m.declare("compile_options", r"-a ./bench_template.rs -lang rust -ct 1 -cn Apf -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0");
            m.declare("filename", r"APF.dsp");
            m.declare("maths.lib/author", r"GRAME");
            m.declare("maths.lib/copyright", r"GRAME");
            m.declare("maths.lib/license", r"LGPL with exception");
            m.declare("maths.lib/name", r"Faust Math Library");
            m.declare("maths.lib/version", r"2.8.0");
            m.declare("maxmsp.lib/author", r"GRAME");
            m.declare("maxmsp.lib/copyright", r"GRAME");
            m.declare("maxmsp.lib/license", r"LGPL with exception");
            m.declare("maxmsp.lib/name", r"MaxMSP compatibility Library");
            m.declare("maxmsp.lib/version", r"1.1.0");
            m.declare("name", r"APF");
            m.declare("platform.lib/name", r"Generic Platform Library");
            m.declare("platform.lib/version", r"1.3.0");
        }

        fn get_sample_rate(&self) -> i32 {
            self.fSampleRate as i32
        }

        pub fn class_init(sample_rate: i32) {}
        pub fn instance_reset_params(&mut self) {
            self.fHslider0 = 1e+03;
            self.fHslider1 = 1.0;
        }
        pub fn instance_clear(&mut self) {
            for l0 in 0..3 {
                self.fRec0[l0 as usize] = 0.0;
            }
        }
        pub fn instance_constants(&mut self, sample_rate: i32) {
            self.fSampleRate = sample_rate;
            self.fConst0 = 6.2831855 / F32::min(1.92e+05, F32::max(1.0, (self.fSampleRate) as F32));
        }
        pub fn instance_init(&mut self, sample_rate: i32) {
            self.instance_constants(sample_rate);
            self.instance_reset_params();
            self.instance_clear();
        }
        pub fn init(&mut self, sample_rate: i32) {
            Apf::class_init(sample_rate);
            self.instance_init(sample_rate);
        }

        pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
            Self::build_user_interface_static(ui_interface);
        }

        pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
            ui_interface.open_vertical_box("APF");
            ui_interface.add_horizontal_slider("Freq", ParamIndex(0), 1e+03, 1e+02, 1e+04, 1.0);
            ui_interface.add_horizontal_slider("Q", ParamIndex(1), 1.0, 0.01, 1e+02, 0.01);
            ui_interface.close_box();
        }

        pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
            match param.0 {
                0 => Some(self.fHslider0),
                1 => Some(self.fHslider1),
                _ => None,
            }
        }

        pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
            match param.0 {
                0 => self.fHslider0 = value,
                1 => self.fHslider1 = value,
                _ => {}
            }
        }

        #[inline]
        pub fn compute_arrays(
            &mut self,
            count: usize,
            inputs: &[&[FaustFloat]; 1],
            outputs: &mut [&mut [FaustFloat]; 1],
        ) {
            let [inputs0] = inputs;
            let inputs0 = inputs0.iter();
            let [outputs0] = outputs;
            let outputs0 = outputs0.iter_mut();
            let mut fSlow0: F32 = self.fConst0 * F32::max(0.0, self.fHslider0);
            let mut fSlow1: F32 = 2.0 * F32::cos(fSlow0);
            let mut fSlow2: F32 = 0.5 * (F32::sin(fSlow0) / F32::max(0.001, self.fHslider1));
            let mut fSlow3: F32 = 1.0 - fSlow2;
            let mut fSlow4: F32 = 1.0 / (fSlow2 + 1.0);
            let zipped_iterators = inputs0.zip(outputs0).take(count);
            for (input0, output0) in zipped_iterators {
                let mut fTemp0: F32 = fSlow1 * self.fRec0[1];
                self.fRec0[0] = *input0 - fSlow4 * (fSlow3 * self.fRec0[2] - fTemp0);
                *output0 = self.fRec0[2] + fSlow4 * (fSlow3 * self.fRec0[0] - fTemp0);
                self.fRec0[2] = self.fRec0[1];
                self.fRec0[1] = self.fRec0[0];
            }
        }

        pub fn compute(
            &mut self,
            count: usize,
            inputs: &[&[FaustFloat]],
            outputs: &mut [&mut [FaustFloat]],
        ) {
            let input_array = inputs
                .split_at(1)
                .0
                .try_into()
                .expect("too few input buffers");
            let output_array = outputs
                .split_at_mut(1)
                .0
                .try_into()
                .expect("too few output buffers");
            self.compute_arrays(count, input_array, output_array);
        }
    }

    impl FaustDsp for Apf {
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

pub use flag::Apf;

use criterion::{measurement::WallTime, BenchmarkGroup};
use std::hint::black_box;

pub fn run_benchmark(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = Apf::new();
    dsp.init(sample_rate);

    let num_inputs = flag::FAUST_INPUTS;
    let num_outputs = flag::FAUST_INPUTS;

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
        .collect::<Vec<&[flag::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    c.bench_function("flagt_Apf", |b| {
        b.iter(|| dsp.compute(black_box(buffer_size), black_box(ib), black_box(ob)))
    });
}

pub fn run_benchmark_arr(c: &mut criterion::BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = Apf::new();
    dsp.init(sample_rate);

    let num_inputs = flag::FAUST_INPUTS;
    let num_outputs = flag::FAUST_INPUTS;

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
        .collect::<Vec<&[flag::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flag::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    let input_array = ib.split_at(1).0.try_into().expect("too few input buffers");

    c.bench_function("flagt_Apf_arr", |b| {
        let output_array: &mut [&mut [flag::FaustFloat]; 1] = ob
            .split_at_mut(1)
            .0
            .try_into()
            .expect("too few output buffers");

        b.iter(|| {
            dsp.compute_arrays(
                black_box(buffer_size),
                black_box(input_array),
                black_box(output_array),
            );
        })
    });
}
