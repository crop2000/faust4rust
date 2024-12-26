use faust_build::FaustBuilder;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::path::{Path, PathBuf};
use std::{env, fs};

// fn build_with_faust(in_file: &str, out_path_scalar: &str, out_path_vector: &str, xml_dir: &str) {
//     let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
//     let dir = Path::new(&dir);
//     let faust_path: &Path = &dir.join("faust/");
//     let fb = FaustBuilder::new(in_file, out_path_scalar)
//         .set_faust_path(faust_path.join("build/bin/faust").to_str().unwrap())
//         .set_arch_file("./bench_template.rs")
//         .faust_arg("-I")
//         .faust_arg(faust_path.join("libraries/").to_str().unwrap());

//     fb.build_xml(xml_dir);

//     fb.build();

//     let fbv = FaustBuilder::new(in_file, out_path_vector)
//         .set_faust_path(faust_path.join("build/bin/faust").to_str().unwrap())
//         .set_arch_file("./faust-template.rs")
//         .faust_arg("-vec")
//         .faust_arg("-vs")
//         .faust_arg("4")
//         .faust_arg("-I")
//         .faust_arg(faust_path.join("libraries/").to_str().unwrap());
//     fbv.build();
// }

fn generate_mod_rs(path: PathBuf) -> std::io::Result<()> {
    let entries = fs::read_dir(path.clone())?;
    let re_exports = entries
        .filter_map(|entry| entry.ok())
        .filter_map(|entry| {
            entry
                .file_type()
                .unwrap()
                .is_file()
                .then_some(entry.file_name())
        })
        .filter_map(|entry| entry.to_str().unwrap().ends_with(".rs").then_some(entry))
        .filter_map(|entry| (entry != "mod.rs").then_some(entry))
        .map(|entry| {
            let entry = entry.to_str().unwrap();
            let module_name = entry.trim_end_matches(".rs");
            let module_ident = Ident::new(module_name, Span::call_site());
            Some(quote! { pub mod #module_ident; })
        })
        .collect::<Vec<_>>();

    // Write the re-export statements to mod.rs
    let mod_rs_content = quote! {
        #(#re_exports)*
    };
    let t =
        syn::parse_file(&("#![allow(non_snake_case)]\n".to_owned() + &mod_rs_content.to_string()))
            .unwrap();
    let ps = prettyplease::unparse(&t);
    fs::write(path.join("mod.rs"), ps)
}

fn maybe_build_with_faust(fb: &mut FaustBuilder, in_path: PathBuf, out_path: PathBuf) {
    let src_metadata = fs::metadata(&in_path).unwrap();
    let out_metadata = fs::metadata(&out_path);

    match out_metadata {
        Ok(gen_metadata) => {
            if src_metadata.modified().unwrap() > gen_metadata.modified().unwrap() {
                {
                    fb.in_file = in_path.to_str().unwrap().to_owned();
                    fb.out_file = out_path.to_str().unwrap().to_owned();
                    fb.build();
                };
            }
        }
        Err(_) => {
            fb.in_file = in_path.to_str().unwrap().to_owned();
            fb.out_file = out_path.to_str().unwrap().to_owned();
            fb.build();
        }
    };
}

fn generate_dsp() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&dir);
    let dsp_path: &Path = &dir.join("./faust/tests/impulse-tests/dsp");

    let dummy_file = "";
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&dir);
    let faust_path: &Path = &dir.join("faust/");

    let flags = vec![vec![], vec!["-rt"]];

    for flags in flags {
        let flag_folder: String = flags
            .iter()
            .fold("flag".to_owned(), |r, &s| r + s)
            .replace("-", "")
            .replace(" ", "");
        let out_path: &Path = &dir.join("benches").join(&flag_folder);

        fs::create_dir_all(out_path).unwrap();

        let mut fb: FaustBuilder = FaustBuilder::new(dummy_file, dummy_file)
            .set_faust_path(faust_path.join("build/bin/faust").to_str().unwrap())
            .set_arch_file("./bench_template.rs")
            .faust_arg("-I")
            .set_module_name(&flag_folder)
            .faust_arg(faust_path.join("libraries").to_str().unwrap());

        for flag in flags {
            fb = fb.faust_arg(flag);
        }

        let entries = fs::read_dir(dsp_path).expect("cannot read dir");
        entries
            .filter_map(|entry| entry.ok())
            .filter(|entry| {
                entry
                    .file_type()
                    .map(|ft| {
                        let p = entry.path();
                        let f: &str = if let Some(e) = p.file_name() {
                            e.to_str()
                        } else {
                            Some("")
                        }
                        .unwrap_or("");
                        let ext = match p.extension() {
                            Some(e) => e.to_str(),
                            None => Some(""),
                        }
                        .unwrap_or("");
                        ft.is_file()
                            && ext == "dsp"
                            && f != "sound.dsp"
                            && f != "osc_enable.dsp"
                            && f != "bs.dsp"
                            && f != "prefix.dsp"
                    })
                    .unwrap_or(false)
            })
            .map(|entry| entry.path())
            .for_each(|in_path| {
                let file_name = in_path.file_name().unwrap();
                let out_path = out_path.join(file_name).with_extension("rs");
                maybe_build_with_faust(&mut fb, in_path, out_path);
            });

        generate_mod_rs(out_path.to_path_buf()).unwrap();
    }
}

// $(MAKE) -f Make.rust FAUSTOPTIONS="-I dsp -double -cm -a archs/rust/architecture_cm.rs"
// $(MAKE) -f Make.rust FAUSTOPTIONS="-I dsp -double -ec -a archs/rust/architecture.rs"
// $(MAKE) -f Make.rust FAUSTOPTIONS="-I dsp -double -a archs/rust/architecture_normal.rs"
// $(MAKE) -f Make.rust FAUSTOPTIONS="-I dsp -double -fp -a archs/rust/architecture_normal.rs"
// $(MAKE) -f Make.rust FAUSTOPTIONS="-I dsp -double -vec -vs 4 -a archs/rust/architecture_normal.rs"
// $(MAKE) -f Make.rust FAUSTOPTIONS="-I dsp -double -vec -vs 32 -a archs/rust/architecture_normal.rs"

// $(FAUST) -lang rust $(FAUSTOPTIONS) -i -A ../../architecture dsp/$*.dsp -o archs/rust/src/bin/$*.rs

fn main() {
    // let _args = [
    //     "-I",
    //     "dsp",
    //     "-double",
    //     "-cm",
    //     "-a",
    //     "archs/rust/architecture_cm.rs",
    //     "-i",
    //     "-A",
    //     "../../architecture",
    //     "dsp/$*.dsp",
    //     "-o",
    //     "archs/rust/src/bin/$*.rs",
    // ];
    // prepare_submodules();
    // build_faust();
    //  generate_dsp();
}
