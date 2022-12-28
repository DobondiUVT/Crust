use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn compile_code(file_path: &Path) {
    let file_directory = file_path.parent();
    let file_full_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_name = file_full_name.split('.').next().unwrap();
    let compile_command = format!("gcc -o {} {}", file_name, file_full_name);
    let status = Command::new("cmd")
        .env("PATH", "C:\\MinGW\\bin")
        .current_dir(file_directory.unwrap().to_str().unwrap())
        .arg("/C")
        .arg(&compile_command)
        .status()
        .expect("C compilation failed");
    if !status.success() {
        std::process::exit(1);
    }
    println!("Compiled successfully");
}
fn execute_file(file_path: &Path, file_output: &mut String) {
    let file_directory = file_path.parent();
    let file_full_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_name = file_full_name.split('.').next().unwrap();
    let execute_command = format!("{}.exe", file_name);
    println!("Executing...");
    let output = Command::new("cmd")
        .current_dir(file_directory.unwrap().to_str().unwrap())
        .arg("/C")
        .arg(&execute_command)
        .output()
        .expect("Execution failed");
    if !output.status.success() {
        std::process::exit(1);
    }
    file_output.push_str(&String::from_utf8_lossy(&output.stdout));
}
fn get_test_output(file_path: &Path, test_output: &mut String) {
    let mut path_buf = PathBuf::new();
    path_buf.push(file_path);
    path_buf.set_extension("txt");
    let contents = fs::read_to_string(path_buf).expect("Cannot read test file");
    test_output.push_str(&contents);
}
fn compare_test(file_output: &String, test_output: &String) -> bool {
    if file_output == test_output {
        print!("Test passed ✅");
        return true;
    }
    print!("Test failed ❌");
    false
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path_string = &args[1];
    let file_path = Path::new(file_path_string);
    let mut file_output = "".to_string();
    let mut test_output = "".to_string();
    if !file_path.exists() {
        eprintln!("File does not exist");
        std::process::exit(1);
    }

    if cfg!(target_os = "windows") {
        compile_code(file_path);
        execute_file(file_path, &mut file_output);
        get_test_output(file_path, &mut test_output);
        compare_test(&file_output, &test_output);
    }
}
