use std::path::PathBuf;
use std::{fs, io};

use clap::{CommandFactory, ValueEnum};
use clap_complete::Shell;
use clap_mangen::Man;

include!("src/cli.rs");

fn main() -> Result<(), io::Error> {
    let out_dir = PathBuf::from("../../docs");

    let mut cmd = Cli::command();

    for &shell in Shell::value_variants() {
        clap_complete::generate_to(shell, &mut cmd, "gfas", &out_dir)?;
    }

    let man = Man::new(cmd);
    let mut buffer = Vec::<u8>::new();
    man.render(&mut buffer)?;

    fs::write(out_dir.join("gfas.1"), buffer)?;

    println!("cargo::rerun-if-changed=src/cli.rs");

    Ok(())
}
