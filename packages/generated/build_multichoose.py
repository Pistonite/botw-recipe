from build import write_rust_source, src_file
MAX_N = 234
MAX_K = 5

def generate_multichoose():
    multichoose = make_multichoose()
    lines = [
        "/// MAX N to feed into multichoose",
        f"pub(crate) const MAX_N: u32 = {MAX_N};",
        "/// MAX K to feed into multichoose",
        f"pub(crate) const MAX_K: u32 = {MAX_K};",
        "/// Pre-computed multichoose(n, k) values",
        "///",
        "/// MULTICHOOSE[n][k] is the number of ways to choose k items from n items with repetition.",
        f"pub(crate) const MULTICHOOSE: [[u64; {MAX_K+1}]; {MAX_N+1}] = [",
    ]
    for multichoose_n in multichoose:
        lines.append("[")
        for k in multichoose_n:
            lines.append(f"{k}, ")
        lines.append("],")
    lines.append("];")

    write_rust_source(src_file("multichoose", "gen.rs"), lines)

def make_multichoose():
    """
    Precompute all multichoose[n][k] using Dynamic Programming
    """
    # bionmial(n, k), k<=MAX_K is bino[n][k]
    bino = []
    for _ in range(MAX_N+MAX_K):
        bino.append([0]*(MAX_K+1))
    for n in range(MAX_N+MAX_K):
        bino[n][0] = 1

    for k in range(MAX_K+1):
        bino[k][k] = 1

    for n in range(1,MAX_N+MAX_K):
        for k in range(1,MAX_K+1):
            bino[n][k] = bino[n-1][k-1] + bino[n-1][k]
    # multichoose(n, k) is multichoose[n][k]
    multichoose = []
    for _ in range(MAX_N+1):
        multichoose.append([0]*(MAX_K+1))
    for n in range(MAX_N+1):
        multichoose[n][0] = 1
    for k in range(1, MAX_K+1):
        for n in range(MAX_N+1):
            multichoose[n][k] = bino[n+k-1][k]
    return multichoose


if  __name__ == "__main__":
    print(f"{MAX_N=} {MAX_K=}")
    generate_multichoose()

