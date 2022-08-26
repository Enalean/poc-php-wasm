use wasi_common::WasiCtx;
use wasi_common::pipe::{ReadPipe, WritePipe};
use wasmtime::*;
use anyhow::{Result, Context};
use std::ffi::{CStr, CString};
use std::fs::File;
use std::io::Read;
use std::os::raw::{c_char};
use std::sync::Arc;
use wasmtime_wasi::sync::WasiCtxBuilder;

// Define the maximum capacity of the memory
const MAX_MEMORY_SIZE: usize = 2 << 20; /* 2 Mo */

// Define the maximum execution time of the WASM module in ms
const MAX_EXEC_TIME: u64 = 80; /* 80 ms */ 

// Define the names of the functions that we will import
// from the WASM module
const MEMORY: &str = "memory";
const ALLOC_FN: &str = "alloc";
const MAIN_FN: &str = "add_json";

// Define the cache path (MODIFY FOR YOUR SYSTEM)
const CACHE_PATH: &str = "/home/bot/Documents/experimentations/PHP-FFI-Rustlib/wasmtime-wrapper-lib/CacheConfig.toml";

#[no_mangle]
/* Function to execute a WebAssembly module with the given JSON input */
pub extern "C" fn compile_and_exec(filename: *const c_char, json: *const c_char) -> *const c_char {
    // You can change the function that is called here !
    match wrapped_compile_and_exec_wasiio(filename, json) {
        Ok(s)  => {
            let c_str = CString::new(s).unwrap();
            let c_ptr = c_str.as_ptr();
            std::mem::forget(c_str);
            return c_ptr;
        },
        Err(e) => eprintln!("Failed to run program: {}", e)
    };
    let c_str = CString::new("error").unwrap();
    let c_ptr = c_str.as_ptr();
    std::mem::forget(c_str);
    return c_ptr;
}

struct MyApplicationState {
    limits: StoreLimits,
    wasi: WasiCtx
}

fn load_module(engine: &Engine, path: &String) -> Result<Module> {
    // Peek at the first few bytes of the file to figure out if this is
    // something we can pass off to `deserialize_file`.
    let mut file =
        File::open(path).with_context(|| format!("failed to open: {}", path))?;
    let mut magic = [0; 4];
    if let Ok(()) = file.read_exact(&mut magic) {
        if &magic == b"\x7fELF" {
            return unsafe { Module::deserialize_file(engine, path) };
        }
    }

    Module::from_file(engine, path)
}

fn wrapped_compile_and_exec_wasiio(filename_ptr: *const c_char, json_ptr: *const c_char) -> Result<String, anyhow::Error> {
    // Check that the argument pointer are valid
    if filename_ptr.is_null() {
        return Err(anyhow::anyhow!("filename_ptr is null in wrapped_compile_and_exec"));
    } else if json_ptr.is_null() {
        return Err(anyhow::anyhow!("json_ptr is null in wrapped_compile_and_exec"));
    }
    // Convert filename_ptr to String
    let filename = unsafe { CStr::from_ptr(filename_ptr).to_string_lossy().into_owned() };
    // Convert json_ptr to proper JSON input
    let mut input = unsafe { CStr::from_ptr(json_ptr).to_string_lossy().into_owned() };
    input.pop();

    // Prepare stdio
    let stdin = ReadPipe::from(input.to_owned());
    let stdout = WritePipe::new_in_memory();

    // Create my_state with StoreLimiter and WASI
    let my_state = MyApplicationState {
        limits: StoreLimitsBuilder::new()
            .memory_size(MAX_MEMORY_SIZE /* Limit memory size here */)
            .instances(2)
            .build(),
        wasi: WasiCtxBuilder::new()
        .stdin(Box::new(stdin.clone()))
        .stdout(Box::new(stdout.clone()))
        .inherit_stderr()
        .build(),
    };

    // Instantiate engine
    // It is configured with epoch interruption enabled
    let mut config = Config::new();
    config.epoch_interruption(true);
    config.cache_config_load(CACHE_PATH).expect("could not load the cache configuration file for wasmtime");
    let engine = Arc::new(Engine::new(&config)?);

    // Instantiate linker
    let mut linker = Linker::new(&engine);

    // Put the WASI context and configure the limiter in the Store;
    let mut store = Store::new(&engine, my_state);
    store.limiter(|state| &mut state.limits);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyApplicationState| &mut state.wasi)?;

    // Configure the store to trap on reaching the epoch deadline
    store.epoch_deadline_trap();
    // Configure the store to have an initial epoch deadline one tick in the future
    store.set_epoch_deadline(1);

    // Compile the input WASM into a module
    let module = load_module(&engine, &filename)?;

    // Create instance and allocate a single page of memory for use by a running wasm module
    let instance = linker.instantiate(&mut store, &module)?;

    let memory_ty = MemoryType::new(1, None);
    Memory::new(&mut store, memory_ty)?;
    let memory = instance
        .get_memory(&mut store, MEMORY)
        .expect("expected memory not found");

    // Print informations about the memory of the WASM module
    println!(">WRAPPER imported memory size: {} pages = {} (max allowed = {}, delta = {})", memory.size(&store), memory.size(&store) * 64000, MAX_MEMORY_SIZE, MAX_MEMORY_SIZE - (memory.size(&store) * 64000) as usize);
    println!(">WRAPPER JSON input: {}", input);

    // Limit execution time of the module to MAX_EXEC_TIME ms
    // Start a thread that will bump the epoch after MAX_EXEC_TIME
    let engine_clone = engine.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(MAX_EXEC_TIME));
        engine_clone.increment_epoch();
    });

    linker
        .module(&mut store, "", &module)
        .expect("linking the function");
    linker
        .get_default(&mut store, "")
        .expect("should get the wasi runtime")
        .typed::<(), (), _>(&store)
        .expect("should type the function")
        .call(&mut store, ())
        .expect("should call the function");

    drop(store);

    let contents: Vec<u8> = stdout.try_into_inner()
    .map_err(|_err| anyhow::Error::msg("sole remaining reference"))?
    .into_inner();
    let str: String = String::from_utf8(contents.clone()).unwrap();

    println!(">WRAPPER module output: {:?}", str);

    Ok(str.to_string())
}


#[allow(dead_code)]
/* 
    LEGACY FUNCTION
    This is the first function that we implemented, it uses a more restrictive way of sharing informations with the module
 */
fn wrapped_compile_and_exec(filename_ptr: *const c_char, json_ptr: *const c_char) -> Result<String, anyhow::Error> {
    // Check that the argument pointer are valid
    if filename_ptr.is_null() {
        return Err(anyhow::anyhow!("filename_ptr is null in wrapped_compile_and_exec"));
    } else if json_ptr.is_null() {
        return Err(anyhow::anyhow!("json_ptr is null in wrapped_compile_and_exec"));
    }
    // Convert filename_ptr to String
    let filename = unsafe { CStr::from_ptr(filename_ptr).to_string_lossy().into_owned() };
    // Convert json_ptr to proper JSON input
    let input = unsafe { CStr::from_ptr(json_ptr).to_string_lossy().into_owned() };

    // Create my_state with StoreLimiter and WASI
    let my_state = MyApplicationState {
        limits: StoreLimitsBuilder::new()
            .memory_size(MAX_MEMORY_SIZE /* Limit memory size here */)
            .instances(2)
            .build(),
        wasi: WasiCtxBuilder::new()
            .inherit_stdout()
            .inherit_stderr()
            .build(),
    };

    // Instantiate engine
    // It is configured with epoch interruption enabled
    let mut config = Config::new();
    config.epoch_interruption(true);
    config.cache_config_load(CACHE_PATH).expect("could not load the cache configuration file for wasmtime");
    let engine = Arc::new(Engine::new(&config)?);

    // Instantiate linker
    let mut linker = Linker::new(&engine);

    // Put the WASI context and configure the limiter in the Store;
    let mut store = Store::new(&engine, my_state);
    store.limiter(|state| &mut state.limits);
    wasmtime_wasi::add_to_linker(&mut linker, |state: &mut MyApplicationState| &mut state.wasi)?;

    // Configure the store to trap on reaching the epoch deadline
    store.epoch_deadline_trap();
    // Configure the store to have an initial epoch deadline one tick in the future
    store.set_epoch_deadline(1);

    // Compile the input WASM into a module
    let module = Module::from_file(&engine, filename)?;

    // Create instance and allocate a single page of memory for use by a running wasm module
    let instance = linker.instantiate(&mut store, &module)?;

    let memory_ty = MemoryType::new(1, None);
    Memory::new(&mut store, memory_ty)?;
    let memory = instance
        .get_memory(&mut store, MEMORY)
        .expect("expected memory not found");

    // Print informations about the memory of the WASM module
    println!(">WRAPPER imported memory size: {} pages = {} (max allowed = {}, delta = {})", memory.size(&store), memory.size(&store) * 64000, MAX_MEMORY_SIZE, MAX_MEMORY_SIZE - (memory.size(&store) * 64000) as usize);
    println!(">WRAPPER JSON input: {}", input);


    // Limit execution time of the module to MAX_EXEC_TIME ms
    // Start a thread that will bump the epoch after MAX_EXEC_TIME
    let engine_clone = engine.clone();
    std::thread::spawn(move || {
        std::thread::sleep(std::time::Duration::from_millis(MAX_EXEC_TIME));
        engine_clone.increment_epoch();
    });

    // Allocate memory on the WASM module
    let alloc = instance
        .get_typed_func::<i32, i32, _>(&mut store, ALLOC_FN).expect("expected alloc function not found");
    let alloc_size = input.len()*2; // Alloc twice the size of the input
    let alloc_ptr = alloc.call(&mut store, alloc_size as i32)?;
    let guest_ptr_offset = alloc_ptr as isize;

    // Store the JSON in the memory of the WASM module
    println!(">WRAPPER Saving JSON input in memory");
    unsafe {
        let raw = memory.data_ptr(&mut store).offset(guest_ptr_offset);
        raw.copy_from(input.as_ptr(), input.len());
    }

    // Call the module's exported function
    let fun = instance
        .get_typed_func::<(i32, i32), i32, _>(&mut store, MAIN_FN)
        .expect("expected MAIN_FN function not found");
    let res_ptr = fun.call(&mut store, (guest_ptr_offset as i32, input.len() as i32))?;

    // Attempt to read a UTF-8 string from the memory
    let data = memory.data(&mut store)
    .get(res_ptr as u32 as usize..)
    .and_then(|arr| arr.get(..arr.iter().position(|&r| r == 0).unwrap() as u32 as usize));


    let str = match data {
        Some(data) => match std::str::from_utf8(data) {
            Ok(s) => s,
            Err(_) => return Err(anyhow::Error::msg("invalid utf-8")),
        },
        None => return Err(anyhow::Error::msg("pointer/length out of bounds")),
    };

    println!(">WRAPPER module output: {:?}", str);

    Ok(str.to_string())
}