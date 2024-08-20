import json
import bitarray
from find_recipes_simple import process_recipe
import time
import sys

PROCESS_SIZE = 50000000
def execute_find_recipe(item_str, recipe_data):
    if not item_str:
        return 0, 0
    hp_crit, hp, price = process_recipe(recipe_data, item_str)
   
    # crit increases hp by 12, we just need to store if it's different
    crit_different = hp_crit != hp
    main_data = ((price << 7) + hp) & 0xFFFF
    return main_data, crit_different

NUM_INGR = 5

def array2d(first_order, second_order):
    array = [None] * first_order
    for i in range(first_order):
        array[i] = [0] * second_order
    return array

class RecipeIterator:
    def __init__(self, id_data, start, end):
        self.current = start
        self.end = end
        self.id_data = id_data
        self.num_items = len(id_data)
        data = array2d(NUM_INGR+1, self.num_items+1)
        bino = array2d(self.num_items+NUM_INGR, NUM_INGR+1)
        # binomial(n, k), k<=NUM_INGR is bino[n][k]

        # Compute binomial with dynamic programming
        for n in range(self.num_items+NUM_INGR):
            bino[n][0] = 1

        for k in range(NUM_INGR+1):
            bino[k][k] = 1

        for n in range(1,self.num_items+NUM_INGR):
            for k in range(1, NUM_INGR+1):
                bino[n][k] = bino[n-1][k-1] + bino[n-1][k]

        # data[i][m] is size of choosing i ingredients from m, so bino[i+m-1][i]
        for m in range(self.num_items+1):
            data[0][m] = 1

        for i in range(1, NUM_INGR+1):
            for m in range(self.num_items+1):
                data[i][m] = bino[i+m-1][i]
        
        self.data = data
        self.total = data[NUM_INGR][self.num_items]
    
    def get_total(self):
        return self.total
        
    def __iter__(self):
        return self
    def __next__(self):
        if self.current >= self.end:
            raise StopIteration
        input = self.current    
        self.current += 1
        
        rest_items = self.num_items
        items = []
        good = False

        for item in range(NUM_INGR):
            index = 0
            for m in range(self.num_items-rest_items+1, self.num_items+1):
                if index + self.data[NUM_INGR-1-item][self.num_items-m+1] > input:
                    items.append(m-1)
                    good = True
                    break
                
                index += self.data[NUM_INGR-1-item][self.num_items-m+1]
            
            if not good:
                break
            
            rest_items=self.num_items-items[item]
            input -= index
        
        if good:
            items = [self.id_data[i] for i in items if i != 0]
            return ",".join(items)

        else:
            raise StopIteration

sample = "[08]========================= 100%    "

        
def run_dump(part, is_multi=True):
    screen_x = (part % 4) * 38 + 1
    screen_y = int(part/4) + 1
    part_str = f"[0{part}]" if part < 10 else f"[{part}]"
    def update_progress(permillage):
        percentage = int(permillage/10)
        if percentage >= 100:
            progress_bar = "="*25
        else:
            progress_bar = "="*int(percentage/4)+">"
        if is_multi:
            sys.stdout.write("\x1b7\x1b[%d;%df%s\x1b8" % (screen_y, screen_x, f"{part_str}{progress_bar} {permillage/10}%"))
        else:
            print(f"\r{part_str}{progress_bar} {permillage/10}%", end="")
        sys.stdout.flush()
    
    # Load the items
    with open("../ids.json", "r", encoding="utf-8") as ids_file:
        id_data_dict = json.load(ids_file)

    id_data = []
    for k in id_data_dict:
        id_data.append(id_data_dict[k])

    with open("recipeData.json", "r", encoding="utf-8") as recipe_file:
        recipe_data = json.load(recipe_file)

    recipes = RecipeIterator(id_data, part*PROCESS_SIZE,(part+1)*PROCESS_SIZE)
    crit_buffer = bitarray.bitarray(endian='little')

    progress = 0
    permillage = 0
    update_progress(0)

    with open(f"parts/main{part}.db", "wb") as main_db:
        for recipe in recipes:
            main_data, crit_flag = execute_find_recipe(recipe, recipe_data)
            crit_buffer.append(crit_flag)
            main_db.write(bytearray(main_data.to_bytes(2, "big")))
            progress += 1
            new_permillage = int(progress*1000/PROCESS_SIZE)
            if new_permillage != permillage:
                update_progress(new_permillage)
                permillage = new_permillage

    update_progress(1000)
    with open(f"parts/crit{part}.db", "wb") as crit_db:
        crit_db.write(crit_buffer.tobytes())

    if not is_multi:
        print()

if __name__ == "__main__":
    run_dump(int(sys.argv[1]), False)
