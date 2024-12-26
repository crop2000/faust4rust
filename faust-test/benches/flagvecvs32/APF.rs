/* ------------------------------------------------------------
name: "APF"
Code generated with Faust 2.76.0 (https://faust.grame.fr)
Compilation options: -a ./bench_template.rs -lang rust -ct 1 -cn Apf -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0 -vec -lv 0 -vs 32
------------------------------------------------------------ */
mod flagvecvs32 {
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
	fRec0_perm: [F32;4],
}

impl Apf {

		
	pub fn new() -> Apf { 
		Apf {
			fHslider0: 0.0,
			fSampleRate: 0,
			fConst0: 0.0,
			fHslider1: 0.0,
			fRec0_perm: [0.0;4],
		}
	}
	pub fn metadata(&self, m: &mut dyn Meta) { 
		m.declare("compile_options", r"-a ./bench_template.rs -lang rust -ct 1 -cn Apf -es 1 -mcd 16 -mdd 1024 -mdy 33 -single -ftz 0 -vec -lv 0 -vs 32");
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

	
	fn get_sample_rate(&self) -> i32 { self.fSampleRate as i32}
	
	pub fn class_init(sample_rate: i32) {
	}
	pub fn instance_reset_params(&mut self) {
		self.fHslider0 = 1e+03;
		self.fHslider1 = 1.0;
	}
	pub fn instance_clear(&mut self) {
		self.fRec0_perm.fill(0.0);
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
			0 => { self.fHslider0 = value }
			1 => { self.fHslider1 = value }
			_ => {}
		}
	}
	
	#[allow(non_snake_case)]
	#[allow(unused_mut)]
	
	
	pub fn compute_arrays(&mut self, count: usize, inputs: &[&[FaustFloat] ; 1], outputs: &mut [&mut [FaustFloat] ; 1]) {
		
		const vsize: i32 = 32;
		let [inputs0, ] = inputs;
		let inputs0 = inputs0[..count].chunks(vsize as usize);
		let [outputs0, ] = outputs;
		let outputs0 = outputs0[..count].chunks_mut(vsize as usize);
		let mut fSlow0: F32 = self.fConst0 * F32::max(0.0, self.fHslider0);
		let mut fSlow1: F32 = 2.0 * F32::cos(fSlow0);
		let mut fZec0: [F32;32] = [0.0;32];
		let mut fSlow2: F32 = 0.5 * (F32::sin(fSlow0) / F32::max(0.001, self.fHslider1));
		let mut fSlow3: F32 = 1.0 - fSlow2;
		let mut fSlow4: F32 = 1.0 / (fSlow2 + 1.0);
		let mut fRec0_tmp: [F32;36] = [0.0;36];
		/* Main loop */
		let zipped_iterators = inputs0.zip(outputs0);
		for (input0, output0) in zipped_iterators {
			/* Recursive loop 0 */
			/* Pre code */
			for j0 in 0..4 {
				fRec0_tmp[j0 as usize] = self.fRec0_perm[j0 as usize];
			}
			/* Compute code */
			for i in 0..output0.len() as i32 {
				fZec0[i as usize] = fSlow1 * fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 1))) as usize];
				fRec0_tmp[(i32::wrapping_add(4, i)) as usize] = input0[i as usize] - fSlow4 * (fSlow3 * fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] - fZec0[i as usize]);
			}
			/* Post code */
			for j1 in 0..4 {
				self.fRec0_perm[j1 as usize] = fRec0_tmp[(i32::wrapping_add(vsize, j1)) as usize];
			}
			/* Vectorizable loop 1 */
			/* Compute code */
			for i in 0..output0.len() as i32 {
				output0[i as usize] = fRec0_tmp[(i32::wrapping_add(4, i32::wrapping_sub(i, 2))) as usize] + fSlow4 * (fSlow3 * fRec0_tmp[(i32::wrapping_add(4, i)) as usize] - fZec0[i as usize]);
			}
		}
	}

	
	pub fn compute(&mut self, count: usize, inputs: & [& [FaustFloat] ], outputs: & mut[& mut[FaustFloat] ]) {
		let input_array = inputs.split_at(1).0.try_into().expect("too few input buffers");
		let output_array = outputs.split_at_mut(1).0.try_into().expect("too few output buffers");
		self.compute_arrays(count, input_array, output_array);
	}

}

impl FaustDsp for Apf {
	type T = FaustFloat;
	fn new() -> Self where Self: Sized {
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
	fn class_init(sample_rate: i32) where Self: Sized {
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
	fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>) where Self: Sized {
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

pub use flagvecvs32::Apf;

use std::hint::black_box;
use criterion::{
     measurement::WallTime, BenchmarkGroup,
};

pub fn run_benchmark(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = Apf::new();
    dsp.init(sample_rate);

    let num_inputs = flagvecvs32::FAUST_INPUTS;
    let num_outputs = flagvecvs32::FAUST_INPUTS;

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
    .collect::<Vec<&[flagvecvs32::FaustFloat]>>();
     
    let ib = 
        ibb.as_slice();
    
        let mut obb = 
        out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flagvecvs32::FaustFloat]>>();
        let ob = obb 
            .as_mut_slice();
    

    c.bench_function("flagvecvs32_Apf", |b| {
        b.iter(|| dsp.compute(black_box(buffer_size), black_box(ib), black_box(ob)))
    });
}


pub fn run_benchmark_arr(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = Apf::new();
    dsp.init(sample_rate);

    let num_inputs = flagvecvs32::FAUST_INPUTS;
    let num_outputs = flagvecvs32::FAUST_INPUTS;

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
        .collect::<Vec<&[flagvecvs32::FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [flagvecvs32::FaustFloat]>>();
    let ob = obb.as_mut_slice();

    let input_array = ib.split_at(1).0.try_into().expect("too few input buffers");

    c.bench_function("flagvecvs32_Apf_arr", |b| {
        b.iter(|| {
            let output_array = ob
                .split_at_mut(1)
                .0
                .try_into()
                .expect("too few output buffers");

            dsp.compute_arrays(
                black_box(buffer_size),
                black_box(input_array),
                black_box(output_array),
            );
        })
    });
}
