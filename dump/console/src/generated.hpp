
//! Automatically generated.
//!
//! DO NOT EDIT
//!
//! Run `cd research && python main.py` (or `task research`) to regenerate.

#pragma once
#include <cstdint>
#define NUM_GROUPS 175
#define NUM_INGR 5
#define NUM_TOTAL_RECORDS 1447490660
#define CHUNK_SIZE 409600
#define CHUNK_COUNT 3534
#define LAST_CHUNK_SIZE 373860
namespace botw::rdump {
uint64_t multichoose(uint64_t n, uint64_t k);
const char* actor_name(uint64_t group);
}
