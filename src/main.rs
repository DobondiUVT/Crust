use std::path::{Path, PathBuf};
use std::process::Command;
use std::{env, fs};

fn compile_code(file_path: &Path) {
    // Compile C code using GCC
    let file_directory = file_path.parent();
    let file_full_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_name = file_full_name.split('.').next().unwrap();
    let compile_command = format!("gcc -o {} {}", file_name, file_full_name);
    let status;

    // Check for OS and compile using cmd or shell
    if cfg!(windows) {
        status = Command::new("cmd")
            .env("PATH", "C:\\MinGW\\bin")
            .current_dir(file_directory.unwrap().to_str().unwrap())
            .arg("/C")
            .arg(&compile_command)
            .status()
            .expect("C compilation failed");
    } else {
        status = Command::new("sh")
            .arg("-c")
            .arg(&compile_command)
            .status()
            .expect("C compilation failed");
    }
    if !status.success() {
        std::process::exit(1);
    }
    println!("Compiled successfully");
}

fn execute_file(file_path: &Path, file_output: &mut String) {
    // Execute C file
    let file_directory = file_path.parent();
    let file_full_name = file_path.file_name().unwrap().to_str().unwrap();
    let file_name = file_full_name.split('.').next().unwrap();
    let output;
    println!("Executing...");

    // Check for OS and execute using cmd or shell
    if cfg!(windows) {
        let execute_command = format!("{}.exe", file_name);
        output = Command::new("cmd")
            .current_dir(file_directory.unwrap().to_str().unwrap())
            .arg("/C")
            .arg(&execute_command)
            .output()
            .expect("Execution failed");
    } else {
        let execute_command = format!("./{}", file_name);
        output = Command::new("sh")
            .arg("-c")
            .arg(&execute_command)
            .output()
            .expect("Execution failed");
    }
    if !output.status.success() {
        std::process::exit(1);
    }
    file_output.push_str(&String::from_utf8_lossy(&output.stdout));
}

fn get_test_output(test_path: &Path, test_output: &mut String) {
    // Get text from the test file
    let mut path_buf = PathBuf::new();
    path_buf.push(test_path);
    path_buf.set_extension("txt");
    let contents = fs::read_to_string(path_buf).expect("Cannot read test file");
    test_output.push_str(&contents);
}

fn compare_test(file_output: &String, test_output: &String) -> bool {
    // Remove whitespace to compare only data.
    let file_bytes: String = file_output.chars().filter(|c| !c.is_whitespace()).collect();
    let test_bytes: String = test_output.chars().filter(|c| !c.is_whitespace()).collect();
    if file_bytes == test_bytes {
        print!("Test passed ✅");
        return true;
    }
    print!("Test failed ❌");
    false
}

fn main() {
    // Collect two arguments, C file, test file
    let args: Vec<String> = env::args().collect();
    let file_path_string = &args[1];
    let file_path = Path::new(file_path_string);
    let test_path_string = &args[2];
    let test_path = Path::new(test_path_string);
    let mut file_output = "".to_string();
    let mut test_output = "".to_string();

    // Arg error handle
    if !file_path.exists() {
        eprintln!("File does not exist");
        std::process::exit(1);
    }
    if !test_path.exists() {
        eprintln!("Test file does not exist");
        std::process::exit(1);
    }

    // Execute and test
    compile_code(file_path);
    execute_file(file_path, &mut file_output);
    get_test_output(test_path, &mut test_output);
    compare_test(&file_output, &test_output);
}
