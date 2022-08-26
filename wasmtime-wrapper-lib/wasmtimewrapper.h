#include <cstdarg>
#include <cstdint>
#include <cstdlib>

static const uint64_t MAX_EXEC_TIME = 80;

extern "C" {

const char *compile_and_exec(const char *filename, const char *json);

} // extern "C"
