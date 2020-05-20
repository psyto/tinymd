fn get_version() -> u16 {
    1000
}

fn usage() {
    let the_version = get_version();
    println!("tinymd, a markdown compiler written by Hiroyuki Saito");
    println!("Version {}", the_version);
}

fn main() {
    usage();
}
