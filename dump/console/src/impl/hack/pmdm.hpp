#pragma once

namespace sead {
template <typename T>
class OffsetList;
}

namespace uking::ui {
    class PauseMenuDataMgr;
    class PouchItem;
}

namespace botw::rdump::hack {

class PmdmHack {

public:
    PmdmHack(uking::ui::PauseMenuDataMgr* pmdm) : inner(pmdm) {}

    uking::ui::PauseMenuDataMgr* inner;
    uking::ui::PouchItem* get_grabbed_item(int idx);
    void get_equipped_items(uking::ui::PouchItem** output);
    sead::OffsetList<uking::ui::PouchItem>& items();
};

}
