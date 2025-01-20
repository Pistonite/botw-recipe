from build_multichoose import make_multichoose

import sys

# number of records in the chunk
RAW_CHUNK_SIZE = 409600
COMPACT_CHUNK_SIZE = 2048000

TAG = "v2"

def print_fn(tag: str, is_raw: bool, chunk_size: int, chunk_count: int, total_record: int):
    db = "RawDB" if is_raw else "CompactDB"
    db_fn = "raw" if is_raw else "compact"
    lines = [
        f"/// Get the metadata for {db} {tag}",
        "#[inline]",
        f"pub const fn {db_fn}_{tag}() -> DbMeta {{",
        f"    DbMeta {{",
        f"        is_raw: {"true" if is_raw else "false"},",
        f"        chunk_size: {chunk_size},",
        f"        chunk_count: {chunk_count},",
        f"        total_record: {total_record},",
        "    }",
        "}",
    ]
    print("\n".join(lines))

if __name__ == "__main__":
    # the number of groups, including the None group
    num = int(sys.argv[1])
    m = make_multichoose()
    total_record = m[num][5]

    # This should be impossible unless specifically trying...
    if total_record % RAW_CHUNK_SIZE == 0:
        raise ValueError("total_record % RAW_CHUNK_SIZE == 0")
    if total_record % COMPACT_CHUNK_SIZE == 0:
        raise ValueError("total_record % COMPACT_CHUNK_SIZE == 0")

    # number of chunks
    raw_chunk_count = total_record // RAW_CHUNK_SIZE + 1
    compact_chunk_count = total_record // COMPACT_CHUNK_SIZE + 1

    print_fn(TAG, True, RAW_CHUNK_SIZE, raw_chunk_count, total_record)
    print_fn(TAG, False, COMPACT_CHUNK_SIZE, compact_chunk_count, total_record)

    
