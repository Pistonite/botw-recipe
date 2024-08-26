#include <nn/os.h>
#include <nn/time.h>
#include <nn/fs.h>
#include <exl/types.h>
#include <Game/Cooking/cookManager.h>

#include "botw_rdump.hpp"
#include "generated.hpp"
#include "mem.h"

#define STACK_SIZE 0x80000
#define BATCH_SIZE 8192
static_assert(CHUNK_SIZE % BATCH_SIZE == 0);
static_assert(sizeof(botw::rdump::CookData) * BATCH_SIZE * 2 < STACK_SIZE);

namespace botw::rdump {
static nn::os::ThreadType s_thread;

static void trap() {
    while (true) {
        nn::os::YieldThread();
        nn::os::SleepThread(nn::TimeSpan::FromSeconds(5));
    }
}

static uint64_t parse_int(const char buf[4]) {
    uint64_t out = 0;
    for (int i = 0; i < 4; i++) {
        uint64_t digit = buf[i] - '0';
        out = out * 10 + digit;
    }
    return out;
}

void worker_main(void*) {
    while (true) {
        if (is_cook_ready() && is_screen_ready()) {
            break;
        }
        nn::os::YieldThread();
        nn::os::SleepThread(nn::TimeSpan::FromSeconds(5));
    }
    update_screen(0, 'L');
    // Read config file
    nn::fs::FileHandle cfg_handle;
    nn::Result result = nn::fs::OpenFile(&cfg_handle, "sd:/botwrdump/config.txt", nn::fs::OpenMode_Read);
    if (result.IsFailure()) {
        update_screen(0, '^');
        trap();
    }
    uint64_t size_read;
    char  buf[8];
    result = nn::fs::ReadFile(&size_read, cfg_handle, 0, buf, 8);
    if (result.IsFailure()) {
        update_screen(0, '7');
        trap();
    }
    if (size_read != 8) {
        update_screen(0, '8');
        trap();
    }

    uint64_t chunk_start = parse_int(buf);
    uint64_t chunk_count = parse_int(buf + 4);
    if (chunk_start >= CHUNK_COUNT || chunk_start + chunk_count > CHUNK_COUNT) {
        update_screen(0, '9');
        trap();
    }

    nn::fs::CloseFile(cfg_handle);
    update_config(chunk_start, chunk_count);
    update_screen(chunk_start, 'R');
    for (int i = 5; i >= 0; i--) {
        nn::os::SleepThread(nn::TimeSpan::FromSeconds(1));
        update_screen(chunk_start, i + '0');
    }
    nn::os::SleepThread(nn::TimeSpan::FromSeconds(1));

    // Start!
    uint64_t group[NUM_INGR];
    // iterate each chunk
    uint64_t chunk_id = chunk_start;
    bool error = false;
    for (; chunk_id < chunk_start + chunk_count; chunk_id++) {
        nn::fs::FileHandle handle;
        update_screen(chunk_id, 'W');
        if (!open_chunk(chunk_id, handle)) {
            update_screen(chunk_id, 'o');
            update_error_recipe(0);
            error = true;
            break;
        }
        update_screen(chunk_id, 'D');
        // iterate each recipe in the chunk
        uint64_t r_base = chunk_id * CHUNK_SIZE;
        uking::CookItem cook_item;
        static_assert(CHUNK_SIZE % 100 == 0);
        CookData records[BATCH_SIZE];
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
                    update_screen(chunk_id, '<');
                    update_error_recipe(i);
                    error = true;
                    break;
                }
                int32_t crit_chance = get_last_crit_chance();
                if (cook_item.is_crit) {
                    // crit handler is turned off so this should never be true
                    // check here to make sure nothing went wrong
                    update_screen(chunk_id, 'x');
                    update_error_recipe(i);
                    error = true;
                    break;
                }
                convert_cook_result(cook_item, records[batch_i]);
                records[batch_i].crit_chance = crit_chance;
            }
            batch_i++;
            if (batch_i == BATCH_SIZE) {
                if (!save_to_chunk(
                    handle, 
                    chunk_id, 
                    records, 
                    BATCH_SIZE, chunk_record_start
                )) {
                    update_screen(chunk_id, 's');
                    update_error_recipe(i);
                    error = true;
                    break;
                }
                chunk_record_start += BATCH_SIZE;
                batch_i = 0;
            }
        }
        if (batch_i > 0) {
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
        }
        if (error) {
            break;
        }
        close_chunk(handle);
    }
    if (!error) {
        update_record_count(CHUNK_SIZE);
        update_screen(chunk_id, 'Y');
    }
    trap();
}

void start_worker() {
    void* thread_stack = memalign(0x1000, STACK_SIZE);

    nn::Result result =
        nn::os::CreateThread(&s_thread, worker_main, nullptr, thread_stack, STACK_SIZE, 0);
    if (result.IsFailure()) {
        return;
    }
    nn::os::StartThread(&s_thread);
}

}
