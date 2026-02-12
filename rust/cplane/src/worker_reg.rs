////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////
// Code for managing the worker registry
//
// This looks for jobs to be run in the cp-svr and takes then and runs. It should locally store logs on the jobs,
// but return status updates and user facing logging info to the cp-svr.
//
// On startup it should register itself with the cp-svr, and on shutdown it should unregister itself. It should
// also provide a ping type endpoint to test for availability, and probably some kinds of statistics endpoints
// as well.  While we're at it we should probably add locally stored statistics as well.
////////////////////////////////////////////////////////////////////////////////////////////////////////////////////////

use serde::{Deserialize, Serialize};

const ADJ_LIST: [&str; 121] = [
    // A
    "artsy",
    "avid",
    "antsy",
    "angry",
    "apathetic",
    // B
    "basic",
    "boring",
    "ballsy",
    "brassy",
    "bad",
    "bogus",
    "browsing",
    "bonus",
    // C
    "choosy",
    "costly",
    "cozy",
    "crass",
    "classy",
    // D
    "desperate",
    "dogged",
    "dodgy",
    "daring",
    "dashing",
    "dirty",
    "degenerate",
    // E
    "excited",
    "evocative",
    "egregious",
    "emotional",
    "extreme",
    "elongated",
    "edgy",
    "extra",
    // F
    "flashy",
    "fulsome",
    "fatal",
    "festive",
    "friendly",
    "famished",
    "flailing",
    "french",
    // G
    "green",
    "gracious",
    "golden",
    "giddy",
    "gross",
    "gelatinous",
    "gritty",
    // H
    "heavy",
    "hearty",
    "heated",
    "happy",
    "harsh",
    "heady",
    "horrid",
    // I
    "indigent",
    "indignant",
    "irate",
    "irksome",
    "imperious",
    // J
    "joking",
    "jeering",
    // K
    "kindly",
    // L
    "laughing",
    "lazy",
    "loose",
    // M
    "messy",
    "maudlin",
    "mad",
    "minty",
    // N
    "negative",
    "nasty",
    "nosey",
    "nerdy",
    "nice",
    "neat",
    // O
    "outsized",
    "official",
    "odious",
    "offensive",
    "ostentatius",
    // P
    "preppy",
    "putrid",
    "plastic",
    "poofy",
    "panting",
    // Q
    "quiet",
    "quaint",
    // R
    "rusty",
    "ready",
    "random",
    "revolting",
    "robust",
    // S
    "spicy",
    "salty",
    "sassy",
    "simple",
    "snobby",
    // T
    "timorous",
    "tight",
    "tough",
    "tawdry",
    "tempting",
    "torrid",
    // U
    "unlikely",
    "ugly",
    "unwelcome",
    // V
    "vigorous",
    "virtuous",
    "vicious",
    "vital",
    "vulger",
    "vacuous",
    // W
    "witty",
    "worried",
    "winsome",
    // X
    "xenophobic",
    // Y
    "yelling",
    "yellow",
    "yankee",
    // Z
    "zealous",
    "zippy",
];

const NOUN_LIST: [&str; 130] = [
    // A
    "aardvark",
    "apple",
    "architect",
    "asset",
    "alaskan",
    "aztec",
    "american",
    // B
    "bordello",
    "basket",
    "balloon",
    "baboon",
    "barroom",
    "baseball",
    "basoon",
    "buffalo",
    // C
    "cat",
    "cafe",
    "camel",
    "chicken",
    "cabin",
    "cartoon",
    "chamber",
    "cello",
    "canadian",
    // D
    "dragon",
    "deer",
    "dane",
    "ditch",
    "dance",
    "dungeon",
    // E
    "event",
    "elephant",
    "elevator",
    "element",
    "egret",
    "election",
    // F
    "fever",
    "festival",
    "frog",
    "frankfurter",
    "friend",
    "fish",
    // G
    "giraffe",
    "guest",
    "grifter",
    "garden",
    // H
    "house",
    "horse",
    "hippy",
    "hippo",
    "hand",
    "hearse",
    "hound",
    // I
    "insect",
    "italian",
    "idiot",
    "illness",
    "introvert",
    // J
    "jail",
    "jumbotron",
    "jungle",
    "jaguar",
    "jackel",
    "jackalope",
    // K
    "kangaroo",
    "kettle",
    "knee",
    "knife",
    "kumquat",
    // L
    "lemming",
    "loser",
    "lady",
    "llama",
    "lamb",
    "loft",
    // M
    "mouth",
    "mountain",
    "mouse",
    "mastif",
    "monkey",
    "mule",
    // N
    "night",
    "number",
    "nudist",
    "napkin",
    // O
    "orangutang",
    "organization",
    "optimist",
    "ostrich",
    "outsider",
    // P
    "penguin",
    "prussian",
    "parisian",
    "potato",
    "proof",
    // Q
    "quiche",
    "quarter",
    // R
    "russian",
    "roast",
    "referee",
    "rumble",
    "reflex",
    "robin",
    "range",
    // S
    "salad",
    "sandbox",
    "schnauzer",
    "store",
    "sofa",
    "sample",
    // T
    "teacher",
    "train",
    "tapir",
    "town",
    "tomato",
    // U
    "umbrella",
    "user",
    "umpire",
    // V
    "volvo",
    "victory",
    "villan",
    // W
    "wrist",
    "whale",
    "winter",
    // X
    "xylephone",
    // Y
    "yell",
    // Z
    "zone",
    "zebra",
    "zipper",
    "zeppelin",
];

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
pub struct WorkerRecord {
    pub name: String,
}

impl WorkerRecord {
    pub fn generate_worker_name() -> String {
        let a_max = ADJ_LIST.len();
        let n_max = NOUN_LIST.len();

        let a_idx = rand::random_range(0..a_max);
        let n_idx = rand::random_range(0..n_max);

        let adj = ADJ_LIST[a_idx];
        let noun = NOUN_LIST[n_idx];

        format!("{adj}-{noun}")
    }
}
