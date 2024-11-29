#include <megaton/prelude.h>
#include <nn/fs.h>

#include "botw_rdump.hpp"

extern "C" void megaton_main() {
    nn::fs::MountSdCardForDebug("sd");
    botw::rdump::init_print();
    botw::rdump::init_cook();
    botw::rdump::start_worker();
}
