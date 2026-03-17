use anyhow::Context;
use clap::Parser;
use ttybible::{
    Bible, book_to_id,
    cli::{Cli, parse_reference},
};

fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();
    let bible = Bible::open();

    if cli.random {
        bible.get_random_verse();
        return Ok(());
    }

    let provided_book = cli.book.context("Missing book name")?;
    let provided_reference = cli.reference.context("Missing reference")?;

    let book_name = provided_book.join(" ");
    let book_id = book_to_id(&book_name);

    let reference = parse_reference(&provided_reference)?;

    for verse in bible.get_range(book_id, reference.chapter, reference.start, reference.end) {
        println!("{verse}");
    }

    println!("- {book_name} {provided_reference} ({})", cli.translation);

    Ok(())
}
