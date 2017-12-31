extern crate bindgen;

use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn format_write(builder: bindgen::Builder, output: &str) {
    let s = builder
        .generate()
        .unwrap()
        .to_string()
        .replace("/**", "/*")
        .replace("/*!", "/*");

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(output)
        .unwrap();

    let _ = file.write(s.as_bytes());
}

fn common_builder() -> bindgen::Builder {
    bindgen::builder()
        .raw_line("#![allow(dead_code)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
}

use std::env;

fn main() {
    let ndk = env::var("ANDROID_NDK").expect("Please set the ANDROID_NDK env variable");
    let platform = env::var("NDK_PLATFORM").expect("Please set the NDK_PLATFORM env variable");
    let headers = &[Path::new(&ndk)
        .join("platforms")
        .join(platform)
        .join("usr").join("include")];

     // -I/usr/local/Cellar/android-ndk/r10d/platforms/android-9/arch-mips/usr/includ


    let mut builder = common_builder().header("data/opensles.h");

    for header in headers {
        builder = builder.clang_arg("-I").clang_arg(header.to_str().unwrap());
        println!("{:?}", header);
    }

    // Manually fix the comment so rustdoc won't try to pick them
    format_write(builder, "src/opensles.rs");
}
