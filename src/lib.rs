use byteorder::{ByteOrder, LittleEndian};
use std::collections::HashMap;

pub mod cli;

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
        let idx = self
            .index
            .get(&(book, chapter, verse))
            .expect("verse not found");

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
