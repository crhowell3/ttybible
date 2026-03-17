use anyhow::anyhow;
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
    ///
    /// # Errors
    /// - Will return an error if `index_offset` cannot be converted to a `usize`
    ///
    /// # Panics
    /// - Will panic if bible.dat's binary header is ill-formatted
    ///
    pub fn open() -> anyhow::Result<Self> {
        let data = BIBLE_DATA;
        let mut pos = 0;
        assert!(&data[0..4] == b"BIBL", "Invalid Bible binary encoding");

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

        let mut pos = usize::try_from(index_offset)?;

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

        Ok(Self { data, index })
    }

    ///
    /// # Errors
    /// - Will return an error if hash map index lookup fails, `idx.offset` is unable to be
    ///   converted to a `usize`, or the byte slice contains invalid UTF-8 encoding
    ///
    pub fn get(&self, book: u8, chapter: u16, verse: u16) -> anyhow::Result<String> {
        let idx = self
            .index
            .get(&(book, chapter, verse))
            .ok_or_else(|| anyhow!("Verse does not exist"))?;

        let start = usize::try_from(idx.offset)?;
        let end = start + idx.length as usize;

        let slice = &self.data[start..end];

        Ok(std::str::from_utf8(slice)
            .map_err(|_| anyhow!("UTF-8 data possibly corrupted"))?
            .to_string())
    }

    ///
    /// # Errors
    /// - Will return an error if `self.get()` returns an Error
    ///
    pub fn get_range(
        &self,
        book: u8,
        chapter: u16,
        start: u16,
        end: u16,
    ) -> anyhow::Result<Vec<String>> {
        let mut verses = Vec::new();

        for v in start..=end {
            verses.push(self.get(book, chapter, v)?);
        }

        Ok(verses)
    }

    ///
    /// # Errors
    /// - Will return an error if `self.get()` returns an Error or if `id_to_book`
    ///   returns an Error (which should never happen)
    ///
    pub fn get_random_verse(&self) -> anyhow::Result<String> {
        let mut rng = rand::rng();

        let verses: Vec<_> = self.index.keys().collect();

        let idx = rng.random_range(0..verses.len());

        let (book, chapter, verse) = verses[idx];

        let text = self.get(*book, *chapter, *verse)?;

        let book_name = id_to_book(*book)?;

        Ok(format!("{text}\n- {book_name} {chapter}:{verse}"))
    }
}

///
/// # Errors
/// - Will return an error if a non-existent Bible book is provided
///
pub fn book_to_id(name: &str) -> anyhow::Result<u8> {
    match name.to_lowercase().as_str() {
        "genesis" => Ok(1),
        "exodus" => Ok(2),
        "leviticus" => Ok(3),
        "numbers" => Ok(4),
        "deuteronomy" => Ok(5),
        "joshua" => Ok(6),
        "judges" => Ok(7),
        "ruth" => Ok(8),
        "1 samuel" => Ok(9),
        "2 samuel" => Ok(10),
        "1 kings" => Ok(11),
        "2 kings" => Ok(12),
        "1 chronicles" => Ok(13),
        "2 chronicles" => Ok(14),
        "ezra" => Ok(15),
        "nehemiah" => Ok(16),
        "esther" => Ok(17),
        "job" => Ok(18),
        "psalm" | "psalms" => Ok(19),
        "proverbs" => Ok(20),
        "ecclesiastes" => Ok(21),
        "song of songs" | "song of solomon" => Ok(22),
        "isaiah" => Ok(23),
        "jeremiah" => Ok(24),
        "lamentations" => Ok(25),
        "ezekiel" => Ok(26),
        "daniel" => Ok(27),
        "hosea" => Ok(28),
        "joel" => Ok(29),
        "amos" => Ok(30),
        "obadiah" => Ok(31),
        "jonah" => Ok(32),
        "micah" => Ok(33),
        "nahum" => Ok(34),
        "habakkuk" => Ok(35),
        "zephaniah" => Ok(36),
        "haggai" => Ok(37),
        "zechariah" => Ok(38),
        "malachi" => Ok(39),
        "matthew" => Ok(40),
        "mark" => Ok(41),
        "luke" => Ok(42),
        "john" => Ok(43),
        "acts" => Ok(44),
        "romans" => Ok(45),
        "1 corinthians" => Ok(46),
        "2 corinthians" => Ok(47),
        "galatians" => Ok(48),
        "ephesians" => Ok(49),
        "philippians" => Ok(50),
        "colossians" => Ok(51),
        "1 thessalonians" => Ok(52),
        "2 thessalonians" => Ok(53),
        "1 timothy" => Ok(54),
        "2 timothy" => Ok(55),
        "titus" => Ok(56),
        "philemon" => Ok(57),
        "hebrews" => Ok(58),
        "james" => Ok(59),
        "1 peter" => Ok(60),
        "2 peter" => Ok(61),
        "1 john" => Ok(62),
        "2 john" => Ok(63),
        "3 john" => Ok(64),
        "jude" => Ok(65),
        "revelation" => Ok(66),
        _ => Err(anyhow!("Unknown book {name}")),
    }
}

///
/// # Errors
/// - Will return an error if an ID outside the valid range is provided
///
pub fn id_to_book(id: u8) -> anyhow::Result<&'static str> {
    match id {
        1 => Ok("Genesis"),
        2 => Ok("Exodus"),
        3 => Ok("Leviticus"),
        4 => Ok("Numbers"),
        5 => Ok("Deuteronomy"),
        6 => Ok("Joshua"),
        7 => Ok("Judges"),
        8 => Ok("Ruth"),
        9 => Ok("1 Samuel"),
        10 => Ok("2 Samuel"),
        11 => Ok("1 Kings"),
        12 => Ok("2 Kings"),
        13 => Ok("1 Chronicles"),
        14 => Ok("2 Chronicles"),
        15 => Ok("Ezra"),
        16 => Ok("Nehemiah"),
        17 => Ok("Esther"),
        18 => Ok("Job"),
        19 => Ok("Psalms"),
        20 => Ok("Proverbs"),
        21 => Ok("Ecclesiastes"),
        22 => Ok("Song of Solomon"),
        23 => Ok("Isaiah"),
        24 => Ok("Jeremiah"),
        25 => Ok("Lamentations"),
        26 => Ok("Ezekiel"),
        27 => Ok("Daniel"),
        28 => Ok("Hosea"),
        29 => Ok("Joel"),
        30 => Ok("Amos"),
        31 => Ok("Obadiah"),
        32 => Ok("Jonah"),
        33 => Ok("Micah"),
        34 => Ok("Nahum"),
        35 => Ok("Habakkuk"),
        36 => Ok("Zephaniah"),
        37 => Ok("Haggai"),
        38 => Ok("Zechariah"),
        39 => Ok("Malachi"),
        40 => Ok("Matthew"),
        41 => Ok("Mark"),
        42 => Ok("Luke"),
        43 => Ok("John"),
        44 => Ok("Acts"),
        45 => Ok("Romans"),
        46 => Ok("1 Corinthians"),
        47 => Ok("2 Corinthians"),
        48 => Ok("Galatians"),
        49 => Ok("Ephesians"),
        50 => Ok("Philippians"),
        51 => Ok("Colossians"),
        52 => Ok("1 Thessalonians"),
        53 => Ok("2 Thessalonians"),
        54 => Ok("1 Timothy"),
        55 => Ok("2 Timothy"),
        56 => Ok("Titus"),
        57 => Ok("Philemon"),
        58 => Ok("Hebrews"),
        59 => Ok("James"),
        60 => Ok("1 Peter"),
        61 => Ok("2 Peter"),
        62 => Ok("1 John"),
        63 => Ok("2 John"),
        64 => Ok("3 John"),
        65 => Ok("Jude"),
        66 => Ok("Revelation"),
        _ => Err(anyhow!("Unknown book id {id}")),
    }
}
