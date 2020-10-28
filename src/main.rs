mod lib;
use lib::cli::cli::Cli;
fn main() {    
    if users::get_current_uid() != 0 {
        println!("{}", console::style("Please run this program as root!").red());
        std::process::exit(1);
    }
    match Cli::from_args() {
        Cli::Service { action, packages } => {
            println!("{}", action);
        }
    }
    println!("Sulfctl");
}