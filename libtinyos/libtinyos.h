#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>




#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void __c_exit(int64_t status);

ptrdiff_t __c_read(uint32_t handle, uint8_t *buf, size_t len, size_t timeout);

ptrdiff_t __c_write(uint32_t handle, const uint8_t *buf, size_t len);

void __c_yield(void);

void __print(const char *buf);

void free(uint8_t *ptr);

uint8_t *malloc(size_t size);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
