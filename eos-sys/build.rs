use std::env;
use std::fs::{self};
use std::path::{Path, PathBuf};

//#[cfg(feature = "rebuild-bindings")]
extern crate bindgen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let path = {
        let mut path = PathBuf::new();
        path.push(env::var("CARGO_MANIFEST_DIR").unwrap());
        path.push("lib");
        path.push("SDK");
        path
    };
    let sdk_loc = if let Ok(sdk_loc) = env::var("EOS_SDK_LOCATION") {
        Path::new(&sdk_loc).to_path_buf()
    } else {
        let mut loc = Path::new(&path).to_path_buf();
        loc.push("Bin");
        loc
    };
    let mut audio_loc = Path::new(&sdk_loc).to_path_buf();
    let lib_loc = {
        let mut loc = Path::new(&path).to_path_buf();
        loc.push("Lib");
        loc
    };
    let mut lib = "EOSSDK-Win64-Shipping";
    let audio: &str = "xaudio2_9redist";

    println!("cargo:rerun-if-env-changed=EOS_SDK_LOCATION");
    let target = env::var("TARGET").unwrap();
    if target.contains("windows") {
        if target.contains("i686") {
            lib = "EOSSDK-Win32-Shipping";
            audio_loc.push("x86");
        } else {
            audio_loc.push("x64");
        }
    }
    let mut target_platform = "Windows";

    if target.contains("windows") {
        let dll_file = format!("{}.dll", lib);
        let audio_dll_file = format!("{}.dll", audio);
        let lib_file = format!("{}.lib", lib);
        fs::copy(sdk_loc.join(&dll_file), out_path.join(dll_file))?;
        fs::copy(lib_loc.join(&lib_file), out_path.join(lib_file))?;
        fs::copy(
            audio_loc.join(&audio_dll_file),
            out_path.join(audio_dll_file),
        )?;
    } else if target.contains("darwin") {
        target_platform = "Mac";
        fs::copy(
            sdk_loc.join("libEOSSDK-Mac-Shipping.dylib"),
            out_path.join("libEOSSDK-Mac-Shipping.dylib"),
        )?;
    } else if target.contains("linux") {
        target_platform = "Linux";
        fs::copy(
            sdk_loc.join("libEOSSDK-Linux-Shipping.so"),
            out_path.join("libEOSSDK-Linux-Shipping.so"),
        )?;
    }

    println!("cargo:rustc-link-search={}", out_path.display());
    println!("cargo:rustc-link-lib=dylib={}", lib);

    //#[cfg(feature = "rebuild-bindings")]
    {
        let target_os = if target.contains("windows") {
            "windows"
        } else if target.contains("darwin") {
            "macos"
        } else if target.contains("linux") {
            "linux"
        } else {
            panic!("Unsupported OS");
        };
        let binding_path = Path::new(&format!("src/{}_bindings.rs", target_os)).to_owned();
        let bindings = bindgen::Builder::default()
            .clang_arg("-xc++")
            .clang_arg("-std=c++11")
            .clang_arg(format!("-I{}", path.join("Include").display()))
            .clang_arg(format!("-DEOS_BUILD_PLATFORM_NAME={}", target_platform))
            .header(
                path.join("Include/eos_platform_prereqs.h")
                    .to_string_lossy(),
            )
            .header(path.join("Include/eos_sdk.h").to_string_lossy())
            .header(
                path.join("Include/eos_integratedplatform.h")
                    .to_string_lossy(),
            )
            .default_enum_style(bindgen::EnumVariation::Rust {
                non_exhaustive: true,
            })
            .generate()
            .expect("Unable to generate bindings");

        bindings
            .write_to_file(binding_path)
            .expect("Couldn't write bindings!");
    }

    Ok(())
}
