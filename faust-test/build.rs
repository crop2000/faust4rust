use faust_build::FaustBuilder;
use proc_macro2::{Ident, Span};
use quote::quote;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn build_with_faust(in_file: &str, out_file: &str, xml_dir: &str) {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&dir);
    let faust_path: &Path = &dir.join("../faust/");
    let fb = FaustBuilder::new(in_file, out_file)
        .set_faust_path(faust_path.join("build/bin/faust").to_str().unwrap())
        .set_arch_file("./faust-template.rs")
        .faust_arg("-I")
        .faust_arg(faust_path.join("libraries/").to_str().unwrap());

    fb.build_xml(xml_dir);
    fb.build();
}

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

fn prepare_submodules() {
    Command::new("git")
        .args([
            "submodule",
            "update",
            "--init",
            "--depth 1",
            "--recommend-shallow",
        ])
        .output()
        .unwrap();
}

fn build_faust() {
    let makefile_dir = "../faust";

    // Call the make command to run the Makefile in the specified directory
    let make_command = Command::new("make")
        .current_dir(makefile_dir)
        .status()
        .expect("failed to execute make command");

    if !make_command.success() {
        panic!("make command failed");
    }
}

fn maybe_build_with_faust(file_path: PathBuf, out_path: PathBuf, xml_path: PathBuf) {
    let src_metadata = fs::metadata(&file_path).unwrap();
    let gen_metadata = fs::metadata(&out_path);

    match gen_metadata {
        Ok(gen_metadata) => {
            if src_metadata.modified().unwrap() > gen_metadata.modified().unwrap() {
                build_with_faust(
                    file_path.to_str().unwrap(),
                    out_path.to_str().unwrap(),
                    xml_path.to_str().unwrap(),
                );
            }
        }
        Err(_) => build_with_faust(
            file_path.to_str().unwrap(),
            out_path.to_str().unwrap(),
            xml_path.to_str().unwrap(),
        ),
    };
}

fn generate_dsp() {
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let dir = Path::new(&dir);
    let dsp_path: &Path = &dir.join("../faust/tests/impulse-tests/dsp");
    let out_path: &Path = &dir.join("src/gen");
    let xml_path: &Path = &dir.join("src/xml");

    fs::create_dir_all(out_path).unwrap();
    fs::create_dir_all(xml_path).unwrap();

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
                    ft.is_file() && ext == "dsp" && f != "sound.dsp"
                })
                .unwrap_or(false)
        })
        .map(|entry| entry.path())
        .for_each(|file_path| {
            let out_path = out_path
                .join(file_path.file_name().unwrap())
                .with_extension("rs");

            maybe_build_with_faust(file_path, out_path, xml_path.to_path_buf());
        });

    generate_mod_rs(out_path.to_path_buf()).unwrap();
}

fn main() {
    prepare_submodules();
    build_faust();
    generate_dsp();
}
