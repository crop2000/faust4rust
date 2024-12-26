/* ------------------------------------------------------------
name: "test"
Code generated with Faust 2.76.0 (https://faust.grame.fr)
Compilation options: -a ../../architecture/rust/jack-double.rs -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0
------------------------------------------------------------ */
/************************************************************************
FAUST Architecture File
Copyright (C) 2003-2024 GRAME, Centre National de Creation Musicale
---------------------------------------------------------------------
This Architecture section is free software; you can redistribute it
and/or modify it under the terms of the GNU General Public License
as published by the Free Software Foundation; either version 3 of
the License, or (at your option) any later version.

This program is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with this program; If not, see <http://www.gnu.org/licenses/>.

EXCEPTION : As a special exception, you may create a larger work
that contains this FAUST architecture section and distribute
that work under terms of your choice, so long as this FAUST
architecture section is not modified.

************************************************************************
************************************************************************/

#![allow(unused_parens)]
#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_mut)]
#![allow(non_upper_case_globals)]

//! Faust JACK architecture file
extern crate jack;
use jack::prelude as j;
use std::borrow::Borrow;
use std::borrow::BorrowMut;
use std::io;
extern crate libm;

type F32 = f32;
type F64 = f64;

#[derive(Copy, Clone)]
pub struct ParamIndex(pub i32);

pub struct Soundfile<'a, T> {
    fBuffers: &'a &'a T,
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

pub type FaustFloat = F64;
use std::borrow;
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

pub const FAUST_INPUTS: usize = 1;
pub const FAUST_OUTPUTS: usize = 1;
pub const FAUST_ACTIVES: usize = 0;
pub const FAUST_PASSIVES: usize = 0;

#[cfg_attr(feature = "default-boxed", derive(default_boxed::DefaultBoxed))]
#[repr(C)]
pub struct mydsp {
    fSampleRate: i32,
}

impl mydsp {
    pub fn new() -> mydsp {
        mydsp { fSampleRate: 0 }
    }
    pub fn metadata(&self, m: &mut dyn Meta) {
        m.declare("compile_options", r"-a ../../architecture/rust/jack-double.rs -lang rust -ct 1 -es 1 -mcd 16 -mdd 1024 -mdy 33 -double -ftz 0");
        m.declare("filename", r"test.dsp");
        m.declare("name", r"test");
    }

    pub fn get_sample_rate(&self) -> i32 {
        self.fSampleRate
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
        mydsp::class_init(sample_rate);
        self.instance_init(sample_rate);
    }

    pub fn build_user_interface(&self, ui_interface: &mut dyn UI<FaustFloat>) {
        Self::build_user_interface_static(ui_interface);
    }

    pub fn build_user_interface_static(ui_interface: &mut dyn UI<FaustFloat>) {
        ui_interface.open_vertical_box("test");
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

    pub fn compute<InType, OutType>(
        &mut self,
        count: usize,
        inputs: impl borrow::Borrow<[InType]>,
        mut outputs: impl borrow::BorrowMut<[OutType]>,
    ) where
        InType: borrow::Borrow<[FaustFloat]>,
        OutType: borrow::BorrowMut<[FaustFloat]>,
    {
        let [inputs0] = inputs.borrow() else {
            panic!("wrong number of inputs");
        };
        let inputs0 = inputs0.borrow()[..count].iter();
        let [outputs0] = outputs.borrow_mut() else {
            panic!("wrong number of outputs");
        };
        let outputs0 = outputs0.borrow_mut()[..count].iter_mut();
        let zipped_iterators = inputs0.zip(outputs0);
        for (input0, output0) in zipped_iterators {
            *output0 = 0.9 * *input0;
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

// fn main() {
//     // Create JACK client
//     let (client, _status) =
//         j::Client::new("faust_rust", j::client_options::NO_START_SERVER).unwrap();

//     // Allocation DSP on the heap
//     let mut dsp;
//     #[cfg(feature = "default-boxed")]
//     {
//         use default_boxed::DefaultBoxed;
//         dsp = mydsp::default_boxed();
//     }

//     #[cfg(not(feature = "default-boxed"))]
//     {
//         dsp = Box::new(mydsp::new());
//     }

//     println!(
//         "Faust Rust code running with JACK: sample-rate = {} buffer-size = {}",
//         client.sample_rate(),
//         client.buffer_size()
//     );

//     println!("get_num_inputs: {}", dsp.get_num_inputs());
//     println!("get_num_outputs: {}", dsp.get_num_outputs());

//     // Init DSP with a given SR
//     dsp.init(client.sample_rate() as i32);

//     // Register ports. They will be used in a callback that will be
//     // called when new data is available.

//     let in_a = client
//         .register_port("in1", j::AudioInSpec::default())
//         .unwrap();
//     let in_b = client
//         .register_port("in2", j::AudioInSpec::default())
//         .unwrap();

//     let mut out_a = client
//         .register_port("out1", j::AudioOutSpec::default())
//         .unwrap();
//     let mut out_b = client
//         .register_port("out2", j::AudioOutSpec::default())
//         .unwrap();

//     let process_callback = move |_: &j::Client, ps: &j::ProcessScope| -> j::JackControl {
//         let mut out_a_p = j::AudioOutPort::new(&mut out_a, ps);
//         let mut out_b_p = j::AudioOutPort::new(&mut out_b, ps);

//         let in_a_p = j::AudioInPort::new(&in_a, ps);
//         let in_b_p = j::AudioInPort::new(&in_b, ps);

//         // Adapt f32 inputs in f64 inputs
//         let input0: &[f32] = &in_a_p;
//         let input1: &[f32] = &in_b_p;

//         let input0_f64: Vec<f64> = input0.iter().map(|&sample| sample as f64).collect();
//         let input1_f64: Vec<f64> = input1.iter().map(|&sample| sample as f64).collect();

//         let inputs_f64: [&[f64]; 2] = [&input0_f64[..], &input1_f64[..]];
//         let inputs_ref: &[&[f64]] = &inputs_f64;

//         // Prepare f64 outputs
//         let mut output0_f64: Vec<f64> = vec![0.0; out_a_p.len()];
//         let mut output1_f64: Vec<f64> = vec![0.0; out_b_p.len()];

//         let mut outputs_f64: [&mut [f64]; 2] = [&mut output0_f64[..], &mut output1_f64[..]];
//         // let outputs_ref: &mut [&mut [f64]] = &mut outputs_f64;

//         // Compute using f64 inputs and outputs
//         dsp.compute(
//             in_a_p.len(),
//             inputs_f64.as_slice(),
//             outputs_f64.as_mut_slice(),
//         );

//         // Convert f64 outputs to f32 outputs
//         let output0: &mut [f32] = &mut out_a_p;
//         let output1: &mut [f32] = &mut out_b_p;

//         // Copy and convert outputs_ref[0] (f64) to output0 (f32)
//         for (dest, &src) in output0.iter_mut().zip(outputs_f64[0].iter()) {
//             *dest = src as f32;
//         }
//         for (dest, &src) in output1.iter_mut().zip(outputs_f64[1].iter()) {
//             *dest = src as f32;
//         }

//         j::JackControl::Continue
//     };
//     let process = j::ClosureProcessHandler::new(process_callback);

//     // Activate the client, which starts the processing.
//     let active_client = j::AsyncClient::new(client, (), process).unwrap();

//     // Wait for user input to quit
//     println!("Press enter/return to quit...");
//     let mut user_input = String::new();
//     io::stdin().read_line(&mut user_input).ok();

//     active_client.deactivate().unwrap();
// }

fn compute_literal(inputs: &[&[f32]], outputs: &mut [&mut [f32]]) {
    dbg!(inputs);
    dbg!(outputs);
}

pub fn compute_ref<InType, OutType>(
    inputs: impl AsRef<[InType]>,
    mut outputs: impl AsMut<[OutType]>,
) where
    InType: AsRef<[f32]>,
    OutType: AsMut<[f32]>,
{
    let i = inputs.as_ref()[0].as_ref()[0];
    let o = outputs.as_mut()[0].as_mut()[0];
    println!("{i}{o}");
}

pub fn compute_borrow<InType, OutType>(
    inputs: impl Borrow<[InType]>,
    mut outputs: impl BorrowMut<[OutType]>,
) where
    InType: borrow::Borrow<[f32]>,
    OutType: borrow::BorrowMut<[f32]>,
{
    let i = inputs.borrow()[0].borrow()[0];
    let o = outputs.borrow_mut()[0].borrow()[0];
    println!("{i}{o}");
}

pub fn compute_andref<InType, OutType>(
    inputs: &(impl AsRef<[InType]> + ?Sized),
    mut outputs: &mut (impl AsMut<[OutType]> + ?Sized),
) where
    InType: AsRef<[f32]>,
    OutType: AsMut<[f32]>,
{
    let i = inputs.as_ref()[0].as_ref()[0];
    let o = outputs.as_mut()[0].as_mut()[0];
    println!("{i}{o}");
}

pub fn compute_andborrow<InType, OutType>(
    inputs: &(impl Borrow<[InType]> + ?Sized),
    mut outputs: &mut (impl BorrowMut<[OutType]> + ?Sized),
) where
    InType: borrow::Borrow<[f32]>,
    OutType: borrow::BorrowMut<[f32]>,
{
    let i = inputs.borrow()[0].borrow()[0];
    let o = outputs.borrow_mut()[0].borrow()[0];
    println!("{i}{o}");
}

fn main() {
    //vectors of vectors
    let v1 = vec![0f32; 16];
    let v2 = vec![0f32; 16];
    let mut v3 = vec![0f32; 16];
    let mut v4 = vec![0f32; 16];

    let ivv = vec![v1, v2];
    let mut ovv = vec![v3, v4];

    // compute_literal(ivv, ovv);
    // compute_ref(ivv, ovv);//consumes
    compute_ref(&ivv, &mut ovv);
    // compute_borrow(ivv, ovv);//consumes

    compute_andref(&ivv, &mut ovv);
    compute_andborrow(&ivv, &mut ovv);
    compute_andref(&ivv, &mut ovv);
    compute_andborrow(&ivv, &mut ovv);

    //vectors of slices
    let v1 = vec![0f32; 16];
    let v2 = vec![0f32; 16];
    let mut v3 = vec![0f32; 16];
    let mut v4 = vec![0f32; 16];

    let ivs = vec![v1.as_slice(), v2.as_slice()];
    let mut ovs = vec![v3.as_mut_slice(), v4.as_mut_slice()];

    compute_literal(&ivs, &mut ovs);
    compute_ref(&ivs, &mut ovs);
    // compute_borrow(ivs, ovs); //consumes
    // compute_borrow(&ivs, &mut ovs);
    compute_ref(&*ivs, &mut ovs);
    compute_borrow(&*ivs, &mut *ovs);

    compute_andref(&ivs, &mut ovs);
    compute_andborrow(&ivs, &mut ovs);

    //slices of slices
    let iss = ivs.as_slice();
    let oss = ovs.as_mut_slice();

    compute_literal(iss, oss);
    // compute_ref(iss, oss); //consumes oss
    // compute_borrow(iss, oss);//consumes oss
    compute_ref(iss, &mut *oss);
    compute_borrow(iss, &mut *oss);

    compute_andref(iss, oss);
    compute_andborrow(iss, oss);

    //arrays of slices
    let ias = [v1.as_slice(), v2.as_slice()];
    let mut oas = [v3.as_mut_slice(), v4.as_mut_slice()];

    compute_literal(&ias, &mut oas);
    compute_ref(&ias, &mut oas);
    compute_ref(ias, &mut oas);
    // compute_borrow(ias, &mut oas);
    // compute_borrow(ias, oas); //consums oas
    compute_andref(&ias, &mut oas);
    compute_andborrow(&ias, &mut oas);

    //slices of slices from array
    let iss = ias.as_slice();
    let oss = oas.as_mut_slice();

    compute_literal(iss, oss);
    // compute_ref(iss, oss); //consumes oss
    compute_borrow(iss, &mut *oss);
    // compute_borrow(iss, oss); // compute oss
    compute_ref(iss, &mut *oss);
    compute_andref(iss, oss);
    compute_andborrow(iss, oss);

    // BOTH MUT

    //vectors of vectors
    let mut v1 = vec![0f32; 16];
    let mut v2 = vec![0f32; 16];
    let mut v3 = vec![0f32; 16];
    let mut v4 = vec![0f32; 16];

    let mut ivv = vec![v1, v2];
    let mut ovv = vec![v3, v4];

    // compute_literal(ivv, ovv);
    // compute_ref(ivv, ovv);//consumes both
    compute_ref(&mut ivv, &mut ovv);
    compute_ref(&mut ivv, &mut ovv);
    // compute_borrow(&mut ivv, &mut ovv);
    // compute_borrow(ivv, ovv); //consumes both
    compute_andref(&ivv, &mut ovv);
    compute_andborrow(&ivv, &mut ovv);

    //vectors of slices
    let mut v1 = vec![0f32; 16];
    let mut v2 = vec![0f32; 16];
    let mut v3 = vec![0f32; 16];
    let mut v4 = vec![0f32; 16];

    let mut ivs = vec![v1.as_mut_slice(), v2.as_mut_slice()];
    let mut ovs = vec![v3.as_mut_slice(), v4.as_mut_slice()];

    // compute_literal(&ivs, &mut ovs);
    // compute_literal(ivs, ovs); //consumes mut
    compute_ref(&mut ivs, &mut ovs);
    compute_ref(&mut *ivs, &mut *ovs); //reborrow not needed
                                       // compute_borrow(ivs, ovs); // consumes both
    compute_borrow(&*ivs, &mut *ovs);
    compute_andref(&mut ivs, &mut ovs);
    compute_andborrow(&mut ivs, &mut ovs);

    //slices of slices
    let iss = ivs.as_mut_slice();
    let oss = ovs.as_mut_slice();

    // compute_literal(iss, oss);
    // compute_ref(iss, oss); //consumes both
    compute_ref(&*iss, &mut *oss);
    // compute_borrow(iss, oss); //consumes both
    compute_borrow(&*iss, &mut *oss);
    compute_andref(iss, oss);
    compute_andborrow(iss, oss);

    //arrays of slices
    let mut ias = [v1.as_mut_slice(), v2.as_mut_slice()];
    let mut oas = [v3.as_mut_slice(), v4.as_mut_slice()];

    // compute_literal(&ias, &mut oas);
    compute_ref(&ias, &mut oas);
    // compute_ref(ias, &mut oas); //consumes both
    // compute_borrow(ias, &mut oas);
    // compute_borrow(ias, oas); //consumes oas
    compute_andref(&ias, &mut oas);
    compute_andborrow(&ias, &mut oas);

    //slices of slices from array
    let iss = ias.as_mut_slice();
    let oss = oas.as_mut_slice();

    // compute_literal(iss, oss);
    // compute_ref(iss, oss);//consumes both
    // compute_borrow(iss, oss); //consumes both
    compute_andref(iss, oss);
    compute_andborrow(iss, oss);
}
