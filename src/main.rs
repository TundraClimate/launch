mod app;

use app::App;
use std::io;

fn main() -> io::Result<()> {
    App::init()?;
    
    App::run();

    Ok(())
}
