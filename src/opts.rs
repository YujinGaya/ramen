use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "horn", about = "Opinionated static site generator.")]
pub enum Horn {
    Build {},
}
