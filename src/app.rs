use clap::Parser;
use toml::map::Map;
use std::{fs::{self, File}, io, process::Command};

#[derive(Parser)]
#[command(version, author, about, long_about = None)]
pub struct App {
    /// Set command by name
    #[arg(long, value_name = "COMMAND")]
    set: Option<String>,

    /// Remove command by name
    #[arg(long)]
    rm: bool,

    /// Show info by name
    #[arg(long)]
    show: bool,

    /// Run on all scripts(ignored if --set)
    #[arg(short, long)]
    all: bool,

    /// script name
    script: Option<String>,
}

impl App {
    pub fn run() -> io::Result<()> {
        let app = App::parse();
        let script = app.script;
        
        if let Some(v) = app.set {
            if let Some(script) = script {
                set_script(script, v)?;
            } else {
                println!("SCRIPT not entered");
            }
            return Ok(());
        }

        if app.rm {
            if app.all {
                for k in read_conf()?.keys() {
                    rm_script(k.clone())?;
                }
            } else {
                if let Some(script) = script {
                    rm_script(script)?;
                } else {
                    println!("SCRIPT not entered");
                }
            }
            return Ok(());
        }

        if app.show {
            let conf = read_conf()?;
            if app.all {
                for e in conf.iter() {
                    println!("{}: {}", e.0, e.1);
                }
            } else {
                if let Some(script) = script {
                    let cmd = conf.get(script.as_str()).unwrap();
                    println!("{}: {}", script, cmd);
                } else {
                    println!("SCRIPT not entered");
                }
            }
            return Ok(());
        }

        let conf = read_conf()?;
        if let Some(script) = script {
            if conf.contains_key(script.as_str()) {
                let cmd = conf.get(script.as_str()).unwrap().as_str().unwrap();
                let mut cmd: Vec<_> = cmd.split(" ").collect();
                let command = cmd[0];
                cmd.remove(0);
                let args = cmd;
                Command::new(command).args(args).spawn()?;
            }
        } else {
            println!("SCRIPT not entered");
        }
        std::thread::sleep(std::time::Duration::from_secs(1));

        Ok(())
    }

    pub fn init() -> io::Result<()> {
        // create data directory
        let data_dir = dirs::data_local_dir().unwrap();
        let data_dir = data_dir.join("launch");
        if !data_dir.exists() {
            fs::create_dir(&data_dir)?;
        }

        // create configration file
        let config = data_dir.join("launch.config.toml");
        if !config.exists() {
            File::create(config)?;
        }

        Ok(())
    }
}

fn read_conf() -> io::Result<Map<String, toml::Value>> {
    let config = dirs::data_local_dir().unwrap().join("launch").join("launch.config.toml");
    let config = fs::read_to_string(config).unwrap();
    let conf: Map<_, _> = toml::from_str(config.as_str()).expect("Unable to load config");
    Ok(conf)
}

fn set_script(script: String, cmd: String) -> io::Result<()> {
    let config = dirs::data_local_dir().unwrap().join("launch").join("launch.config.toml");
    let mut conf: Map<_, _> = read_conf()?;
    if conf.contains_key(script.as_str()) {
        conf.remove(script.as_str());
    }
    conf.insert(script, toml::Value::String(cmd));
    fs::write(config, conf.to_string().as_bytes())?;
    
    Ok(())
}

fn rm_script(script: String) -> io::Result<()> {
    let config = dirs::data_local_dir().unwrap().join("launch").join("launch.config.toml");
    let mut conf: Map<_, _> = read_conf()?;
    conf.remove(script.as_str());
    fs::write(config, conf.to_string().as_bytes())?;
    Ok(())
}
