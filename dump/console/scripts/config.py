import sys

# usage: python config.py <chunk_start> <chunk_end>

with open("scripts/config.txt", "w+", encoding="utf-8") as f:
    chunk_start = int(sys.argv[1])
    chunk_end = int(sys.argv[2])
    print(f"chunk_start = {chunk_start}")
    print(f"chunk_end = {chunk_end}")
    if chunk_start >= chunk_end:
        print("chunk_start must be less than chunk_end")
        exit(1)
    if chunk_start < 0:
        print("chunk_start must be non-negative")
        exit(1)
    chunk_count = chunk_end - chunk_start
    out = f"{chunk_start:04}{chunk_count:04}"
    print(f"out = {out}")
    f.write(out)

