#define FFI_SCOPE "libwasmtimewrapper"
#define FFI_LIB "../wasmtime-wrapper-lib/target/release/libwasmtimewrapper.so"

const char * compile_and_exec(const char *filename, const char *funcname);