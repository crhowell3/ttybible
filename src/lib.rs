use byteorder::{ByteOrder, LittleEndian};
use rand::RngExt;
use std::collections::HashMap;

pub mod cli;

pub const VERSION_AND_GIT_HASH: &str = env!("VERSION_AND_GIT_HASH");

pub static BIBLE_DATA: &[u8] = include_bytes!("../data/bible.dat");

pub struct Bible {
    data: &'static [u8],
    index: HashMap<(u8, u16, u16), VerseIndex>,
}

#[derive(Clone)]
struct VerseIndex {
    offset: u64,
    length: u32,
}

impl Bible {
    pub fn open() -> Self {
        let data = BIBLE_DATA;
        let mut pos = 0;
        if &data[0..4] != b"BIBL" {
            panic!("Invalid bible file");
        }
        pos += 4;

        let _version = LittleEndian::read_u16(&data[pos..]);
        pos += 2;

        let _book_count = LittleEndian::read_u16(&data[pos..]);
        pos += 2;

        let verse_count = LittleEndian::read_u32(&data[pos..]);
        pos += 4;

        let index_offset = LittleEndian::read_u64(&data[pos..]);
        pos += 8;

        let _text_offset = LittleEndian::read_u64(&data[pos..]);

        let mut index = HashMap::new();

        let mut pos = index_offset as usize;

        for _ in 0..verse_count {
            let book = data[pos];
            pos += 1;

            let chapter = LittleEndian::read_u16(&data[pos..]);
            pos += 2;

            let verse = LittleEndian::read_u16(&data[pos..]);
            pos += 2;

            let offset = LittleEndian::read_u64(&data[pos..]);
            pos += 8;

            let length = LittleEndian::read_u32(&data[pos..]);
            pos += 4;

            index.insert((book, chapter, verse), VerseIndex { offset, length });
        }

        Self { data, index }
    }

    pub fn get(&self, book: u8, chapter: u16, verse: u16) -> String {
        let Some(idx) = self.index.get(&(book, chapter, verse)) else {
            return "Verse does not exist".to_string();
        };

        let start = idx.offset as usize;
        let end = start + idx.length as usize;

        let slice = &self.data[start..end];

        std::str::from_utf8(slice).unwrap().to_string()
    }

    pub fn get_range(&self, book: u8, chapter: u16, start: u16, end: u16) -> Vec<String> {
        let mut verses = Vec::new();

        for v in start..=end {
            verses.push(self.get(book, chapter, v));
        }

        verses
    }

    pub fn get_random_verse(&self) {
        let mut rng = rand::rng();

        let verses: Vec<_> = self.index.keys().collect();

        let idx = rng.random_range(0..verses.len());

        let (book, chapter, verse) = verses[idx];

        let text = self.get(*book, *chapter, *verse);

        let book_name = id_to_book(*book);

        println!("{text}\n- {book_name} {chapter}:{verse}");
    }
}

pub fn book_to_id(name: &str) -> u8 {
    match name.to_lowercase().as_str() {
        "genesis" => 1,
        "exodus" => 2,
        "leviticus" => 3,
        "numbers" => 4,
        "deuteronomy" => 5,
        "joshua" => 6,
        "judges" => 7,
        "ruth" => 8,
        "1 samuel" => 9,
        "2 samuel" => 10,
        "1 kings" => 11,
        "2 kings" => 12,
        "1 chronicles" => 13,
        "2 chronicles" => 14,
        "ezra" => 15,
        "nehemiah" => 16,
        "esther" => 17,
        "job" => 18,
        "psalm" | "psalms" => 19,
        "proverbs" => 20,
        "ecclesiastes" => 21,
        "song of songs" | "song of solomon" => 22,
        "isaiah" => 23,
        "jeremiah" => 24,
        "lamentations" => 25,
        "ezekiel" => 26,
        "daniel" => 27,
        "hosea" => 28,
        "joel" => 29,
        "amos" => 30,
        "obadiah" => 31,
        "jonah" => 32,
        "micah" => 33,
        "nahum" => 34,
        "habakkuk" => 35,
        "zephaniah" => 36,
        "haggai" => 37,
        "zechariah" => 38,
        "malachi" => 39,
        "matthew" => 40,
        "mark" => 41,
        "luke" => 42,
        "john" => 43,
        "acts" => 44,
        "romans" => 45,
        "1 corinthians" => 46,
        "2 corinthians" => 47,
        "galatians" => 48,
        "ephesians" => 49,
        "philippians" => 50,
        "colossians" => 51,
        "1 thessalonians" => 52,
        "2 thessalonians" => 53,
        "1 timothy" => 54,
        "2 timothy" => 55,
        "titus" => 56,
        "philemon" => 57,
        "hebrews" => 58,
        "james" => 59,
        "1 peter" => 60,
        "2 peter" => 61,
        "1 john" => 62,
        "2 john" => 63,
        "3 john" => 64,
        "jude" => 65,
        "revelation" => 66,
        _ => panic!("Unknown book {name}"),
    }
}

pub fn id_to_book(id: u8) -> &'static str {
    match id {
        1 => "Genesis",
        2 => "Exodus",
        3 => "Leviticus",
        4 => "Numbers",
        5 => "Deuteronomy",
        6 => "Joshua",
        7 => "Judges",
        8 => "Ruth",
        9 => "1 Samuel",
        10 => "2 Samuel",
        11 => "1 Kings",
        12 => "2 Kings",
        13 => "1 Chronicles",
        14 => "2 Chronicles",
        15 => "Ezra",
        16 => "Nehemiah",
        17 => "Esther",
        18 => "Job",
        19 => "Psalms",
        20 => "Proverbs",
        21 => "Ecclesiastes",
        22 => "Song of Solomon",
        23 => "Isaiah",
        24 => "Jeremiah",
        25 => "Lamentations",
        26 => "Ezekiel",
        27 => "Daniel",
        28 => "Hosea",
        29 => "Joel",
        30 => "Amos",
        31 => "Obadiah",
        32 => "Jonah",
        33 => "Micah",
        34 => "Nahum",
        35 => "Habakkuk",
        36 => "Zephaniah",
        37 => "Haggai",
        38 => "Zechariah",
        39 => "Malachi",
        40 => "Matthew",
        41 => "Mark",
        42 => "Luke",
        43 => "John",
        44 => "Acts",
        45 => "Romans",
        46 => "1 Corinthians",
        47 => "2 Corinthians",
        48 => "Galatians",
        49 => "Ephesians",
        50 => "Philippians",
        51 => "Colossians",
        52 => "1 Thessalonians",
        53 => "2 Thessalonians",
        54 => "1 Timothy",
        55 => "2 Timothy",
        56 => "Titus",
        57 => "Philemon",
        58 => "Hebrews",
        59 => "James",
        60 => "1 Peter",
        61 => "2 Peter",
        62 => "1 John",
        63 => "2 John",
        64 => "3 John",
        65 => "Jude",
        66 => "Revelation",
        _ => panic!("Unknown book id {id}"),
    }
}
