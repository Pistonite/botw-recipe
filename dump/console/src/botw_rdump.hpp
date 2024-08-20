#pragma once

#include <cstdint>

#define RDUMP_CHUNK_START 0
#define RDUMP_DUMP_CHUNKS 1

namespace nn::fs {
struct FileHandle;
}

namespace uking {
struct CookItem;
}

namespace botw::rdump {

struct CookData {
    int health_recover;
    int effect_duration;
    int sell_price;
    float effect_id;
    float effect_level;
    int crit_chance;
};
static_assert(sizeof(CookData) == 24);

/** Init printing hooks */
void init_print();
/** Update the screen */
void update_screen(uint64_t chunk_id, char status);
void update_record_count(uint64_t record_count);
void update_error_recipe(uint64_t recipe);

bool is_screen_ready();
/** Main function of the worker thread */
void worker_main(void*);
/** Start the worker thread to dump the data */
void start_worker();

/** Init cooking hooks */
void init_cook();

/** Convert recipe ID to group IDs */
void recipe_to_groups(uint64_t recipe_id, uint64_t groups[5]);

bool is_cook_ready();

/** Do the cooking */
bool cook(
    uint64_t chunk_id,
    uint64_t a1,
    uint64_t a2,
    uint64_t a3,
    uint64_t a4,
    uint64_t a5,
    uking::CookItem& out
);

/** Get the last crit chance when cook() called */
int32_t get_last_crit_chance();
void reset_last_crit_chance();

/**
 * This re-implements part of PauseMenuDataMgr::setCookDataOnLastAddedItem
 *
 * Theoretically we can hijack that function to set our data. However, that
 * function also sorts ingredient list and will cause overhead.
 * It's much easier to just reimplement the logic here.
 */
void convert_cook_result(const uking::CookItem& item, CookData& data);

/** Open a chunk file and return the handle */
bool open_chunk(uint64_t chunk_id, nn::fs::FileHandle& handle);

/** Save records to chunk. records[0] is the i-th record, save count total. */
bool save_to_chunk(
    nn::fs::FileHandle& handle, 
    uint64_t chunk_id, 
    CookData* records,
    uint64_t count,
    uint64_t i
);

/** Close the chunk file */
void close_chunk(nn::fs::FileHandle& handle);

}
