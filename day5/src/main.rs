use indicatif::{ProgressBar, ProgressStyle};
use once_cell::sync::OnceCell;
use rayon::prelude::*;
use std::collections::HashSet;
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

fn main() {
    let input = std::fs::read_to_string("data/input.txt").unwrap();
    println!(
        "Part 1: {}",
        lowest_location_number::<RangeParser, Converter>(&input)
    );
    println!(
        "Part 2: {}",
        lowest_location_number::<RangeParser, Converter>(&input)
    );
}

// #[test]
// fn test_lowest_location_number() {
//     let input = std::fs::read_to_string("data/test.txt").unwrap();
//     assert_eq!(
//         35,
//         lowest_location_number::<SimpleParser, Converter>(&input)
//     );
// }
//
// #[test]
// fn test_lowest_location_number_range() {
//     let input = std::fs::read_to_string("data/test.txt").unwrap();
//     assert_eq!(46, lowest_location_number::<RangeParser, Converter>(&input));
// }

trait SeedParser {
    fn parse_seeds(seed: &str) -> Seeds;
}

fn lowest_location_number<S: SeedParser, C: Applicable + Sync + Debug>(input: &str) -> i64 {
    let (seed_input, converter_input) = input.split_once('\n').unwrap();
    let seeds = S::parse_seeds(seed_input);
    let converter = C::parse(converter_input);
    // dbg!(&converter);
    // println!("Converter len : {}", converter.len());

    // Progress tracking
    let progress = Arc::new(AtomicUsize::new(0));
    let total = seeds.len();
    let pb = ProgressBar::new(total as u64);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
        .unwrap().progress_chars("#>-"));

    let result = seeds
        .into_par_iter()
        .map_with(progress.clone(), |progress, seed| {
            let result = converter.apply(seed);
            progress.fetch_add(1, Ordering::SeqCst);
            pb.inc(1); // Update progress bar
            result
        })
        .min()
        .unwrap();

    let result = converter.apply(seeds);

    pb.finish_with_message("Done");

    result
}

fn lowest_location_number_bck<S: SeedParser, C: Applicable + Sync + Debug>(input: &str) -> i64 {
    let (seed_input, converter_input) = input.split_once('\n').unwrap();
    let seeds = S::parse_seeds(seed_input);
    let converter = C::parse(converter_input);
    // dbg!(&converter);
    // println!("Converter len : {}", converter.len());

    // Progress tracking
    let progress = Arc::new(AtomicUsize::new(0));
    let total = seeds.len();
    let pb = ProgressBar::new(total as u64);
    // pb.set_style(ProgressStyle::default_bar()
    //     .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos:>7}/{len:7} ({eta})")
    //     .unwrap().progress_chars("#>-"));

    // let result = seeds
    //     .into_par_iter()
    //     .map_with(progress.clone(), |progress, seed| {
    //         let result = converter.apply(seed);
    //         progress.fetch_add(1, Ordering::SeqCst);
    //         pb.inc(1); // Update progress bar
    //         result
    //     })
    //     .min()
    //     .unwrap();

    let result = converter.apply(seeds);

    // pb.finish_with_message("Done");

    result
}

static SEED_REGEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"\d+").unwrap());
struct RangeParser;
struct SimpleParser;

// impl SeedParser for SimpleParser {
//     fn parse_seeds(seed: &str) -> Seeds {
//         SEED_REGEX
//             .find_iter(seed)
//             .filter_map(|number| number.as_str().parse::<i64>().ok())
//             .collect()
//     }
// }

impl SeedParser for RangeParser {
    fn parse_seeds(seed: &str) -> Seeds {
        let mut seeds = Vec::new();
        let mut fixed = seed.trim_start_matches("seeds: ").split_whitespace();
        while let (Some(range_start), Some(len)) = (fixed.next(), fixed.next()) {
            if let (Ok(range_start), Ok(len)) = (range_start.parse::<i64>(), len.parse::<i64>()) {
            //     seeds.extend((range_start..range_start + len).into_iter());
            //     println!("Extended to: {}", seeds.len());
                seeds.push(range_start..range_start+len)
            }
        }
        println!("Reduced seeds to {}", seeds.len());
        seeds
    }
}


type Transformation = i64;
type Converter = Vec<Vec<(std::ops::Range<i64>, Transformation)>>;
static CONVERT_REGEX: once_cell::sync::Lazy<regex::Regex> =
    once_cell::sync::Lazy::new(|| regex::Regex::new(r"(?<map>((\d+) (\d+) (\d+)\n*)+)").unwrap());

type Seeds = Vec<std::ops::Range<i64>>;

trait Applicable {
    fn apply(&self, seeds: Seeds) -> i64;
    fn parse(input: &str) -> Self;
}

impl Applicable for Converter {
    fn apply(&self, mut seeds: Seeds) -> i64 {
        // 'maps: for map in self {
        //     for (range, transformation) in map {
        //         if range.contains(&seed) {
        //             seed += transformation;
        //             continue 'maps;
        //         }
        //     }
        // }
        // seed
        for map in self {

        }

        todo!()
    }
    fn parse(input: &str) -> Converter {
        let mut converter = Converter::new();
        let maps: Vec<std::str::Lines> = CONVERT_REGEX
            .captures_iter(input)
            .filter_map(|mat| mat.name("map"))
            .map(|map| map.as_str().trim_end().lines())
            .collect();
        for map in maps {
            let mut finished_map: Vec<(std::ops::Range<i64>, Transformation)> = vec![];
            for line in map {
                let mut numbers = line
                    .split_whitespace()
                    .filter_map(|number| number.parse::<i64>().ok());
                let dst = numbers.next().unwrap();
                let src = numbers.next().unwrap();
                let len = numbers.next().unwrap();
                let range = src..src + len;
                let transformation = dst - src;
                finished_map.push((range, transformation));
            }
            converter.push(finished_map);
        }
        converter
    }
}
//
// impl Applicable for BetterConverter {
//     fn apply(&self, seed: i64) -> i64 {
//         for map in self {
//             if map.range.contains(&seed) {
//                 return seed + map.transformation;
//             }
//         }
//         return seed;
//     }
//
//     fn parse(_: &str) -> BetterConverter {
//         let maps: Vec<Vec<Map<i64>>> = MAPS
//             .clone()
//             .into_iter()
//             .map(|map| {
//                 let mut unsorted: Vec<Map<i64>> = map
//                     .into_iter()
//                     .map(|[dst, src, len]| Map {
//                         range: src..src + len,
//                         transformation: dst - src,
//                     })
//                     .collect();
//                 unsorted.sort_by(|this, other| this.range.start.cmp(&other.range.start));
//                 unsorted
//             })
//             .collect();
//
//         let ret = maps
//             .into_iter()
//             .reduce(|mut broken_apart, mut maps| {
//                 dbg!(&broken_apart);
//                 let mut i = 0;
//                 loop {
//                     let mut leftover: Option<Map<i64>> = None;
//                     let Some(mut new_map) = maps.pop() else {
//                         break;
//                     };
//
//                     while leftover.is_some() || i < broken_apart.len() {
//                         let Some(mut map) = (if leftover.is_some() {
//                             let ret = leftover;
//                             leftover = None;
//                             ret
//                         } else {
//                             broken_apart.get_mut(i).cloned()
//                         }) else {
//                             break;
//                         };
//                         dbg!(&map);
//                         dbg!(&broken_apart);
//                         // ---------------[..new_map..]------
//                         // ---[....map....]-------------------
//                         if &new_map.range.start >= &map.range.end {
//                             i += 1;
//                             continue;
//                         }
//
//                         // ---------------[....map....]------
//                         // ---[..new_map..]-------------------
//                         if &new_map.range.end <= &map.range.start {
//                             broken_apart.insert(i, new_map);
//                             i+=1;
//                             break;
//                         }
//
//                         // -----------[....map....]---------
//                         // ---[..new_map..]-----------------
//                         if &new_map.range.start <= &map.range.start {
//                             broken_apart.insert(
//                                 i,
//                                 Map {
//                                     range: map.range.start..new_map.range.end,
//                                     transformation: new_map.transformation + map.transformation,
//                                 },
//                             );
//                             broken_apart.insert(
//                                 i,
//                                 Map {
//                                     range: new_map.range.start..map.range.start,
//                                     transformation: new_map.transformation,
//                                 },
//                             );
//                             map.range.start = new_map.range.end;
//                             i+=2;
//                             leftover = Some(Map {
//                                 range: new_map.range.end..map.range.end,
//                                 transformation: map.transformation,
//                             });
//                         }
//
//                         // -----------[..new_map..]---------
//                         // ---[....map....]-----------------
//                         if &new_map.range.start >= &map.range.start {
//                             i+=2;
//                             broken_apart.insert(
//                                 i,
//                                 Map {
//                                     range: new_map.range.start..map.range.end,
//                                     transformation: new_map.transformation + map.transformation,
//                                 },
//                             );
//                             broken_apart.insert(
//                                 i,
//                                 Map {
//                                     range: map.range.start..new_map.range.start,
//                                     transformation: map.transformation,
//                                 },
//                             );
//                             map.range.end = new_map.range.start;
//                             leftover = Some(Map {
//                                 range: map.range.end..new_map.range.end,
//                                 transformation: new_map.transformation,
//                             });
//                         }
//                     }
//                 }
//                 broken_apart.sort_by(|this, other| this.range.start.cmp(&other.range.start));
//                 broken_apart
//             })
//             .unwrap();
//         ret
//     }
// }
//
type BetterConverter = Vec<Map<i64>>;

#[derive(Debug, Clone, Eq, PartialEq)]
struct Map<T: Eq + PartialEq> {
    range: std::ops::Range<T>,
    transformation: Transformation,
}

// impl<T> PartialEq for Map<T>
// where
//     T: PartialEq,
// {
//     fn eq(&self, other: &Self) -> bool {
//         self.range.eq(&other.range)
//     }
// }
// impl<T> PartialOrd for Map<T>
// where
//     T: PartialOrd,
// {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.range.partial_cmp(&other.range)
//     }
// }
static MAPS: once_cell::sync::Lazy<Vec<Vec<[i64; 3]>>> = once_cell::sync::Lazy::new(|| {vec![
    vec![[50,98,2],
         [52,50,48],
    ],vec![
        [0,15,37],
        [37,52,2],
        [39,0,15],
    ],vec![
        [49,53,8],
        [0,11,42],
        [42,0,7],
        [57,7,4],
    ],vec![
        [88,18,7],
        [18,25,70],
    ],vec![
        [45,77,23],
        [81,45,19],
        [68,64,13],
    ],vec![
        [0,69,1],
        [1,0,69],
    ],vec![
        [60,56,37],
        [56,93,4]]]
});

static tmp_MAPS: once_cell::sync::Lazy<Vec<Vec<[i64; 3]>>> = once_cell::sync::Lazy::new(|| {
    vec![
        vec![
            [3680121696, 1920754815, 614845600],
            [1920754815, 3846369604, 448597692],
            [193356576, 570761634, 505124585],
            [2369352507, 2535600415, 31531965],
            [2400884472, 2567132380, 1279237224],
            [0, 459278395, 111483239],
            [698481161, 97868205, 361410190],
            [1059891351, 0, 15994868],
            [111483239, 15994868, 81873337],
        ],
        vec![
            [1633669237, 1273301814, 72865265],
            [2398515176, 2671190790, 99210785],
            [2397916384, 3018946373, 598792],
            [4034325916, 3061716397, 20017393],
            [3298612516, 3793795301, 14249501],
            [4030007411, 3051046904, 2833129],
            [1906984482, 224872691, 14620134],
            [864506893, 1590633724, 149044542],
            [1029530319, 442871336, 36727018],
            [1921604616, 770934113, 68546178],
            [3560536321, 3114405501, 28822192],
            [1019762634, 1263534129, 9767685],
            [3852235341, 3579014714, 60339892],
            [2385228698, 1577946038, 12687686],
            [2234322470, 239492825, 150906228],
            [0, 170310676, 54562015],
            [3208946111, 3808044802, 89666405],
            [1209615399, 839480291, 424053838],
            [4032840540, 4041982568, 1485376],
            [2497725961, 2174737461, 293042810],
            [2002543511, 1346167079, 231778959],
            [3312862017, 3475611771, 103402943],
            [318739997, 1739678266, 354749094],
            [1013551435, 3012735174, 6211199],
            [4014277153, 4160859076, 15730258],
            [3589358513, 3143227693, 230682158],
            [1990150794, 2467780271, 12392717],
            [3051046904, 3081733790, 32671711],
            [3820040671, 3761600631, 32194670],
            [148429321, 0, 170310676],
            [673489091, 2480172988, 191017802],
            [1066257337, 627576051, 143358062],
            [2790768771, 2770401575, 242333599],
            [3091554979, 4043467944, 117391132],
            [3416264960, 3897711207, 144271361],
            [3912575233, 3373909851, 101701920],
            [4072291714, 3639354606, 104297620],
            [3083718615, 3053880033, 7836364],
            [54562015, 3019545165, 13557205],
            [1759006785, 479598354, 147977697],
            [68119220, 2094427360, 80310101],
            [1706534502, 390399053, 52472283],
            [4054343309, 3743652226, 17948405],
        ],
        vec![
            [0, 1095885172, 129797665],
            [2661548513, 1044284418, 17872363],
            [3282164642, 3678907615, 214830258],
            [1440687421, 2218635146, 325889720],
            [3496994900, 4208791298, 25912548],
            [3253828209, 4136945159, 5561683],
            [1797056017, 864689597, 109403664],
            [3259389892, 4186016548, 22774750],
            [2578517508, 1225682837, 83031005],
            [3193832718, 3618912124, 59995491],
            [3695649169, 3214450646, 211194594],
            [820325042, 974093261, 70191157],
            [1284591017, 1074888739, 20996433],
            [2929761569, 3893737873, 85668135],
            [1305587450, 2605461705, 73959171],
            [2168339930, 1062156781, 12731958],
            [2465234843, 2135490203, 52666067],
            [3522907448, 4108091872, 882860],
            [3523790308, 4255675252, 39292044],
            [2517900910, 1308713842, 60616598],
            [3563082352, 3176039879, 38410767],
            [3015429704, 3979406008, 128685864],
            [1913427402, 2131043562, 4446641],
            [2235159285, 1419841841, 190190495],
            [3673886186, 4108974732, 21762983],
            [891504291, 1610032336, 393086726],
            [3927815169, 3425645240, 193266884],
            [4121082053, 3002154636, 173885243],
            [2084864581, 2004107154, 83475349],
            [1917874043, 557612753, 69524983],
            [890516199, 2003119062, 988092],
            [766355924, 2551492587, 53969118],
            [1379546621, 0, 61140800],
            [3187625274, 4130737715, 6207444],
            [528804063, 627137736, 237551861],
            [1987399026, 230411125, 97465555],
            [1906459681, 2544524866, 6967721],
            [2184647884, 1369330440, 50511401],
            [2425349780, 2091158499, 39885063],
            [2181071888, 2087582503, 3575996],
            [3144115568, 4142506842, 43509706],
            [1766577141, 2188156270, 30478876],
            [359533738, 61140800, 169270325],
            [129797665, 327876680, 229736073],
            [3906843763, 4234703846, 20971406],
            [3601493119, 2929761569, 72393067],
        ],
        vec![
            [2375927917, 1595026882, 126334140],
            [1307603095, 818620477, 43777869],
            [2050676589, 1855896418, 112224406],
            [3618302244, 2909504698, 119958941],
            [3078570200, 3088215627, 6211083],
            [3084781283, 3094426710, 141266337],
            [524666822, 53020621, 149058240],
            [673725062, 862398346, 147671362],
            [2364320682, 2577001713, 11607235],
            [1941578413, 1584221500, 10805382],
            [2162900995, 2536766467, 40235246],
            [162015400, 237365123, 4480592],
            [821396424, 241845715, 141336168],
            [166495992, 1138498800, 212882164],
            [4277433486, 4220367555, 17533810],
            [3226047620, 2229635217, 307131250],
            [2909428734, 1968120824, 34606070],
            [1885573816, 3954749082, 56004597],
            [2711875933, 2868267590, 41237108],
            [0, 726306378, 92314099],
            [2944034804, 1721361022, 134535396],
            [1584221500, 3392008740, 301352316],
            [962732592, 34415039, 18605582],
            [2235705153, 2101019688, 128615529],
            [4252936467, 4237901365, 24497019],
            [92314099, 202078861, 35286262],
            [981338174, 400041457, 326264921],
            [379378156, 1010069708, 128429092],
            [3591930858, 3693361056, 26371386],
            [3785017329, 3719732442, 235016640],
            [2502262057, 4010753679, 209613876],
            [3533178870, 3029463639, 58751988],
            [3738261185, 2821511446, 46756144],
            [4020033969, 2806534555, 14976891],
            [1952383795, 2002726894, 98292794],
            [127600361, 0, 34415039],
            [2753113041, 3235693047, 156315693],
            [507807248, 383181883, 16859574],
            [2203136241, 4262398384, 32568912],
            [4035010860, 2588608948, 217925607],
        ],
        vec![
            [2137189745, 1335050925, 100355790],
            [639139367, 2440321747, 987829],
            [1663612748, 1778059435, 153830272],
            [1122754252, 1950103191, 82536600],
            [1929621334, 1199531530, 135519395],
            [1286703174, 2032639791, 207137687],
            [245313533, 981575774, 217955756],
            [2597564380, 2824691125, 293777778],
            [895004176, 331442633, 25226735],
            [1493840861, 236388681, 616173],
            [1494457034, 764560381, 107637728],
            [1817443020, 1435406715, 112178314],
            [1205290852, 356669368, 33552643],
            [474799702, 0, 164339665],
            [2341054397, 2260378974, 100255179],
            [1043066658, 2360634153, 79687594],
            [125852143, 390222011, 119461390],
            [3924383937, 3130691909, 13614218],
            [2467721984, 3747288823, 76649669],
            [2065140729, 164339665, 72049016],
            [920230911, 1673437172, 104622263],
            [640127196, 509683401, 254876980],
            [1024853174, 1931889707, 18213484],
            [2331983314, 2251307891, 9071083],
            [2237545535, 237004854, 94437779],
            [3912160931, 3118468903, 12223006],
            [1238843495, 933716095, 47859679],
            [3326002417, 3517222025, 230066798],
            [3556069215, 3144306127, 241363224],
            [3797432439, 4180238804, 114728492],
            [1602094762, 872198109, 61517986],
            [2544371653, 4127046077, 53192727],
            [2891342158, 3823938492, 303107585],
            [463269289, 2239777478, 11530413],
            [3194449743, 3385669351, 131552674],
            [3937998155, 2467721984, 356969141],
            [0, 1547585029, 125852143],
        ],
        vec![
            [2687600833, 2313887435, 187105587],
            [3281196981, 2291603041, 22284394],
            [1771250828, 1899269239, 314167725],
            [784031720, 478456148, 306959384],
            [2605226464, 1771250828, 58348072],
            [2085418553, 3793564740, 111907603],
            [1090991104, 785415532, 575136195],
            [3437652344, 1829598900, 69670339],
            [2874706420, 2500993022, 389039942],
            [3303481375, 3905472343, 134170969],
            [305575572, 0, 478456148],
            [3263746362, 2890032964, 17450619],
            [2527060387, 2213436964, 78166077],
            [2428623843, 3695128196, 98436544],
            [2663574536, 4270940999, 24026297],
            [3507322683, 2907483583, 787644613],
            [2197326156, 4039643312, 231297687],
            [0, 1360551727, 305575572],
        ],
        vec![
            [1919184105, 1156349110, 51114849],
            [4031284281, 3411510751, 25609498],
            [0, 171183359, 79004094],
            [1253227229, 2072782209, 122019778],
            [4056893779, 3437120249, 136289693],
            [3402931364, 4156827458, 101778985],
            [84557792, 1207463959, 134801591],
            [635909965, 1371746366, 266495395],
            [4029464617, 4127764171, 1819664],
            [4193183472, 2857352625, 101783824],
            [1375247007, 2200355685, 41445634],
            [1996492203, 0, 171183359],
            [3601595563, 3699895117, 427869054],
            [2218993186, 1133540977, 22808133],
            [3217192942, 2959136449, 140385316],
            [2987922009, 4258606443, 9236491],
            [2628749093, 2543337773, 86365212],
            [2167675562, 369831582, 51317624],
            [3504710349, 2446452559, 96885214],
            [902405360, 421149206, 108869392],
            [3357578258, 2811999519, 45353106],
            [1196458443, 1638241761, 56768786],
            [1178674352, 693035436, 17784091],
            [1970298954, 530018598, 26193249],
            [2868723842, 3099521765, 91954544],
            [1522874936, 895259169, 18612073],
            [2841599480, 4267842934, 27124362],
            [1880563756, 1695010547, 38620349],
            [1011274752, 913871242, 137918784],
            [219359383, 556211847, 136823589],
            [1785350971, 250187453, 95212785],
            [2960678386, 4129583835, 27243623],
            [540622614, 1977494858, 95287351],
            [356182972, 710819527, 184439642],
            [1498443592, 345400238, 24431344],
            [1416692641, 1051790026, 81750951],
            [1541487009, 1733630896, 243863962],
            [79004094, 2194801987, 5553698],
            [1149193536, 1342265550, 29480816],
            [2997158500, 3191476309, 220034442],
            [2715114305, 3573409942, 126485175],
            [2446452559, 2629702985, 182296534],
        ],
    ]
});
