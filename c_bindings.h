#include <stdarg.h>
#include <stdbool.h>
#include <stddef.h>
#include <stdint.h>




#ifdef __cplusplus
extern "C" {
#endif // __cplusplus

void __c_exit(uint64_t status);

uint8_t *__c_heap(size_t size);

ptrdiff_t __c_read(size_t handle, uint8_t *buf, size_t len);

ptrdiff_t __c_write(size_t handle, const uint8_t *buf, size_t len);

void __c_yield(void);

void __print(const char *buf);

#ifdef __cplusplus
}  // extern "C"
#endif  // __cplusplus
