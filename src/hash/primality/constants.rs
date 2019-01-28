pub const SMALL_PRIMES: [u32; 50] = [
  2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
  101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
  197, 199, 211, 223, 227, 229,
];

#[allow(dead_code, clippy::unreadable_literal)]
pub const MED_PRIMES: [u32; 456] = [
  106957, 106961, 106963, 106979, 106993, 107021, 107033, 107053, 107057, 107069, 107071, 107077,
  107089, 107099, 107101, 107119, 107123, 107137, 107171, 107183, 107197, 107201, 107209, 107227,
  107243, 107251, 107269, 107273, 107279, 107309, 107323, 107339, 107347, 107351, 107357, 107377,
  107441, 107449, 107453, 107467, 107473, 107507, 107509, 107563, 107581, 107599, 107603, 107609,
  107621, 107641, 107647, 107671, 107687, 107693, 107699, 107713, 107717, 107719, 107741, 107747,
  107761, 107773, 107777, 107791, 107827, 107837, 107839, 107843, 107857, 107867, 107873, 107881,
  107897, 107903, 107923, 107927, 107941, 107951, 107971, 107981, 107999, 108007, 108011, 108013,
  108023, 108037, 108041, 108061, 108079, 108089, 108107, 108109, 108127, 108131, 108139, 108161,
  108179, 108187, 108191, 108193, 108203, 108211, 108217, 108223, 108233, 108247, 108263, 108271,
  108287, 108289, 108293, 108301, 108343, 108347, 108359, 108377, 108379, 108401, 108413, 108421,
  108439, 108457, 108461, 108463, 108497, 108499, 108503, 108517, 108529, 108533, 108541, 108553,
  108557, 108571, 108587, 108631, 108637, 108643, 108649, 108677, 108707, 108709, 108727, 108739,
  108751, 108761, 108769, 108791, 108793, 108799, 108803, 108821, 108827, 108863, 108869, 108877,
  108881, 108883, 108887, 108893, 108907, 108917, 108923, 108929, 108943, 108947, 108949, 108959,
  108961, 108967, 108971, 108991, 109001, 109013, 109037, 109049, 109063, 109073, 109097, 109103,
  109111, 109121, 109133, 109139, 109141, 109147, 109159, 109169, 109171, 109199, 109201, 109211,
  109229, 109253, 109267, 109279, 109297, 109303, 109313, 109321, 109331, 109357, 109363, 109367,
  109379, 109387, 109391, 109397, 109423, 109433, 109441, 109451, 109453, 109469, 109471, 109481,
  109507, 109517, 109519, 109537, 109541, 109547, 109567, 109579, 109583, 109589, 109597, 109609,
  109619, 109621, 109639, 109661, 109663, 109673, 109717, 109721, 109741, 109751, 109789, 109793,
  109807, 109819, 109829, 109831, 109841, 109843, 109847, 109849, 109859, 109873, 109883, 109891,
  109897, 109903, 109913, 109919, 109937, 109943, 109961, 109987, 110017, 110023, 110039, 110051,
  110059, 110063, 110069, 110083, 110119, 110129, 110161, 110183, 110221, 110233, 110237, 110251,
  110261, 110269, 110273, 110281, 110291, 110311, 110321, 110323, 110339, 110359, 110419, 110431,
  110437, 110441, 110459, 110477, 110479, 110491, 110501, 110503, 110527, 110533, 110543, 110557,
  110563, 110567, 110569, 110573, 110581, 110587, 110597, 110603, 110609, 110623, 110629, 110641,
  110647, 110651, 110681, 110711, 110729, 110731, 110749, 110753, 110771, 110777, 110807, 110813,
  110819, 110821, 110849, 110863, 110879, 110881, 110899, 110909, 110917, 110921, 110923, 110927,
  110933, 110939, 110947, 110951, 110969, 110977, 110989, 111029, 111031, 111043, 111049, 111053,
  111091, 111103, 111109, 111119, 111121, 111127, 111143, 111149, 111187, 111191, 111211, 111217,
  111227, 111229, 111253, 111263, 111269, 111271, 111301, 111317, 111323, 111337, 111341, 111347,
  111373, 111409, 111427, 111431, 111439, 111443, 111467, 111487, 111491, 111493, 111497, 111509,
  111521, 111533, 111539, 111577, 111581, 111593, 111599, 111611, 111623, 111637, 111641, 111653,
  111659, 111667, 111697, 111721, 111731, 111733, 111751, 111767, 111773, 111779, 111781, 111791,
  111799, 111821, 111827, 111829, 111833, 111847, 111857, 111863, 111869, 111871, 111893, 111913,
  111919, 111949, 111953, 111959, 111973, 111977, 111997, 112019, 112031, 112061, 112067, 112069,
  112087, 112097, 112103, 112111, 112121, 112129, 112139, 112153, 112163, 112181, 112199, 112207,
  112213, 112223, 112237, 112241, 112247, 112249, 112253, 112261, 112279, 112289, 112291, 112297,
];

#[allow(dead_code)]
pub const LARGE_PRIMES: [u64; 4] = [
  553_525_575_239_331_913,
  12_702_637_924_034_044_211,
  378_373_571_372_703_133,
  8_640_171_141_336_142_787,
];

#[allow(dead_code)]
pub const STRONG_BASE_2_PSEUDOPRIMES: [u32; 10] = [
  2047, 3277, 4033, 4681, 8321, 15841, 29341, 42799, 49141, 52633,
];

#[allow(dead_code)]
pub const STRONG_LUCAS_PSEUDOPRIMES: [u32; 10] = [
  5459, 5777, 10877, 16109, 18971, 22499, 24569, 25199, 40309, 58519,
];

#[allow(dead_code)]
pub const EXTRA_STRONG_LUCAS_PSEUDOPRIMES: [u32; 10] = [
  989, 3239, 5777, 10877, 27971, 29681, 30739, 31631, 39059, 72389,
];

// REVIEW @Pablo: help me pick a better name for this
pub const DS: [i32; 500] = [
  5, -7, 9, -11, 13, -15, 17, -19, 21, -23, 25, -27, 29, -31, 33, -35, 37, -39, 41, -43, 45, -47,
  49, -51, 53, -55, 57, -59, 61, -63, 65, -67, 69, -71, 73, -75, 77, -79, 81, -83, 85, -87, 89,
  -91, 93, -95, 97, -99, 101, -103, 105, -107, 109, -111, 113, -115, 117, -119, 121, -123, 125,
  -127, 129, -131, 133, -135, 137, -139, 141, -143, 145, -147, 149, -151, 153, -155, 157, -159,
  161, -163, 165, -167, 169, -171, 173, -175, 177, -179, 181, -183, 185, -187, 189, -191, 193,
  -195, 197, -199, 201, -203, 205, -207, 209, -211, 213, -215, 217, -219, 221, -223, 225, -227,
  229, -231, 233, -235, 237, -239, 241, -243, 245, -247, 249, -251, 253, -255, 257, -259, 261,
  -263, 265, -267, 269, -271, 273, -275, 277, -279, 281, -283, 285, -287, 289, -291, 293, -295,
  297, -299, 301, -303, 305, -307, 309, -311, 313, -315, 317, -319, 321, -323, 325, -327, 329,
  -331, 333, -335, 337, -339, 341, -343, 345, -347, 349, -351, 353, -355, 357, -359, 361, -363,
  365, -367, 369, -371, 373, -375, 377, -379, 381, -383, 385, -387, 389, -391, 393, -395, 397,
  -399, 401, -403, 405, -407, 409, -411, 413, -415, 417, -419, 421, -423, 425, -427, 429, -431,
  433, -435, 437, -439, 441, -443, 445, -447, 449, -451, 453, -455, 457, -459, 461, -463, 465,
  -467, 469, -471, 473, -475, 477, -479, 481, -483, 485, -487, 489, -491, 493, -495, 497, -499,
  501, -503, 505, -507, 509, -511, 513, -515, 517, -519, 521, -523, 525, -527, 529, -531, 533,
  -535, 537, -539, 541, -543, 545, -547, 549, -551, 553, -555, 557, -559, 561, -563, 565, -567,
  569, -571, 573, -575, 577, -579, 581, -583, 585, -587, 589, -591, 593, -595, 597, -599, 601,
  -603, 605, -607, 609, -611, 613, -615, 617, -619, 621, -623, 625, -627, 629, -631, 633, -635,
  637, -639, 641, -643, 645, -647, 649, -651, 653, -655, 657, -659, 661, -663, 665, -667, 669,
  -671, 673, -675, 677, -679, 681, -683, 685, -687, 689, -691, 693, -695, 697, -699, 701, -703,
  705, -707, 709, -711, 713, -715, 717, -719, 721, -723, 725, -727, 729, -731, 733, -735, 737,
  -739, 741, -743, 745, -747, 749, -751, 753, -755, 757, -759, 761, -763, 765, -767, 769, -771,
  773, -775, 777, -779, 781, -783, 785, -787, 789, -791, 793, -795, 797, -799, 801, -803, 805,
  -807, 809, -811, 813, -815, 817, -819, 821, -823, 825, -827, 829, -831, 833, -835, 837, -839,
  841, -843, 845, -847, 849, -851, 853, -855, 857, -859, 861, -863, 865, -867, 869, -871, 873,
  -875, 877, -879, 881, -883, 885, -887, 889, -891, 893, -895, 897, -899, 901, -903, 905, -907,
  909, -911, 913, -915, 917, -919, 921, -923, 925, -927, 929, -931, 933, -935, 937, -939, 941,
  -943, 945, -947, 949, -951, 953, -955, 957, -959, 961, -963, 965, -967, 969, -971, 973, -975,
  977, -979, 981, -983, 985, -987, 989, -991, 993, -995, 997, -999, 1001, -1003,
];
