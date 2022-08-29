use std::{error::Error, io::stdin};
use wire::{JsonData};

mod wire;

fn add(a: u64, b: u64) -> u64 {
    return a + b;
}

// #[no_mangle]
// pub fn alloc(len: usize) -> *mut u8 {
//     // create a new mutable buffer with capacity `len`
//     let mut buf = Vec::with_capacity(len);
//     // take a mutable pointer to the buffer
//     let ptr = buf.as_mut_ptr();
//     // take ownership of the memory block and
//     // ensure that its destructor is not
//     // called when the object goes out of scope
//     // at the end of the function
//     std::mem::forget(buf);
//     // return the pointer so the runtime
//     // can write data at this offset
//     return ptr;
// }

// #[no_mangle]
// pub unsafe fn add_json(ptr: *mut u8, len: usize) -> *mut u8 {
//     // Create a `Vec<u8>` from the pointer and length
//     let data = Vec::from_raw_parts(ptr, len, len);
        
//     println!("<GUEST input_buf = {:?}", data);

//     // Convert raw input to JsonData
//     let mut input: JsonData = serde_json::from_slice(&data).map_err(|e| {
//         eprintln!("ser: {e}");
//         e
//     }).unwrap();

//     println!("<GUEST input = {:?}", input);

//     // Prepare output
//     input.res = add(input.number1, input.number2);
//     let mut serialized = serde_json::to_vec(&input).map_err(|e| {
//         eprintln!("de: {e}");
//         e
//     }).unwrap();
    
//     // Return pointer
//     let ptr = serialized.as_mut_ptr();
//     std::mem::forget(ptr);
//     return ptr;
// }

fn main() -> Result<(), Box<dyn Error>> {
    let mut input: JsonData = serde_json::from_reader(stdin()).map_err(|e| {
        eprintln!("ser: {e}");
        e
    })?;

    input.res = add(input.number1, input.number2);
    let serialized = serde_json::to_string(&input).map_err(|e| {
        eprintln!("de: {e}");
        e
    }).unwrap();

    println!("{serialized}");

    Ok(())
}

