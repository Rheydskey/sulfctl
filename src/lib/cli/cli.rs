use structopt::StructOpt;

#[derive(StructOpt, Debug)]
#[structopt(
    name = "Sulfur -",
    version = "0.0.1",
    author = "By Rheydskey and Sulfurium OS team",
    about = "Package Manager of Sulfurium OS"
)]
pub enum Cli {
    Service {
        #[structopt()]
        action: String,
    }
}