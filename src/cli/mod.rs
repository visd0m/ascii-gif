use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Cli {
    pub q: String,
    #[structopt(short, long)]
    pub tenor: bool,
    #[structopt(short, long)]
    pub giphy: bool,
}
