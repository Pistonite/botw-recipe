#include <container/seadSafeArray.h>
#define private public
#include <Game/UI/uiPauseMenuDataMgr.h>
#undef private
#include "pmdm.hpp"

namespace botw::rdump::hack {

uking::ui::PouchItem* PmdmHack::get_grabbed_item(int idx) {
    return inner->mGrabbedItems[idx].item;
}

void PmdmHack::get_equipped_items(uking::ui::PouchItem** output) {
    inner->updateEquippedItemArray();
    for (int i = 0; i < 4; i++) {
        output[i] = inner->mEquippedWeapons[i];
    }
}

sead::OffsetList<uking::ui::PouchItem>& PmdmHack::items() {
    return inner->getItems();
}

}
