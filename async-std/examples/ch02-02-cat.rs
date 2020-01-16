//! [Tasks] example
//!
//! [tasks]: https://book.async.rs/concepts/tasks.html
use async_std::io::ReadExt;
use async_std::{fs::File, io, task};
use std::{env, error::Error, process};

fn main() -> Result<(), Box<dyn Error>> {
    let argv: Vec<String> = env::args().collect();
    if argv.len() < 2 {
        usage(&argv[0]);
    }
    let reader = task::spawn(async move {
        eprintln!("[[started a task]]");
        match read_file(&argv[1]).await {
            Ok(data) => print!("{}", data),
            Err(err) => eprintln!("read_file(): {:?}", err),
        }
        eprintln!("[[finished a task]]");
    });
    eprintln!("<<waiting for the task>>");
    task::block_on(reader);
    eprintln!("<<finish wating for the task>>");
    Ok(())
}

// read_file to read a data from the specified `path` file
// and returns the String.
async fn read_file(path: &str) -> io::Result<String> {
    let mut f = File::open(path).await?;
    let mut buf = String::new();
    f.read_to_string(&mut buf).await?;
    Ok(buf)
}

fn usage(progname: &str) -> ! {
    println!("usage: {} <file_name>", progname);
    process::exit(1)
}
