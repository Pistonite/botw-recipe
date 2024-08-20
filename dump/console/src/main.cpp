#include <exl/lib.hpp>
#include <nn/fs.h>

#include "botw_rdump.hpp"

extern "C" void exl_main(void* x0, void* x1) {
    exl::hook::Initialize();
    nn::fs::MountSdCardForDebug("sd");

    botw::rdump::init_print();
    botw::rdump::init_cook();
    botw::rdump::start_worker();
}

extern "C" NORETURN void exl_exception_entry() {
    EXL_ABORT(0x420);
}
