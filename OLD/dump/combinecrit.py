SIZE = 88
with open("data/crit.db", "wb+") as out_file: 
    for i in range(SIZE):
        with open(f"parts/crit{i}.db", "rb") as db_file:
            out_file.write(db_file.read())
