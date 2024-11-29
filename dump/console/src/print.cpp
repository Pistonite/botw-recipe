#include <toolkit/msg/screen.hpp>
#include <gfx/seadTextWriter.h>
#include <Game/UI/uiPauseMenuDataMgr.h>
#include <Game/Actor/actWeapon.h>

#include "botw_rdump.hpp"
#include "generated.hpp"

namespace botw::rdump {

static uint64_t s_rdump_chunk_start = 0;
static uint64_t s_rdump_chunk_count = 1;
static uint64_t s_current_chunk_relative = 0;
static uint64_t s_current_record_count = 0;
static char s_current_status = 'U';
static bool s_screen_ready = false;
static uint64_t error_recipe = 0;

void compute() {
    s_screen_ready = true;
}

bool is_screen_ready() {
    return s_screen_ready;
}

void render(sead::TextWriter* w) {
    w->printf(
        "[%d/%d] C=%04d %06d/%d :%c %d",
        s_current_chunk_relative, 
        s_rdump_chunk_count,
        s_current_chunk_relative + s_rdump_chunk_start,
        s_current_record_count,
        CHUNK_SIZE,
        s_current_status,
        error_recipe
    );
}

void update_screen(uint64_t chunk_id, char status) {
    s_current_chunk_relative = chunk_id - s_rdump_chunk_start;
    s_current_status = status;
}

void update_config(uint64_t chunk_start, uint64_t chunk_count) {
    s_rdump_chunk_start = chunk_start;
    s_rdump_chunk_count = chunk_count;
}

void update_record_count(uint64_t record_count) {
    s_current_record_count = record_count;
}

void update_error_recipe(uint64_t recipe) {
    error_recipe = recipe;
}

void init_print() {
    botw::msg::screen::init(compute, render);
}

}
