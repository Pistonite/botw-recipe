/**
 * @file mem.h
 * @brief Memory functions.
 */

#pragma once

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

void* memset(void* src, int val, uint64_t num);
void* memcpy(void* dest, void const* src, uint64_t count);
void* memmove(void* dest, const void* src, uint64_t count);
void* memalign(size_t alignment, size_t size);

#ifdef __cplusplus
}
#endif
