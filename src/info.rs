use colored::Colorize;

pub fn print_usage(with_help: bool) {
    println!("{}:\n\tambient-pl {} {}", "USAGE".bold(), "[options]".bold().cyan(), "[directory]".bold().yellow());
    if with_help {
        println!("\nUse 'ambient-pl --help' for more help.");
    }
}

pub fn print_options() {
    println!("{}:", "OPTIONS".bold());

    println!("\t--help\t\tdisplay this help.");
    println!("\t-r, --recurse\tsearch folder given for music recursively.");

}

pub fn print_help() {
    print_usage(false);
    print_options();

    std::process::exit(0);
}
