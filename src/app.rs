use clap::Parser;

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct App {
    /// Set command by name
    #[arg(long, value_name = "COMMAND")]
    set: Option<String>,

    /// Remove command by name
    #[arg(long, value_name = "COMMAND")]
    rm: Option<String>,

    /// Show info by name
    #[arg(long)]
    show: bool,

    /// Run on all scripts(ignored if --add)
    #[arg(short, long)]
    all: bool,

    /// script name
    script: String,
}

impl App {
    pub fn run() {
        let app = App::parse();
    }
}
