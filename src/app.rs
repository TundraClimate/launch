use clap::Parser;
use std::{fs::{self, File}, io};
use dotenv;

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

    pub fn init() -> io::Result<()> {
        // create data directory
        let data_dir = dirs::data_local_dir().unwrap();
        let data_dir = data_dir.join("launch");
        if !data_dir.exists() {
            fs::create_dir(&data_dir)?;
        }

        // create configration file
        let config = data_dir.join("launch.config");
        if !config.exists() {
            File::create(config)?;
        }

        load_scripts();
        
        Ok(())
    }
}

fn load_scripts() {
    let config = dirs::data_local_dir().unwrap().join("launch").join("launch.config");
    dotenv::from_path(config).expect("file cannot be read");
}
