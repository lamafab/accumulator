#[allow(dead_code)]
pub const SMALL_PRIMES: [u64; 50] = [
  2, 3, 5, 7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47, 53, 59, 61, 67, 71, 73, 79, 83, 89, 97,
  101, 103, 107, 109, 113, 127, 131, 137, 139, 149, 151, 157, 163, 167, 173, 179, 181, 191, 193,
  197, 199, 211, 223, 227, 229,
];

#[allow(dead_code, clippy::unreadable_literal)]
pub const MED_PRIMES: [u64; 456] = [
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
pub const EXTRA_STRONG_LUCAS_PSEUDOPRIMES: [u64; 4] = [989, 3239, 5777, 10877];
