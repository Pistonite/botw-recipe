
//! Automatically generated.
//!
//! DO NOT EDIT
//!
//! Run `cd research && python main.py` (or `task research`) to regenerate.

#include "generated.hpp"
namespace botw::rdump {
static uint64_t MULTICHOOSE[NUM_GROUPS+1][NUM_INGR+1] = {
    { 1, 0, 0, 0, 0, 0 },
    { 1, 1, 1, 1, 1, 1 },
    { 1, 2, 3, 4, 5, 6 },
    { 1, 3, 6, 10, 15, 21 },
    { 1, 4, 10, 20, 35, 56 },
    { 1, 5, 15, 35, 70, 126 },
    { 1, 6, 21, 56, 126, 252 },
    { 1, 7, 28, 84, 210, 462 },
    { 1, 8, 36, 120, 330, 792 },
    { 1, 9, 45, 165, 495, 1287 },
    { 1, 10, 55, 220, 715, 2002 },
    { 1, 11, 66, 286, 1001, 3003 },
    { 1, 12, 78, 364, 1365, 4368 },
    { 1, 13, 91, 455, 1820, 6188 },
    { 1, 14, 105, 560, 2380, 8568 },
    { 1, 15, 120, 680, 3060, 11628 },
    { 1, 16, 136, 816, 3876, 15504 },
    { 1, 17, 153, 969, 4845, 20349 },
    { 1, 18, 171, 1140, 5985, 26334 },
    { 1, 19, 190, 1330, 7315, 33649 },
    { 1, 20, 210, 1540, 8855, 42504 },
    { 1, 21, 231, 1771, 10626, 53130 },
    { 1, 22, 253, 2024, 12650, 65780 },
    { 1, 23, 276, 2300, 14950, 80730 },
    { 1, 24, 300, 2600, 17550, 98280 },
    { 1, 25, 325, 2925, 20475, 118755 },
    { 1, 26, 351, 3276, 23751, 142506 },
    { 1, 27, 378, 3654, 27405, 169911 },
    { 1, 28, 406, 4060, 31465, 201376 },
    { 1, 29, 435, 4495, 35960, 237336 },
    { 1, 30, 465, 4960, 40920, 278256 },
    { 1, 31, 496, 5456, 46376, 324632 },
    { 1, 32, 528, 5984, 52360, 376992 },
    { 1, 33, 561, 6545, 58905, 435897 },
    { 1, 34, 595, 7140, 66045, 501942 },
    { 1, 35, 630, 7770, 73815, 575757 },
    { 1, 36, 666, 8436, 82251, 658008 },
    { 1, 37, 703, 9139, 91390, 749398 },
    { 1, 38, 741, 9880, 101270, 850668 },
    { 1, 39, 780, 10660, 111930, 962598 },
    { 1, 40, 820, 11480, 123410, 1086008 },
    { 1, 41, 861, 12341, 135751, 1221759 },
    { 1, 42, 903, 13244, 148995, 1370754 },
    { 1, 43, 946, 14190, 163185, 1533939 },
    { 1, 44, 990, 15180, 178365, 1712304 },
    { 1, 45, 1035, 16215, 194580, 1906884 },
    { 1, 46, 1081, 17296, 211876, 2118760 },
    { 1, 47, 1128, 18424, 230300, 2349060 },
    { 1, 48, 1176, 19600, 249900, 2598960 },
    { 1, 49, 1225, 20825, 270725, 2869685 },
    { 1, 50, 1275, 22100, 292825, 3162510 },
    { 1, 51, 1326, 23426, 316251, 3478761 },
    { 1, 52, 1378, 24804, 341055, 3819816 },
    { 1, 53, 1431, 26235, 367290, 4187106 },
    { 1, 54, 1485, 27720, 395010, 4582116 },
    { 1, 55, 1540, 29260, 424270, 5006386 },
    { 1, 56, 1596, 30856, 455126, 5461512 },
    { 1, 57, 1653, 32509, 487635, 5949147 },
    { 1, 58, 1711, 34220, 521855, 6471002 },
    { 1, 59, 1770, 35990, 557845, 7028847 },
    { 1, 60, 1830, 37820, 595665, 7624512 },
    { 1, 61, 1891, 39711, 635376, 8259888 },
    { 1, 62, 1953, 41664, 677040, 8936928 },
    { 1, 63, 2016, 43680, 720720, 9657648 },
    { 1, 64, 2080, 45760, 766480, 10424128 },
    { 1, 65, 2145, 47905, 814385, 11238513 },
    { 1, 66, 2211, 50116, 864501, 12103014 },
    { 1, 67, 2278, 52394, 916895, 13019909 },
    { 1, 68, 2346, 54740, 971635, 13991544 },
    { 1, 69, 2415, 57155, 1028790, 15020334 },
    { 1, 70, 2485, 59640, 1088430, 16108764 },
    { 1, 71, 2556, 62196, 1150626, 17259390 },
    { 1, 72, 2628, 64824, 1215450, 18474840 },
    { 1, 73, 2701, 67525, 1282975, 19757815 },
    { 1, 74, 2775, 70300, 1353275, 21111090 },
    { 1, 75, 2850, 73150, 1426425, 22537515 },
    { 1, 76, 2926, 76076, 1502501, 24040016 },
    { 1, 77, 3003, 79079, 1581580, 25621596 },
    { 1, 78, 3081, 82160, 1663740, 27285336 },
    { 1, 79, 3160, 85320, 1749060, 29034396 },
    { 1, 80, 3240, 88560, 1837620, 30872016 },
    { 1, 81, 3321, 91881, 1929501, 32801517 },
    { 1, 82, 3403, 95284, 2024785, 34826302 },
    { 1, 83, 3486, 98770, 2123555, 36949857 },
    { 1, 84, 3570, 102340, 2225895, 39175752 },
    { 1, 85, 3655, 105995, 2331890, 41507642 },
    { 1, 86, 3741, 109736, 2441626, 43949268 },
    { 1, 87, 3828, 113564, 2555190, 46504458 },
    { 1, 88, 3916, 117480, 2672670, 49177128 },
    { 1, 89, 4005, 121485, 2794155, 51971283 },
    { 1, 90, 4095, 125580, 2919735, 54891018 },
    { 1, 91, 4186, 129766, 3049501, 57940519 },
    { 1, 92, 4278, 134044, 3183545, 61124064 },
    { 1, 93, 4371, 138415, 3321960, 64446024 },
    { 1, 94, 4465, 142880, 3464840, 67910864 },
    { 1, 95, 4560, 147440, 3612280, 71523144 },
    { 1, 96, 4656, 152096, 3764376, 75287520 },
    { 1, 97, 4753, 156849, 3921225, 79208745 },
    { 1, 98, 4851, 161700, 4082925, 83291670 },
    { 1, 99, 4950, 166650, 4249575, 87541245 },
    { 1, 100, 5050, 171700, 4421275, 91962520 },
    { 1, 101, 5151, 176851, 4598126, 96560646 },
    { 1, 102, 5253, 182104, 4780230, 101340876 },
    { 1, 103, 5356, 187460, 4967690, 106308566 },
    { 1, 104, 5460, 192920, 5160610, 111469176 },
    { 1, 105, 5565, 198485, 5359095, 116828271 },
    { 1, 106, 5671, 204156, 5563251, 122391522 },
    { 1, 107, 5778, 209934, 5773185, 128164707 },
    { 1, 108, 5886, 215820, 5989005, 134153712 },
    { 1, 109, 5995, 221815, 6210820, 140364532 },
    { 1, 110, 6105, 227920, 6438740, 146803272 },
    { 1, 111, 6216, 234136, 6672876, 153476148 },
    { 1, 112, 6328, 240464, 6913340, 160389488 },
    { 1, 113, 6441, 246905, 7160245, 167549733 },
    { 1, 114, 6555, 253460, 7413705, 174963438 },
    { 1, 115, 6670, 260130, 7673835, 182637273 },
    { 1, 116, 6786, 266916, 7940751, 190578024 },
    { 1, 117, 6903, 273819, 8214570, 198792594 },
    { 1, 118, 7021, 280840, 8495410, 207288004 },
    { 1, 119, 7140, 287980, 8783390, 216071394 },
    { 1, 120, 7260, 295240, 9078630, 225150024 },
    { 1, 121, 7381, 302621, 9381251, 234531275 },
    { 1, 122, 7503, 310124, 9691375, 244222650 },
    { 1, 123, 7626, 317750, 10009125, 254231775 },
    { 1, 124, 7750, 325500, 10334625, 264566400 },
    { 1, 125, 7875, 333375, 10668000, 275234400 },
    { 1, 126, 8001, 341376, 11009376, 286243776 },
    { 1, 127, 8128, 349504, 11358880, 297602656 },
    { 1, 128, 8256, 357760, 11716640, 309319296 },
    { 1, 129, 8385, 366145, 12082785, 321402081 },
    { 1, 130, 8515, 374660, 12457445, 333859526 },
    { 1, 131, 8646, 383306, 12840751, 346700277 },
    { 1, 132, 8778, 392084, 13232835, 359933112 },
    { 1, 133, 8911, 400995, 13633830, 373566942 },
    { 1, 134, 9045, 410040, 14043870, 387610812 },
    { 1, 135, 9180, 419220, 14463090, 402073902 },
    { 1, 136, 9316, 428536, 14891626, 416965528 },
    { 1, 137, 9453, 437989, 15329615, 432295143 },
    { 1, 138, 9591, 447580, 15777195, 448072338 },
    { 1, 139, 9730, 457310, 16234505, 464306843 },
    { 1, 140, 9870, 467180, 16701685, 481008528 },
    { 1, 141, 10011, 477191, 17178876, 498187404 },
    { 1, 142, 10153, 487344, 17666220, 515853624 },
    { 1, 143, 10296, 497640, 18163860, 534017484 },
    { 1, 144, 10440, 508080, 18671940, 552689424 },
    { 1, 145, 10585, 518665, 19190605, 571880029 },
    { 1, 146, 10731, 529396, 19720001, 591600030 },
    { 1, 147, 10878, 540274, 20260275, 611860305 },
    { 1, 148, 11026, 551300, 20811575, 632671880 },
    { 1, 149, 11175, 562475, 21374050, 654045930 },
    { 1, 150, 11325, 573800, 21947850, 675993780 },
    { 1, 151, 11476, 585276, 22533126, 698526906 },
    { 1, 152, 11628, 596904, 23130030, 721656936 },
    { 1, 153, 11781, 608685, 23738715, 745395651 },
    { 1, 154, 11935, 620620, 24359335, 769754986 },
    { 1, 155, 12090, 632710, 24992045, 794747031 },
    { 1, 156, 12246, 644956, 25637001, 820384032 },
    { 1, 157, 12403, 657359, 26294360, 846678392 },
    { 1, 158, 12561, 669920, 26964280, 873642672 },
    { 1, 159, 12720, 682640, 27646920, 901289592 },
    { 1, 160, 12880, 695520, 28342440, 929632032 },
    { 1, 161, 13041, 708561, 29051001, 958683033 },
    { 1, 162, 13203, 721764, 29772765, 988455798 },
    { 1, 163, 13366, 735130, 30507895, 1018963693 },
    { 1, 164, 13530, 748660, 31256555, 1050220248 },
    { 1, 165, 13695, 762355, 32018910, 1082239158 },
    { 1, 166, 13861, 776216, 32795126, 1115034284 },
    { 1, 167, 14028, 790244, 33585370, 1148619654 },
    { 1, 168, 14196, 804440, 34389810, 1183009464 },
    { 1, 169, 14365, 818805, 35208615, 1218218079 },
    { 1, 170, 14535, 833340, 36041955, 1254260034 },
    { 1, 171, 14706, 848046, 36890001, 1291150035 },
    { 1, 172, 14878, 862924, 37752925, 1328902960 },
    { 1, 173, 15051, 877975, 38630900, 1367533860 },
    { 1, 174, 15225, 893200, 39524100, 1407057960 },
    { 1, 175, 15400, 908600, 40432700, 1447490660 }};
uint64_t multichoose(uint64_t n, uint64_t k) {
    return MULTICHOOSE[n][k];
}
const char* actor_name(uint64_t group) {
    switch (group) {
    case 0: return "";
    case 1: return "Item_Fruit_D";
    case 2: return "Item_Fruit_G";
    case 3: return "Item_Fruit_A";
    case 4: return "Item_Fruit_B";
    case 5: return "Item_Fruit_F";
    case 6: return "Item_Fruit_I";
    case 7: return "Item_Fruit_C";
    case 8: return "Item_Fruit_E";
    case 9: return "Item_Fruit_H";
    case 10: return "Item_Mushroom_N";
    case 11: return "Item_Mushroom_F";
    case 12: return "Item_Mushroom_O";
    case 13: return "Item_Mushroom_E";
    case 14: return "Item_Mushroom_A";
    case 15: return "Item_Mushroom_B";
    case 16: return "Item_Mushroom_C";
    case 17: return "Item_Mushroom_H";
    case 18: return "Item_MushroomGet_D";
    case 19: return "Item_Mushroom_L";
    case 20: return "Item_Mushroom_M";
    case 21: return "Item_Mushroom_J";
    case 22: return "Item_PlantGet_C";
    case 23: return "Item_PlantGet_B";
    case 24: return "Item_PlantGet_Q";
    case 25: return "Item_PlantGet_A";
    case 26: return "Item_PlantGet_M";
    case 27: return "Item_Fruit_J";
    case 28: return "Item_PlantGet_E";
    case 29: return "Item_PlantGet_F";
    case 30: return "Item_PlantGet_L";
    case 31: return "Item_PlantGet_O";
    case 32: return "Item_PlantGet_G";
    case 33: return "Item_PlantGet_H";
    case 34: return "Item_PlantGet_I";
    case 35: return "Item_PlantGet_J";
    case 36: return "Item_Meat_11";
    case 37: return "Item_Meat_12";
    case 38: return "Item_Meat_02";
    case 39: return "Item_Meat_07";
    case 40: return "Item_Meat_01";
    case 41: return "Item_Meat_06";
    case 42: return "BeeHome";
    case 43: return "Item_Material_03";
    case 44: return "Item_Material_04";
    case 45: return "Item_Material_07";
    case 46: return "Item_Material_05";
    case 47: return "Item_Fruit_K";
    case 48: return "Item_Fruit_L";
    case 49: return "Item_Material_01";
    case 50: return "Item_Material_06";
    case 51: return "Item_Material_02";
    case 52: return "Item_Ore_H";
    case 53: return "Item_Ore_J";
    case 54: return "Item_Enemy_38";
    case 55: return "Item_Enemy_39";
    case 56: return "Item_Enemy_47";
    case 57: return "Item_Enemy_48";
    case 58: return "Item_FishGet_I";
    case 59: return "Item_FishGet_K";
    case 60: return "Item_FishGet_B";
    case 61: return "Item_FishGet_A";
    case 62: return "Item_FishGet_L";
    case 63: return "Item_FishGet_C";
    case 64: return "Item_FishGet_J";
    case 65: return "Item_FishGet_D";
    case 66: return "Item_FishGet_X";
    case 67: return "Item_FishGet_E";
    case 68: return "Item_FishGet_H";
    case 69: return "Item_FishGet_Z";
    case 70: return "Item_FishGet_F";
    case 71: return "Item_FishGet_G";
    case 72: return "Item_FishGet_M";
    case 73: return "Item_InsectGet_K";
    case 74: return "Item_InsectGet_O";
    case 75: return "Item_InsectGet_Z";
    case 76: return "Animal_Insect_F";
    case 77: return "Animal_Insect_N";
    case 78: return "Animal_Insect_Q";
    case 79: return "Animal_Insect_R";
    case 80: return "Animal_Insect_AB";
    case 81: return "Animal_Insect_C";
    case 82: return "Animal_Insect_T";
    case 83: return "Animal_Insect_I";
    case 84: return "Animal_Insect_H";
    case 85: return "Animal_Insect_G";
    case 86: return "Animal_Insect_P";
    case 87: return "Animal_Insect_AA";
    case 88: return "Animal_Insect_E";
    case 89: return "Animal_Insect_A";
    case 90: return "Animal_Insect_B";
    case 91: return "Animal_Insect_S";
    case 92: return "Animal_Insect_M";
    case 93: return "Animal_Insect_X";
    case 94: return "Item_Ore_I";
    case 95: return "Item_Ore_F";
    case 96: return "Item_Ore_E";
    case 97: return "Item_Ore_G";
    case 98: return "Item_Ore_D";
    case 99: return "Item_Ore_B";
    case 100: return "Item_Ore_C";
    case 101: return "Item_Ore_A";
    case 102: return "Item_Enemy_00";
    case 103: return "Item_Enemy_01";
    case 104: return "Item_Enemy_02";
    case 105: return "Item_Enemy_06";
    case 106: return "Item_Enemy_07";
    case 107: return "Item_Enemy_08";
    case 108: return "Item_Enemy_03";
    case 109: return "Item_Enemy_04";
    case 110: return "Item_Enemy_05";
    case 111: return "Item_Enemy_42";
    case 112: return "Item_Enemy_12";
    case 113: return "Item_Enemy_13";
    case 114: return "Item_Enemy_14";
    case 115: return "Item_Enemy_17";
    case 116: return "Item_Enemy_18";
    case 117: return "Item_Enemy_46";
    case 118: return "Item_Enemy_21";
    case 119: return "Item_Enemy_24";
    case 120: return "Item_Enemy_25";
    case 121: return "Item_Enemy_32";
    case 122: return "Item_Enemy_33";
    case 123: return "Item_Enemy_34";
    case 124: return "Item_Enemy_27";
    case 125: return "Item_Enemy_28";
    case 126: return "Item_Enemy_26";
    case 127: return "Item_Enemy_29";
    case 128: return "Item_Enemy_30";
    case 129: return "Item_Enemy_31";
    case 130: return "Obj_FireWoodBundle";
    case 131: return "Item_Roast_03";
    case 132: return "Item_Roast_10";
    case 133: return "Item_Roast_07";
    case 134: return "Item_Roast_48";
    case 135: return "Item_Roast_09";
    case 136: return "Item_Roast_12";
    case 137: return "Item_Roast_13";
    case 138: return "Item_Roast_16";
    case 139: return "Item_Roast_06";
    case 140: return "Item_Roast_04";
    case 141: return "Item_Roast_53";
    case 142: return "Item_Roast_05";
    case 143: return "Item_Roast_49";
    case 144: return "Item_Roast_31";
    case 145: return "Item_Roast_39";
    case 146: return "Item_Roast_18";
    case 147: return "Item_Roast_19";
    case 148: return "Item_Roast_24";
    case 149: return "Item_Roast_50";
    case 150: return "Item_Roast_15";
    case 151: return "Item_Roast_27";
    case 152: return "Item_Roast_51";
    case 153: return "Item_Roast_01";
    case 154: return "Item_Roast_40";
    case 155: return "Item_Roast_45";
    case 156: return "Item_Roast_41";
    case 157: return "Item_RoastFish_01";
    case 158: return "Item_RoastFish_02";
    case 159: return "Item_RoastFish_04";
    case 160: return "Item_RoastFish_03";
    case 161: return "Item_RoastFish_07";
    case 162: return "Item_RoastFish_15";
    case 163: return "Item_Chilled_01";
    case 164: return "Item_Chilled_02";
    case 165: return "Item_Chilled_03";
    case 166: return "Item_ChilledFish_01";
    case 167: return "Item_ChilledFish_02";
    case 168: return "Item_ChilledFish_03";
    case 169: return "Item_ChilledFish_04";
    case 170: return "Item_ChilledFish_09";
    case 171: return "Obj_DRStone_Get";
    case 172: return "dyecolor_00";
    case 173: return "Obj_Photo_Animal";
    case 174: return "Obj_Photo_BossEnemy";
    default: return "";
    }
}
}
