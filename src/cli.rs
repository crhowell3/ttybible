use clap::Parser;

#[derive(Parser, Debug)]
#[clap(name = "bible")]
#[command(
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
    #[arg(required = true, num_args = 1.., help = "Book name")]
    pub book: Vec<String>,
    #[arg(help = "Chapter and verse(s)")]
    pub reference: String,
}

pub struct ParsedReference {
    pub chapter: u16,
    pub start: u16,
    pub end: u16,
}

pub fn parse_reference(reference: &str) -> ParsedReference {
    let (chapter, verse_part) = reference.split_once(':').unwrap();

    let chapter = chapter.parse().unwrap();

    let (start, end) = if let Some((a, b)) = verse_part.split_once('-') {
        (a.parse().unwrap(), b.parse().unwrap())
    } else {
        let v = verse_part.parse().unwrap();
        (v, v)
    };

    ParsedReference {
        chapter,
        start,
        end,
    }
}
