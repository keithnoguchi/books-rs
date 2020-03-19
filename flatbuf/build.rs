use std::process::Command;

fn main() -> std::io::Result<()> {
    Command::new("flatc")
        .args(&["-o", "./src", "-r"])
        .arg("./schema/monster.fbs")
        .output()?;
    Ok(())
}
