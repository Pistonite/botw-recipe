#include <nn/fs.h>
#include "generated.hpp"
#include "botw_rdump.hpp"
#include "mem.h"

namespace botw::rdump {

static int64_t count_by_chunk_id(uint64_t chunk_id) {
    if (chunk_id == CHUNK_COUNT - 1) {
        return LAST_CHUNK_SIZE;
    }
    return CHUNK_SIZE;
}

static int64_t file_size_by_chunk_id(uint64_t chunk_id) {
    int64_t record_size = CHUNK_SIZE;
    if (chunk_id == CHUNK_COUNT - 1) {
        record_size = LAST_CHUNK_SIZE;
    }
    return record_size * sizeof(CookData);
}

bool open_chunk(uint64_t chunk_id, nn::fs::FileHandle &handle) {
    int t = chunk_id % 10000;
    int a = (t / 1000) % 10;
    int b = ((t % 1000) / 100) % 10;
    int c = ((t % 100) / 10) % 10;
    int d = t % 10;
    char path[] = "sd:/botwrdump/ck_0000.bin";
    //-------------0123456789ABCDEF 11
    //                             10
    path[0x11] = '0' + a;
    path[0x12] = '0' + b;
    path[0x13] = '0' + c;
    path[0x14] = '0' + d;

    int64_t file_size = file_size_by_chunk_id(chunk_id);

    nn::fs::DirectoryEntryType type;
    nn::Result result = nn::fs::GetEntryType(&type, path);

    if (result.IsFailure()) {
        // doesn't exist
        nn::Result result = nn::fs::CreateFile(path, file_size);
        if (result.IsFailure()) {
            return false;
        }
    } else {
        // must be a file
        if (type == nn::fs::DirectoryEntryType_Directory) {
            return false;
        }
    }

    result = nn::fs::OpenFile(&handle, path, nn::fs::OpenMode_ReadWrite);
    if (result.IsFailure()) {
        return false;
    }

    result = nn::fs::SetFileSize(handle, file_size);
    if (result.IsFailure()) {
        nn::fs::CloseFile(handle);
        return false;
    }

    return true;
}

bool save_to_chunk(
    nn::fs::FileHandle& handle, 
    uint64_t chunk_id, 
    CookData* records,
    uint64_t count,
    uint64_t record_start
) {
    uint64_t chunk_size = count_by_chunk_id(chunk_id);
    int8_t buffer[sizeof(CookData)];
    for (uint64_t i = 0; i < count; i++) {
        if (record_start + i >= chunk_size) {
            break;
        }
        CookData* item = records + i;
        memcpy(buffer, item, sizeof(CookData));
        nn::Result result = nn::fs::WriteFile(
            handle, 
            (record_start + i) * sizeof(CookData), 
            buffer, sizeof(CookData),
            nn::fs::WriteOption::CreateOption({})
        );
        if (result.IsFailure()) {
            return false;
        }
    }
    nn::Result result = nn::fs::FlushFile(handle);
    return result.IsSuccess();
}

void close_chunk(nn::fs::FileHandle& handle) {
    nn::fs::CloseFile(handle);
}

}
