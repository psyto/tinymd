fn parse_markdown_file() {}

fn print_short_banner() {
    println!("{}", get_title());
}

// print_short_banner()
// "Written by: " + env!("CARGO_PKG_AUTHORS")
// "Homepage: " + env!("CARGO_PKG_HOMEPAGE")
// "Usage: tinymd <somefile>.md"
fn print_long_banner() {
    print_short_banner();
    println!(
        "Written by: {}¥nHomepage: {}¥nUsage: tinymd <somefile>.md",
        env!("CARGO_PKG_AUTHORS"),
        env!("CARGO_PKG_HOMEPAGE")
    );
}

// [TITLE] (v[VERSION]), [DESCRIPTION]
fn get_title() -> String {
    let mut the_title = String::from(env!("CARGO_PKG_NAME"));
    the_title.push_str(" (v");
    the_title.push_str(env!("CARGO_PKG_VERSION"));
    the_title.push_str("), ");
    the_title.push_str(env!("CARGO_PKG_DESCRIPTION"));
    the_title
}

fn usage() {}

fn main() {
    usage();
}
