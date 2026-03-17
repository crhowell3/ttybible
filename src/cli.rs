use anyhow::anyhow;
use clap::Parser;
use std::num::ParseIntError;

use crate::VERSION_AND_GIT_HASH;

#[derive(Parser, Debug)]
#[clap(name = "bible")]
#[command(
    version = VERSION_AND_GIT_HASH,
    about = "terminal-based bible",
    author = "Cameron Howell <me@crhowell.com>",
    display_name = "ttybible",
    help_template = "{name} {version}
author: {author-with-newline}{about-with-newline}
{usage-heading} {usage}

{all-args}{after-help}
"
)]
pub struct Cli {
    #[arg(
        long,
        help = "Select translation (ESV is the default)",
        default_value = "ESV"
    )]
    pub translation: String,

    #[arg(num_args=1..=3, help = "Book name")]
    pub book: Option<Vec<String>>,

    #[arg(help = "Chapter and verse(s)")]
    pub reference: Option<String>,

    #[arg(long, help = "Prints a random verse")]
    pub random: bool,
}

pub struct ParsedReference {
    pub chapter: u16,
    pub start: u16,
    pub end: u16,
}

///
/// # Errors
/// - Will return an error if the chapter-verse(s) reference is ill-formed and unable to be parsed
///
pub fn parse_reference(reference: &str) -> anyhow::Result<ParsedReference> {
    let (chapter, verse_part) = reference.split_once(':').ok_or_else(|| {
        anyhow!("Invalid reference '{reference}'. Expected format <CHAPTER>:<VERSE>")
    })?;

    let chapter: u16 = chapter
        .parse()
        .map_err(|_: ParseIntError| anyhow!("Invalid chapter '{chapter}'"))?;

    let (start, end) = if let Some((a, b)) = verse_part.split_once('-') {
        let start: u16 = a.parse().map_err(|_| anyhow!("Invalid verse '{a}'"))?;

        let end: u16 = b.parse().map_err(|_| anyhow!("Invalid verse '{b}'"))?;

        (start, end)
    } else {
        let v = verse_part
            .parse()
            .map_err(|_| anyhow!("Invalid verse '{verse_part}'"))?;

        (v, v)
    };

    Ok(ParsedReference {
        chapter,
        start,
        end,
    })
}
