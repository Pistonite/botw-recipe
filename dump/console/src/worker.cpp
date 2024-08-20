#include <nn/os.h>
#include <nn/time.h>
#include <nn/fs.h>
#include <exl/types.h>
#include <Game/Cooking/cookManager.h>

#include "botw_rdump.hpp"
#include "generated.hpp"
#include "mem.h"

namespace botw::rdump {
static nn::os::ThreadType s_thread;

void worker_main(void*) {
    while (true) {
        if (is_cook_ready() && is_screen_ready()) {
            break;
        }
        nn::os::YieldThread();
        nn::os::SleepThread(nn::TimeSpan::FromSeconds(5));
    }
    update_screen(RDUMP_CHUNK_START, 'R');
    for (int i = 5; i >= 0; i--) {
        update_screen(RDUMP_CHUNK_START, i + '0');
        nn::os::SleepThread(nn::TimeSpan::FromSeconds(1));
    }
    // Start!
    uint64_t group[NUM_INGR];
    // iterate each chunk
    uint64_t chunk_id = RDUMP_CHUNK_START;
    bool error = false;
    for (; chunk_id < RDUMP_CHUNK_START + RDUMP_DUMP_CHUNKS; chunk_id++) {
        nn::fs::FileHandle handle;
        update_screen(chunk_id, 'W');
        if (!open_chunk(chunk_id, handle)) {
            update_screen(chunk_id, 'o');
            update_error_recipe(0);
            error = true;
            break;
        }
        // iterate each recipe in the chunk
        uint64_t r_base = chunk_id * CHUNK_SIZE;
        uking::CookItem cook_item;
        static_assert(CHUNK_SIZE % 100 == 0);
        constexpr uint64_t batch_size = CHUNK_SIZE / 100;
        CookData records[batch_size];
        uint64_t batch_i = 0;
        uint64_t chunk_record_start = 0;
        for (uint64_t i = 0; i < CHUNK_SIZE; i++) {
            update_record_count(i);
            uint64_t recipe_id = r_base + i;
            if (recipe_id >= NUM_TOTAL_RECORDS) {
                break;
            }
            if (recipe_id == 0) {
                records[batch_i].health_recover = 0;
                records[batch_i].effect_duration = 0;
                records[batch_i].sell_price = 0;
                records[batch_i].effect_id = 0;
                records[batch_i].effect_level = 0;
                records[batch_i].crit_chance = 0;
            } else {
                recipe_to_groups(recipe_id, group);
                cook_item.reset();
                reset_last_crit_chance();
                if (!cook(chunk_id, group[0], group[1], group[2], group[3], group[4], cook_item)) {
                    update_error_recipe(i);
                    error = true;
                    break;
                }
                int32_t crit_chance = get_last_crit_chance();
                convert_cook_result(cook_item, records[batch_i]);
                records[batch_i].crit_chance = crit_chance;
            }
            batch_i++;
            if (batch_i == batch_size) {
                update_screen(chunk_id, 'S');
                if (!save_to_chunk(
                    handle, 
                    chunk_id, 
                    records, 
                    batch_size, chunk_record_start
                )) {
                    update_screen(chunk_id, 's');
                    update_error_recipe(i);
                    error = true;
                    break;
                }
                update_screen(chunk_id, 'D');
                chunk_record_start += batch_size;
                batch_i = 0;
            }
        }
        if (batch_i > 0) {
            update_screen(chunk_id, 'S');
            if (!save_to_chunk(
                handle, 
                chunk_id, 
                records, 
                batch_i, chunk_record_start
            )) {
                update_screen(chunk_id, 's');
                update_error_recipe(CHUNK_SIZE);
                error = true;
            }
            update_screen(chunk_id, 'D');
        }
        if (!error) {
            update_screen(chunk_id, 'O');
        } else {
            break;
        }
    }
    if (!error) {
        update_record_count(CHUNK_SIZE);
        update_screen(chunk_id, 'Y');
    }
    while (true) {
        nn::os::YieldThread();
        nn::os::SleepThread(nn::TimeSpan::FromSeconds(5));
    }
}

void start_worker() {
    const u64 STACK_SIZE = 0x80000;
    void* thread_stack = memalign(0x1000, STACK_SIZE);

    nn::Result result =
        nn::os::CreateThread(&s_thread, worker_main, nullptr, thread_stack, STACK_SIZE, 0);
    if (result.IsFailure()) {
        return;
    }
    nn::os::StartThread(&s_thread);
}
}
