use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::process::{Command, Stdio};

pub fn copy_files_and_link(target: String, platform: &str) -> Option<PathBuf> {
    let mut target = target;
    if fs::metadata(format!("{}/target", target)).is_err() {
        println!("Found workspace, fishing for parent");
        target = PathBuf::from(target)
            .parent()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string();
    }

    let output_dir = format!("{}/target/logical_race_detector", target);
    let _ = fs::remove_dir_all(output_dir.clone());
    fs::create_dir_all(output_dir.clone()).unwrap();

    for path in fs::read_dir(format!("{}/target/{}/debug/deps/", target, platform)).unwrap() {
        let path = path.unwrap();
        let str_path = path.path();
        let str_path = str_path.to_str().unwrap();
        let file_name = path.file_name();
        let file_name = file_name.to_str().unwrap();
        if str_path.ends_with(".ll")
            && !file_name.starts_with("std-")
            //&& !file_name.starts_with("core")
            && !file_name.starts_with("panic_abort")
            && !file_name.starts_with("proc_macro")
        {
            fs::copy(path.path(), format!("{}/{}", output_dir, file_name)).unwrap();
        } else if str_path.ends_with(".rlib") {
            if file_name.starts_with("libstd-") {
                fs::copy(path.path(), format!("{}/{}", output_dir, "std.rlib")).unwrap();
            } else if file_name.starts_with("libcore") {
                //fs::copy(path.path(), format!("{}/{}", output_dir, "core.rlib")).unwrap();
            }
        }
    }

    let output_file_temp = format!("{}/program.temp", output_dir);
    let output_file = format!("{}/program.ll", output_dir);
    if !Command::new("llvm-link")
        .args(["--only-needed", "-S", "*.ll"])
        .current_dir(output_dir.clone())
        .stdout(Stdio::from(File::create(output_file_temp.clone()).unwrap()))
        .stderr(Stdio::inherit())
        .spawn()
        .unwrap()
        .wait()
        .unwrap()
        .success()
    {
        panic!("Failed to link files");
    }

    fs::copy(output_file_temp, output_file.clone()).unwrap();

    return Some(output_file.into());
}

pub fn const_string(var_name: String, string: String) -> String {
    return format!("@{} = private unnamed_addr constant <{{ [{} x i8] }}> <{{ [{} x i8] c\"{}\" }}>, align 1\n", var_name, string.len(), string.len(), string);
}

pub fn get_platform_args(platform: &str) -> Vec<&'static str> {
    match platform {
        "x86_64-pc-windows-msvc" => vec![
            "-LC:\\Windows\\System32",
            "-lkernel32",
            "-lws2_32",
            "-lntdll",
            "-luserenv",
        ],
        _ => unreachable!(),
    }
}
