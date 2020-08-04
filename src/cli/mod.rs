use structopt::StructOpt;

#[derive(StructOpt, Debug)]
pub struct Cli {
    #[structopt(long)]
    pub q: Option<String>,
    #[structopt(long)]
    pub id: Option<String>,
    #[structopt(short, long)]
    pub tenor: bool,
    #[structopt(short, long)]
    pub giphy: bool,
}
