use crate::util::{const_string, copy_files_and_link, get_platform_args};
use std::process::{Command, Stdio};
use std::{env, fs};

mod util;

// Credit to https://stackoverflow.com/questions/69042049/how-to-compile-rust-to-llvm-bitcode-including-dependencies
fn main() {
    let target = env::args().nth(1).expect("missing target. Arguments: <target>");
    let platform = match env::consts::ARCH {
        "x86_64" => match env::consts::OS {
            "windows" => "x86_64-pc-windows-msvc",
            _ => {
                println!("Unsupported OS");
                return;
            }
        },
        _ => {
            println!("Unsupported arch");
            return;
        }
    };

    println!("Building project...");
    Command::new("cargo").args(["+nightly", "test", "-Z", "build-std", "--target", platform])
        .env("RUSTFLAGS", "--emit=llvm-ir -C lto=thin -C embed-bitcode=yes -C linker-plugin-lto")
        .current_dir(target.clone())
        .spawn().unwrap().wait().unwrap();

    println!("Project built, modifying LLVM IR...");
    let ll_file = copy_files_and_link(target, platform).unwrap();
    let content = fs::read_to_string(&ll_file).unwrap();

    let mut output = String::new();
    let mut consts = 0;
    for line in content.lines() {
        if line.starts_with("@") && consts == 0 {
            consts = output.len();
        }

        if line == "; call tokio::runtime::scheduler::Handle::spawn" {
            output.push_str("call void @test_print(ptr align 1 @test_print_str, i64 4)")
        }
        output.push_str(line);
        output.push_str("\n");
    }
    output.insert_str(consts, &*const_string("test_print_str".to_string(), "Testing!".to_string()));
    output.remove(output.len()-1);
    fs::write(ll_file.clone(), output).unwrap();
    fs::copy(ll_file.clone(), ll_file.parent().unwrap().join("out_testing.ll")).unwrap();

    let mut args = vec!["-v", "-fuse-ld=lld", "-Xlinker", "/subsystem:windows"];
    for platform_arg in get_platform_args(platform) {
        args.push(platform_arg);
    }
    args.push("program.ll");
    args.push("std.rlib");
    //args.push("core.rlib");

    if !Command::new("clang").args(args)
        .current_dir(ll_file.parent().unwrap())
        .stdout(Stdio::inherit()).stderr(Stdio::inherit())
    .spawn().unwrap().wait().unwrap().success() {
        println!("Failed to run Clang");
        return;
    }

    if !Command::new("cmd").args(["/C", "a.exe", "--nocapture"]).current_dir(ll_file.parent().unwrap())
        .stdout(Stdio::inherit()).stderr(Stdio::inherit())
        .spawn().unwrap().wait().unwrap().success() {
        println!("Test failed!");
    }
}
