use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "SulfCtl",
    version = "0.0.1",
    author = "SulfuriumOs team",
    about = "Control utility of Sulfurium OS"
)]
pub enum Cli {
    Service {
        #[structopt()]
        action: String,
        #[structopt()]
        packages: Vec<String>,
    }
}