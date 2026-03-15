use byteorder::{LittleEndian, ReadBytesExt};
use std::collections::HashMap;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom};

pub mod cli;

pub struct Bible {
    file: File,
    index: HashMap<(u8, u16, u16), VerseIndex>,
}

#[derive(Clone)]
struct VerseIndex {
    offset: u64,
    length: u32,
}

impl Bible {
    pub fn open(path: &str) -> std::io::Result<Self> {
        let mut file = File::open(path)?;

        let mut magic = [0u8; 4];
        file.read_exact(&mut magic)?;

        if &magic != b"BIBL" {
            panic!("Invalid bible file");
        }

        let _version = file.read_u16::<LittleEndian>()?;
        let _book_count = file.read_u16::<LittleEndian>()?;
        let verse_count = file.read_u32::<LittleEndian>()?;
        let index_offset = file.read_u64::<LittleEndian>()?;
        let _text_offset = file.read_u64::<LittleEndian>()?;

        file.seek(SeekFrom::Start(index_offset))?;

        let mut index = HashMap::new();

        for _ in 0..verse_count {
            let book = file.read_u8()?;
            let chapter = file.read_u16::<LittleEndian>()?;
            let verse = file.read_u16::<LittleEndian>()?;
            let offset = file.read_u64::<LittleEndian>()?;
            let length = file.read_u32::<LittleEndian>()?;

            index.insert((book, chapter, verse), VerseIndex { offset, length });
        }

        Ok(Self { file, index })
    }

    pub fn get(&mut self, book: u8, chapter: u16, verse: u16) -> std::io::Result<String> {
        let idx = self
            .index
            .get(&(book, chapter, verse))
            .expect("verse not found");

        self.file.seek(SeekFrom::Start(idx.offset))?;

        let mut buf = vec![0; idx.length as usize];
        self.file.read_exact(&mut buf)?;

        Ok(String::from_utf8(buf).unwrap())
    }

    pub fn get_range(
        &mut self,
        book: u8,
        chapter: u16,
        start: u16,
        end: u16,
    ) -> std::io::Result<Vec<String>> {
        let mut verses = Vec::new();

        for v in start..=end {
            verses.push(self.get(book, chapter, v)?);
        }

        Ok(verses)
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
