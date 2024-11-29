#include <megaton/hook.h>
#include <Game/Cooking/cookManager.h>
#include <Game/UI/uiPauseMenuDataMgr.h>
#include "generated.hpp"
#include "botw_rdump.hpp"

namespace botw::rdump {

static volatile bool s_cookmgr_initialized = false;
static volatile int32_t s_last_crit_chance = 0;

struct hook_replace_(cookmgr_handle_crit) {
    target_offset_(0x008A0BA0);
    static void call(void* a, void* ingr, void* cook_item) {
        // don't do anything when crit happens
        return;
    }
};

struct hook_inline_(cookmgr_crit_chance) {
    target_offset_(0x008A09D8);
    static void call(megaton::hook::InlineCtx* ctx) {
        // 0x008A09D8 CMP W8, W22
        s_last_crit_chance = ctx->w<22>();
    }
};

struct hook_trampoline_(cookmgr_init) {
    target_offset_(0x008A17B0);
    static void call(void* a, void* b) {
        s_cookmgr_initialized = true;
        call_original(a, b);
    }
};

void init_cook() {
    cookmgr_handle_crit::install();
    cookmgr_crit_chance::install();
    cookmgr_init::install();
}

int32_t get_last_crit_chance() {
    return s_last_crit_chance;
}

void reset_last_crit_chance() {
    s_last_crit_chance = 0;
}

// See /app/data/src/recipe.rs
void recipe_to_groups(uint64_t id, uint64_t items[5]) {
    for (uint64_t slot = 0; slot < NUM_INGR; slot++) {
        items[slot] = 0;
    }
    // how many ids are left
    uint64_t rest = id;
    // how many items are left (since the inputs are ascending)
    uint64_t item_lower_bound = NUM_GROUPS;
    for (uint64_t slot = 0; slot < NUM_INGR; slot++) {
        /* bool good = false; */
        // compute the slot-th item in the input array
        uint64_t index = 0;
        for (uint64_t m = NUM_GROUPS - item_lower_bound + 1; m < NUM_GROUPS + 1; m++) {
            // does m overshot rest of the id
            auto next_block_size = multichoose(NUM_GROUPS-m+1, NUM_INGR-1-slot);
            if (index + next_block_size > rest) {
                items[slot] = m-1;
                /* good = true; */
                break;
            }
            index += next_block_size;
        }
        //if !good {
        //    panic!("bad recipe id: {}, when processing slot {}", usize::from(id), slot);
        //}
        item_lower_bound=NUM_GROUPS-items[slot];
        rest -= index;
    }
}

bool is_cook_ready() {
    auto* cookmgr = uking::CookingMgr::instance();
    if (!cookmgr) {
        return false;
    }
    if (!s_cookmgr_initialized) {
        return false;
    }
    return true;
}

bool cook(
    uint64_t chunk_id,
    uint64_t a1,
    uint64_t a2,
    uint64_t a3,
    uint64_t a4,
    uint64_t a5,
    uking::CookItem& out
) {
    auto* cookmgr = uking::CookingMgr::instance();
    if (!cookmgr) {
        update_screen(chunk_id, 'n');
        return false;
    }
    if (!s_cookmgr_initialized) {
        update_screen(chunk_id, 'f');
        return false;
    }
    uking::CookingMgr::BoostArg arg;
    arg.always_boost = false;
    arg.enable_random_boost = true;
    return cookmgr->cookWithItems(
        actor_name(a1),
        actor_name(a2),
        actor_name(a3),
        actor_name(a4),
        actor_name(a5),
        out,
        arg
    );
}

void convert_cook_result(const uking::CookItem& cook_item, CookData& data) {

    // hehe
    auto& cook = reinterpret_cast<uking::ui::PouchItem::CookData&>(data);

    // mLastAddedItem->getCookData().setEffectDuration(cook_item.effect_time);
    cook.setEffectDuration(cook_item.effect_time);

    // mLastAddedItem->getCookData().setHealthRecover(static_cast<int>(cook_item.life_recover));
    cook.setHealthRecover(static_cast<int>(cook_item.life_recover));

    // mLastAddedItem->getCookData().setSellPrice(cook_item.sell_price);
    cook.setSellPrice(cook_item.sell_price);

    const int level = int(cook_item.vitality_boost);
    const uking::CookEffectId effect_id = cook_item.effect_id;
    // mLastAddedItem->getCookData().setEffect({float(effect_id), float(level)});
    cook.setEffect({float(effect_id), float(level)});
}


}
