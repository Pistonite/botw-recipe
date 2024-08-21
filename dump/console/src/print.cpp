#include <exl/lib.hpp>
#include <botwpp/screen.hpp>
#include <gfx/seadTextWriter.h>
#include <Game/UI/uiPauseMenuDataMgr.h>
#include <Game/Actor/actWeapon.h>

#ifdef RDUMP_PMDM_DEBUG
#include "impl/hack/pmdm.hpp"
#endif
#include "botw_rdump.hpp"
#include "generated.hpp"

namespace botw::rdump {

#ifdef RDUMP_PMDM_DEBUG
static uking::ui::PouchItem* s_grabbed_items[5];
static uking::ui::PouchItem* s_equipped_items[4];
static sead::FixedSafeString<64> s_last_add_to_pouch_name;
static sead::FixedSafeString<64> s_slate_name;
static void compute_pmdm_debug_info() {
    auto pmdm = hack::PmdmHack(uking::ui::PauseMenuDataMgr::instance());
    if (!pmdm.inner) {
        for (int i = 0; i < 5; i++) {
            s_grabbed_items[i] = nullptr;
        }
        for (int i = 0; i < 4; i++) {
            s_equipped_items[i] = nullptr;
        }
    } else {
        for (int i = 0; i < 5; i++) {
            s_grabbed_items[i] = pmdm.get_grabbed_item(i);
        }
        pmdm.get_equipped_items(s_equipped_items);
    }
    s_slate_name.clear();
    for (auto& item: pmdm.items()) {
        bool found = false;
        for (const auto* name : {
            "Dm_Item_Conductor", 
            "Item_ConductorDemo", 
            "Obj_DRStone_A_01", "Obj_DRStone_A_02",
            "Obj_DRStone_Get", "Obj_SheikPadLv2"
        }) {
            if (item.getName() == name) {
                s_slate_name = name;
                found = true;
                break;
            }
        }
        if (found) {
            break;
        }
    }
}

void render_pmdm_debug_info(sead::TextWriter* w) {
    w->printf("Grabbed Items  %s,%s,%s,%s,%s\n",
              s_grabbed_items[0] ? s_grabbed_items[0]->getName().cstr() : "<none>",
              s_grabbed_items[1] ? s_grabbed_items[1]->getName().cstr() : "<none>",
              s_grabbed_items[2] ? s_grabbed_items[2]->getName().cstr() : "<none>",
              s_grabbed_items[3] ? s_grabbed_items[3]->getName().cstr() : "<none>",
              s_grabbed_items[4] ? s_grabbed_items[4]->getName().cstr() : "<none>"
              );
    w->printf("Equipped Items %s,%s,%s,%s",
              s_equipped_items[0] ? s_equipped_items[0]->getName().cstr() : "<none>",
              s_equipped_items[1] ? s_equipped_items[1]->getName().cstr() : "<none>",
              s_equipped_items[2] ? s_equipped_items[2]->getName().cstr() : "<none>",
              s_equipped_items[3] ? s_equipped_items[3]->getName().cstr() : "<none>"
              );
    w->printf("Last addToPouch: %s\n", s_last_add_to_pouch_name.cstr());
    w->printf("Slate: %s\n", s_slate_name.cstr());
}

HOOK_DEFINE_TRAMPOLINE(pmdm_add_to_pouch) {
    static void Callback(
        uking::ui::PauseMenuDataMgr* _this,
        const sead::SafeString& name,
        uking::ui::PouchItemType type,
        void* lists,
        int value,
        bool equipped,
        const uking::act::WeaponModifierInfo* modifier,
        bool is_inventory_load) {
        s_last_add_to_pouch_name = name;
        Orig(_this, name, type, lists, value, equipped, modifier, is_inventory_load);
    }
};
#endif


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
    botwpp::init_debug_print(compute, render);

    /* pmdm_add_to_pouch::InstallAtOffset(0x0096f268); */
}

}
