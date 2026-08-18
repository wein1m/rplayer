mod app;
mod audio;
mod tui;

use app::App;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let app = App::new()?;

    app.run()?;

    Ok(())
}

