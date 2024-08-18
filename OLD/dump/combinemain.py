SIZE = 88
RECORD_SIZE = 2
CHUNK_SIZE = 137273177 * RECORD_SIZE

current_chunk = 0
buffer = bytearray()

def format_chunk(i):
    return f"0{i}" if i < 10 else str(i)

for i in range(SIZE):
    with open(f"parts/main{i}.db", "rb") as db_file:
        print(f"reading {i}")
        incoming = db_file.read()
        incoming_len = len(incoming)
        incoming_index = 0
        buffer_len = len(buffer)
        while buffer_len + incoming_len - incoming_index > CHUNK_SIZE:
            buffer += incoming[incoming_index:CHUNK_SIZE-buffer_len]
            incoming_index += CHUNK_SIZE-buffer_len
            with open(f"data/main{format_chunk(current_chunk)}.db", "wb+") as out_file:
                print(f"Writing chunk {current_chunk}")
                out_file.write(buffer)
            buffer = bytearray()
            buffer_len = 0
            current_chunk += 1
        buffer += incoming[incoming_index:]

with open(f"data/main{format_chunk(current_chunk)}.db", "wb+") as out_file:
    print(f"Writing chunk {current_chunk}")
    out_file.write(buffer)
