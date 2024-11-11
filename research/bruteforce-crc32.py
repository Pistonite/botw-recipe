import zlib
import util
import sys
import os
import subprocess
from multiprocessing import Pool

def calc(chunk):
    outputs = []
    for s in chunk:
        hash = zlib.crc32(bytes(s, "utf-8"))
        outputs.append((hash, s))
    return outputs

if __name__ == "__main__":

    DIR = os.path.dirname(__file__)
    WORDS = os.path.join(DIR, "words_alpha.txt")
    WORDS_ZIP = os.path.join(DIR, "words_alpha.zip")
    WORDS_URL = "https://github.com/dwyl/english-words/raw/refs/heads/master/words_alpha.zip"
    WGET = util.which("wget")
    _7Z = util.which("7z")

    if not os.path.exists(WORDS):
        subprocess.run([WGET, "-O", WORDS_ZIP, WORDS_URL], check=True)
        subprocess.run([_7Z, "x", "-y", WORDS_ZIP], check=True)

    print("loading words...")
    words = []
    with open(WORDS, "r", encoding="utf-8") as f:
        for line in f:
            w = line.strip()
            w = w[0].upper() + w[1:]
            words.append(w)

    words_len = len(words)
    print(f"loaded {words_len} words")

    def total(l):
        return words_len ** (l-1)

    def permutation(l): # yields chunks of words
        if l <= 1:
            yield words
        else:
            for chunk in permutation(l-1):
                for w2 in chunk:
                    yield [w + w2 for w in words]

    inputs = [int(hash, 0) for hash in sys.argv[1:]]
    print(f"finding for {inputs}")

    answers = {}

    try:
        done = False
        with Pool() as p:
            for l in range(1, 3):
                for outputs in util.progress(p.imap_unordered(calc, permutation(l)), f"checking length {l}", total(l)):
                    for crc32, word in outputs:
                        if crc32 in inputs:
                            answers[crc32] = word
                            inputs = [x for x in inputs if x != crc32]
                            if not inputs:
                                done = True
                                break
                    if done:
                        break
                if done:
                    break
    except:
        print("interrupted")
        print("answers so far:")
        print(answers)
        raise

    print(answers)





