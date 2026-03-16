use clap::Parser;
use ttybible::{
    Bible, book_to_id,
    cli::{Cli, parse_reference},
};

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();
    let bible = Bible::open();
    let book = cli.book.join(" ");
    let book_id = book_to_id(&book);
    let reference = parse_reference(&cli.reference);

    for verse in bible.get_range(book_id, reference.chapter, reference.start, reference.end) {
        println!("{verse}");
    }
    println!("-{book} {}", cli.reference);

    Ok(())
}
