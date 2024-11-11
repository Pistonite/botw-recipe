import zlib
import sys
import os
import subprocess
import shutil
from multiprocessing import Pool

def calc(chunk):
    outputs = []
    for s in chunk:
        hash = zlib.crc32(bytes(s, "utf-8"))
        outputs.append((hash, s))
    return outputs

if __name__ == "__main__":

    DIR = os.path.dirname(__file__)
    DATA = os.path.join(DIR, "data")
    WORDS_SMALL = os.path.join(DIR, "data", "words.txt")
    WORDS = os.path.join(DIR, "data", "words_alpha.txt")
    WORDS_OUT = os.path.join(DIR, "data", "words_out.txt")
    WORDS_ZIP = os.path.join(DIR, "data", "words_alpha.zip")
    WORDS_URL = "https://github.com/dwyl/english-words/raw/refs/heads/master/words_alpha.zip"
    WORDS_SMALL_URL = "https://raw.githubusercontent.com/david47k/top-english-wordlists/master/top_english_words_lower_10000.txt"
    WGET = shutil.which("wget")
    _7Z = shutil.which("7z")

    if not WGET or not _7Z:
        print("please install wget and 7z")
        sys.exit(1)

    # if not os.path.exists(WORDS_OUT):
    #     subprocess.run([WGET, "-O", WORDS_ZIP, WORDS_URL], check=True)
    #     subprocess.run([_7Z, "x", "-y", WORDS_ZIP, f"-o{DATA}"], check=True)

    if not os.path.exists(WORDS_SMALL):
        subprocess.run([WGET, "-O", WORDS_SMALL, WORDS_SMALL_URL], check=True)

    print("loading words...")
    words = []
    with open(WORDS_SMALL, "r", encoding="utf-8") as f:
        for line in f:
            w = line.strip()
            w = w[0].upper() + w[1:]
            words.append(w)
    
    words_len = len(words)
    print(f"loaded {words_len} words")
    with open(WORDS_OUT, "w", encoding="utf-8") as f:
        for w in words:
            f.write(w + "\n")
    #
    # def total(l):
    #     return words_len ** (l-1)
    #
    # def permutation(l): # yields chunks of words
    #     if l <= 1:
    #         yield words
    #     else:
    #         for chunk in permutation(l-1):
    #             for w2 in chunk:
    #                 yield [w + w2 for w in words]
    #
    # inputs = [int(hash, 0) for hash in sys.argv[1:]]
    # print(f"finding for {inputs}")
    #
    # answers = {}
    #
    # try:
    #     done = False
    #     with Pool() as p:
    #         for l in range(1, 3):
    #             for outputs in util.progress(p.imap_unordered(calc, permutation(l)), f"checking length {l}", total(l)):
    #                 for crc32, word in outputs:
    #                     if crc32 in inputs:
    #                         answers[crc32] = word
    #                         inputs = [x for x in inputs if x != crc32]
    #                         if not inputs:
    #                             done = True
    #                             break
    #                 if done:
    #                     break
    #             if done:
    #                 break
    # except:
    #     print("interrupted")
    #     print("answers so far:")
    #     print(answers)
    #     raise
    #
    # print(answers)
    #
    #
    #
    #
    #
