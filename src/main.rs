use ansi_term::Color;

mod app;
mod data;
mod nav;

fn main() {
    match app::run() {
        Ok(_) => {}
        Err(e) => {
            eprintln!("{} {}", Color::Red.paint("[tutel]"), e);
            std::process::exit(1);
        }
    }
}
