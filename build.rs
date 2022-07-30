use clap_complete::Shell;
use clap_complete::generate_to;
use std::env;
use std::io::Error;

include!("src/app.rs");

fn main() -> Result<(), Error> {
    println!("cargo:rerun-if-changed=src/app.rs");
    
    let outdir = match env::var_os("OUT_DIR") {
        None => return Ok(()),
        Some(outdir) => outdir,
    };

    let mut cmd = new();

    for shell in [Shell::Bash, Shell::Zsh, Shell::Fish, Shell::Elvish] {
        generate_to(shell, &mut cmd, env!("CARGO_PKG_NAME"), outdir.clone())?;
    }

    Ok(())
}
