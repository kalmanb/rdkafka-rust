extern crate gcc;

use std::env;
use std::path::{PathBuf};
use std::process::Command;
use std::ffi::OsString;

fn main() {
    let mut cfg = gcc::Config::new();
    let compiler = cfg.get_compiler();

    run configure
    run make

    // let dst = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // let mut cmd = Command::new("sh");
    // let mut cflags = OsString::new();
    // for arg in compiler.args() {
    //     cflags.push(arg);
    //     cflags.push(" ");
    // }

    // cmd.env("CC", compiler.path())
    //     .env("CFLAGS", cflags)
    //     .env("LD", &which("ld").unwrap())
    //     .env("VERBOSE", "1")
    //     .current_dir(&dst.join("build"))
    //            .arg(msys_compatible(&src.join("curl/configure")));


    // cfg.include("librdkafka/src")
    //     .file("librdkafka/src/rdkafka.h")
    //     .file("librdkafka/src/rdkafka_int.h")
    //     .compile("librdkafka.a");

    // gcc::compile_library("libfoo.a", &["librdkafka/src/rdkafka.c"]);
}
