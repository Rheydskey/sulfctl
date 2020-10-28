mod lib;
use lib::cli::cli::Cli;
use structopt::StructOpt;
fn main() {    
    if users::get_current_uid() != 0 {
        println!("{}", console::style("Please run this program as root!").red());
        std::process::exit(1);
    }
    match Cli::from_args() {
        Cli::Service { action, service } => {
            let act = action.as_str();
            match act {
                "start" => {
                    lib::services::services::start_srv(service);
                }
                "stop" => {
                    lib::services::services::stop_srv(service);
                }
                "enable" => {
                    lib::services::services::enable_srv(service);
                }
                "disable" => {
                    lib::services::services::disable_srv(service);
                }
                _ => {
                    println!("{}", console::style(&format!("Invalid action: {}", action)).red());
                    std::process::exit(0);
                }
            }
        }
    }
}