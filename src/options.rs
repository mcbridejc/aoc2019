use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct StandardOptions {
    /// Activate debug mode
    #[structopt(short, long)]
    pub debug: bool,

    /// Run part
    #[structopt(short="o", long="one", help="Run part 1", conflicts_with("part2"), required_unless("part2"))]
    pub part1: bool,

    /// Run part2
    #[structopt(short="t", long="two", help="Run part 2", conflicts_with("part1"), required_unless("part1"))]
    pub part2: bool,

    /// Input data file
    #[structopt(short, long, required=false, default_value="none")]
    pub input: String,
}
