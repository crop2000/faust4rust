/* ------------------------------------------------------------
name: "comb_delay2"
Code generated with Faust 2.76.0 (https://faust.grame.fr)
Compilation options: -a archs/rust/architecture_trait.rs -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
------------------------------------------------------------ */
/************************************************************************
************************************************************************
FAUST Architecture File
Copyright (C) 2020 GRAME, Centre National de Creation Musicale
---------------------------------------------------------------------

This is sample code. This file is provided as an example of minimal
FAUST architecture file. Redistribution and use in source and binary
forms, with or without modification, in part or in full are permitted.
In particular you can create a derived work of this FAUST architecture
and distribute that work under terms of your choice.

This sample code is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.
************************************************************************
************************************************************************/

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]
#![recursion_limit = "256"]

// extern crate libm;
// extern crate num_traits;
/* extern crate fastfloat; */

// use num_traits::{cast::FromPrimitive, float::Float};
use std::sync::{Arc, LazyLock, RwLock};
use std::{env, fs::File, io::Write};

type F32 = f32;
type F64 = f64;

/*
type F32 = Fast<f32>;
type F64 = Fast<f64>;
*/

#[derive(Copy, Clone)]
pub struct ParamIndex(i32);

pub struct Soundfile<'a> {
    fBuffers: &'a &'a F32,
    fLength: &'a i32,
    fSR: &'a i32,
    fOffset: &'a i32,
    fChannels: i32,
}

pub trait FaustDsp {
    type T;

    fn new() -> Self
    where
        Self: Sized;
    fn metadata(&self, m: &mut dyn Meta);
    fn get_sample_rate(&self) -> i32;
    fn get_num_inputs(&self) -> i32;
    fn get_num_outputs(&self) -> i32;
    fn class_init(sample_rate: i32)
    where
        Self: Sized;
    fn instance_reset_params(&mut self);
    fn instance_clear(&mut self);
    fn instance_constants(&mut self, sample_rate: i32);
    fn instance_init(&mut self, sample_rate: i32);
    fn init(&mut self, sample_rate: i32);
    fn build_user_interface(&self, ui_interface: &mut dyn UI<Self::T>);
    fn build_user_interface_static(ui_interface: &mut dyn UI<Self::T>)
    where
        Self: Sized;
    fn get_param(&self, param: ParamIndex) -> Option<Self::T>;
    fn set_param(&mut self, param: ParamIndex, value: Self::T);
    fn compute(&mut self, count: i32, inputs: &[&[Self::T]], outputs: &mut [&mut [Self::T]]);
}

pub trait Meta {
    // -- metadata declarations
    fn declare(&mut self, key: &str, value: &str);
}

pub trait UI<T> {
    // -- widget's layouts
    fn open_tab_box(&mut self, label: &str);
    fn open_horizontal_box(&mut self, label: &str);
    fn open_vertical_box(&mut self, label: &str);
    fn close_box(&mut self);

    // -- active widgets
    fn add_button(&mut self, label: &str, param: ParamIndex);
    fn add_check_button(&mut self, label: &str, param: ParamIndex);
    fn add_vertical_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_horizontal_slider(
        &mut self,
        label: &str,
        param: ParamIndex,
        init: T,
        min: T,
        max: T,
        step: T,
    );
    fn add_num_entry(&mut self, label: &str, param: ParamIndex, init: T, min: T, max: T, step: T);

    // -- passive widgets
    fn add_horizontal_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);
    fn add_vertical_bargraph(&mut self, label: &str, param: ParamIndex, min: T, max: T);

    // -- metadata declarations
    fn declare(&mut self, param: Option<ParamIndex>, key: &str, value: &str);
}

pub struct ButtonUI {
    all_button_params: Vec<ParamIndex>,
}

impl ButtonUI {
    fn set_button_parameters_to(&self, dsp: &mut dyn FaustDsp<T = f64>, value: f64) {
        for button_param in &self.all_button_params {
            dsp.set_param(*button_param, value);
        }
    }
}

// // Generated intrinsics:

// // Generated class:

pub type FaustFloat = F64;
use std::borrow;

pub struct mydspSIG0 {
    iRec0: [i32; 2],
}

impl mydspSIG0 {
    fn get_num_inputsmydspSIG0(&self) -> i32 {
        0
    }
    fn get_num_outputsmydspSIG0(&self) -> i32 {
        1
    }

    pub fn instance_initmydspSIG0(&mut self, sample_rate: i32) {
        for l0 in 0..2 {
            self.iRec0[l0 as usize] = 0;
        }
    }

    pub fn fillmydspSIG0(&mut self, count: i32, table: &mut [FaustFloat]) {
        for i1 in 0..count {
            self.iRec0[0] = i32::wrapping_add(self.iRec0[1], 1);
            table[i1 as usize] =
                F64::sin(9.587379924285257e-05 * (i32::wrapping_add(self.iRec0[0], -1)) as F64);
            self.iRec0[1] = self.iRec0[0];
        }
    }
}

pub fn newmydspSIG0() -> mydspSIG0 {
    mydspSIG0 { iRec0: [0; 2] }
}
// static mut ftbl0mydspSIG0: [F64; 65536] = [0.0; 65536];

static ftbl0mydspSIG0: LazyLock<Arc<RwLock<[F64; 65536]>>> =
    LazyLock::new(|| Arc::new(RwLock::new([0f64; 65536])));

mod ffi {
    use std::os::raw::c_double;
    // Conditionally compile the link attribute only on non-Windows platforms
    #[cfg_attr(not(target_os = "windows"), link(name = "m"))]
    extern "C" {
        pub fn remainder(from: c_double, to: c_double) -> c_double;
        pub fn rint(val: c_double) -> c_double;
    }
}
fn remainder_f64(from: f64, to: f64) -> f64 {
    unsafe { ffi::remainder(from, to) }
}
fn rint_f64(val: f64) -> f64 {
    unsafe { ffi::rint(val) }
}

pub const FAUST_INPUTS: usize = 0;
pub const FAUST_OUTPUTS: usize = 1;
pub const FAUST_ACTIVES: usize = 0;
pub const FAUST_PASSIVES: usize = 0;

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct mydsp {
    fSampleRate: i32,
    fConst0: F64,
    fConst1: F64,
    fRec1: [F64; 2],
    IOTA0: i32,
    fVec0: [F64; 128],
    fConst2: F64,
    fRec2: [F64; 2],
}

impl mydsp {
    pub fn new() -> mydsp {
        mydsp {
            fSampleRate: 0,
            fConst0: 0.0,
            fConst1: 0.0,
            fRec1: [0.0; 2],
            IOTA0: 0,
            fVec0: [0.0; 128],
            fConst2: 0.0,
            fRec2: [0.0; 2],
        }
    }
    pub fn metadata(&self, m: &mut dyn Meta) {
        m.declare("compile_options", r"-a archs/rust/architecture_trait.rs -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
        m.declare("filename", r"comb_delay2.dsp");
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
        m.declare("name", r"comb_delay2");
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.fSampleRate
    }

    pub fn class_init(sample_rate: i32) {
        let mut sig0: mydspSIG0 = newmydspSIG0();
        sig0.instance_initmydspSIG0(sample_rate);
        sig0.fillmydspSIG0(65536, &mut *ftbl0mydspSIG0.write().unwrap());
    }
    pub fn instance_reset_params(&mut self) {}
    pub fn instance_clear(&mut self) {
        for l1 in 0..2 {
            self.fRec1[l1 as usize] = 0.0;
        }
        self.IOTA0 = 0;
        for l2 in 0..128 {
            self.fVec0[l2 as usize] = 0.0;
        }
        for l3 in 0..2 {
            self.fRec2[l3 as usize] = 0.0;
        }
    }
    pub fn instance_constants(&mut self, sample_rate: i32) {
        self.fSampleRate = sample_rate;
        self.fConst0 = F64::min(1.92e+05, F64::max(1.0, (self.fSampleRate) as F64));
        self.fConst1 = 4.4e+02 / self.fConst0;
        self.fConst2 = 1e+03 / self.fConst0;
    }
    pub fn instance_init(&mut self, sample_rate: i32) {
        self.instance_constants(sample_rate);
        self.instance_reset_params();
        self.instance_clear();
    }
    pub fn init(&mut self, sample_rate: i32) {
        mydsp::class_init(sample_rate);
        self.instance_init(sample_rate);
    }

    pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
        Self::build_user_interface_static(ui_interface);
    }

    pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
        ui_interface.open_vertical_box("comb_delay2");
        ui_interface.close_box();
    }

    pub fn get_param(&self, param: ParamIndex) -> Option<FaustFloat> {
        None
    }

    pub fn set_param(&mut self, param: ParamIndex, value: FaustFloat) {
        {}
    }

    pub fn compute<InType, OutType>(
        &mut self,
        count: usize,
        inputs: impl borrow::Borrow<[InType]>,
        mut outputs: impl borrow::BorrowMut<[OutType]>,
    ) where
        InType: borrow::Borrow<[FaustFloat]>,
        OutType: borrow::BorrowMut<[FaustFloat]>,
    {
        let [outputs0] = outputs.borrow_mut() else {
            panic!("wrong number of outputs");
        };
        let outputs0 = outputs0.borrow_mut()[..count].iter_mut();
        let zipped_iterators = outputs0;
        let local_ftbl0mydspSIG0: std::sync::RwLockReadGuard<'_, [f64; 65536]> =
            unsafe { ftbl0mydspSIG0.read().unwrap_unchecked() };
        for output0 in zipped_iterators {
            *unsafe { self.fRec1.get_mut(0).unwrap_unchecked() } = self.fConst1
                + (*unsafe { self.fRec1.get(1).unwrap_unchecked() }
                    - F64::floor(self.fConst1 + *unsafe { self.fRec1.get(1).unwrap_unchecked() }));
            *unsafe {
                self.fVec0
                    .get_mut((self.IOTA0 & 127) as usize)
                    .unwrap_unchecked()
            } = {
                *unsafe {
                    local_ftbl0mydspSIG0
                        .get((65536.0 * *self.fRec1.first().unwrap_unchecked()) as usize)
                        .unwrap_unchecked()
                }
            };
            *unsafe { self.fRec2.get_mut(0).unwrap_unchecked() } = self.fConst2
                + (*unsafe { self.fRec2.get(1).unwrap_unchecked() }
                    - F64::floor(self.fConst2 + *unsafe { self.fRec2.get(1).unwrap_unchecked() }));
            *output0 = *unsafe {
                self.fVec0
                    .get(
                        ((i32::wrapping_sub(
                            self.IOTA0,
                            i32::wrapping_add(
                                (5.0 * (*local_ftbl0mydspSIG0
                                    .get(
                                        (*self.fRec2.first().unwrap_unchecked() * 65536.0) as usize,
                                    )
                                    .unwrap_unchecked()
                                    + 1.0)) as i32,
                                100,
                            ),
                        )) & 127) as usize,
                    )
                    .unwrap_unchecked()
            };
            *unsafe { self.fRec1.get_mut(1).unwrap_unchecked() } =
                *unsafe { self.fRec1.first().unwrap_unchecked() };
            self.IOTA0 = i32::wrapping_add(self.IOTA0, 1);
            *unsafe { self.fRec2.get_mut(1).unwrap_unchecked() } =
                *unsafe { self.fRec2.first().unwrap_unchecked() };
        }
    }
}

impl FaustDsp for mydsp {
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

const SAMPLE_RATE: i32 = 44100;

type Dsp64 = dyn FaustDsp<T = f64>;

fn print_header(mut dsp: Box<Dsp64>, num_total_samples: usize, output_file: &mut File) {
    dsp.init(SAMPLE_RATE);
    writeln!(output_file, "number_of_inputs  : {}", dsp.get_num_inputs()).unwrap();
    writeln!(output_file, "number_of_outputs : {}", dsp.get_num_outputs()).unwrap();
    writeln!(output_file, "number_of_frames  : {}", num_total_samples).unwrap();
}

use criterion::{measurement::WallTime, BenchmarkGroup};
use std::hint::black_box;

pub fn run_benchmark(c: &mut BenchmarkGroup<WallTime>) {
    // Generation constants
    let sample_rate = 48000;
    let buffer_size = 1024;

    let mut dsp = mydsp::new();
    dsp.init(sample_rate);

    let num_inputs = FAUST_INPUTS;
    let num_outputs = FAUST_OUTPUTS;

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
        .collect::<Vec<&[FaustFloat]>>();

    let ib = ibb.as_slice();

    let mut obb = out_buffer
        .iter_mut()
        .map(|buffer| buffer.as_mut_slice())
        .collect::<Vec<&mut [FaustFloat]>>();
    let ob = obb.as_mut_slice();

    c.bench_function("flag_comp_delay2_new", |b| {
        b.iter(|| {
            dsp.compute(black_box(buffer_size), black_box(ib), black_box(&mut *ob));
        })
    });
}
