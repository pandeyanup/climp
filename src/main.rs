mod app;
mod blocks;
mod key_event;
mod ui;

use app::App;
use clap::{command, Arg};
use std::{fs, io};

fn main() -> io::Result<()> {
    let matches = command!()
        .about("A cli music player")
        .version("1.1.0")
        .arg(
            Arg::new("directory")
                .short('d')
                .long("dir")
                .help_heading(Some("Play music from a directory")),
        )
        .get_matches();

    let dir_path = matches
        .get_one::<String>("directory")
        .map(|s| s.to_string())
        .unwrap_or_else(|| {
            let directory: String = dialoguer::Input::new()
                .with_prompt("Enter the directory path")
                .interact()
                .unwrap();
            directory
        });

    let path_buf = fs::read_dir(dir_path.clone());
    match path_buf {
        Ok(_) => {
            let app = App::new(&dir_path.to_owned());
            ui::run_ui(app)?;
        }
        Err(_) => {
            println!("Invalid path to directory");
            std::process::exit(1);
        }
    }
    Ok(())
}
