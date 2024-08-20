import multiprocessing
import sys
import os
from dump import run_dump

def init_progress(part):
    screen_x = (part % 4) * 38 + 1
    screen_y = int(part/4) + 1
    part = f"0{part}" if part < 10 else str(part)
    
    sys.stdout.write("\x1b7\x1b[%d;%df%s\x1b8" % (screen_y, screen_x, f"[{part}] 0%                               "))
    sys.stdout.flush()

if __name__ == "__main__":
    if os.name == 'nt':
        os.system('cls')
  
    # for mac and linux(here, os.name is 'posix')
    else:
        os.system('clear')
    for i in range(88):
        init_progress(i)
    with multiprocessing.Pool(32) as pool:
        for _ in pool.imap_unordered(run_dump, range(88)):
            pass

