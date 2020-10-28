use structopt::StructOpt;
#[derive(StructOpt, Debug)]
#[structopt(
    name = "SulfCtl",
    version = "0.0.1",
    author = "SulfuriumOs team",
    about = "Control utility of Sulfurium OS"
)]
pub enum Cli {
    /// Services management
    Service {
        #[structopt()]
        action: String,
        #[structopt()]
        service: String,
    }
}