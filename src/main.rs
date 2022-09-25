use clap::*;
fn main() {
    let _ = Command::new("jr - jq rusty brother")
        .author("\ngithub.com/ddanielsantos")
        .version("- v0.0.5")
        .about("\nLow scale JSON processor")
        // .arg(Arg::new("in_file"))
        .after_help("Just like jq and jp, but now in Rust ")
        .get_matches();
}
