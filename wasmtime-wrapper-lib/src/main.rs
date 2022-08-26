use wasmtimewrapper;
use std::{env, fs, ffi::CStr};

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {} <filename.wat> <JSON>", &args[0]);
        return;
    }

    // Transform args[1] into a CString
    let filename = String::from(&args[1]);
    let c_filename = std::ffi::CString::new(filename).unwrap();

    // Read the content of the file at args[2] and put it into a CString
    let json = fs::read_to_string(&args[2]).unwrap();
    let c_json = std::ffi::CString::new(json).unwrap();


    let out_ptr = wasmtimewrapper::compile_and_exec(c_filename.as_ptr(), c_json.as_ptr());
    let x = unsafe { CStr::from_ptr(out_ptr).to_string_lossy().into_owned() };
    println!(">MAIN {}", x);
}

#[cfg(test)]
mod tests {

    use std::{fs, ffi::CStr};

    #[test]
    fn non_regression() {
            let filename = String::from("../add-json-rs/target/wasm32-wasi/debug/add-json-rs.wasm");
            let c_filename = std::ffi::CString::new(filename).unwrap();

            let json = fs::read_to_string("../wasmtime-ffi/json_input/work.json").unwrap();
            let c_json = std::ffi::CString::new(json).unwrap();

            let out_ptr = wasmtimewrapper::compile_and_exec(c_filename.as_ptr(), c_json.as_ptr());
            let x = unsafe { CStr::from_ptr(out_ptr).to_string_lossy().into_owned() };

            assert_eq!(x, String::from(r#"{"number1":59,"number2":61,"res":120}"#));
    }
}