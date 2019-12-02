// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    use std::error::Error;
    #[test]
    fn open_and_create() -> Result<(), Box<dyn Error>> {
        const NAME: &str = "open_and_create";
        let tests = [
            "test.txt",
            "test1.txt",
            "test2.txt",
            "test3.txt",
            "test4.txt",
        ];
        for t in &tests {
            use std::fs::{self, File};
            use std::io::ErrorKind;
            let file = format!("{}-{}", NAME, t);
            let _f = match File::open(&file) {
                Ok(f) => f,
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => match File::create(&file) {
                        Ok(f) => f,
                        Err(err) => panic!("{}: create: {:?}", file, err),
                    },
                    other_error => panic!("{}: open: {:?}", file, other_error),
                },
            };
            let msg = format!("{}: remove_file", file);
            fs::remove_file(&file).expect(&msg);
        }
        Ok(())
    }
    #[test]
    fn open_and_create_unwrap_or_else() -> Result<(), Box<dyn Error>> {
        const NAME: &str = "open_and_create_unwrap_or_else";
        let tests = [
            "test.txt",
            "test1.txt",
            "test2.txt",
            "test3.txt",
            "test4.txt",
        ];
        for t in &tests {
            use std::fs::{self, File};
            use std::io::ErrorKind;
            let file = format!("{}-{}", NAME, t);
            let _f = File::open(&file).unwrap_or_else(|err| {
                assert_eq!(ErrorKind::NotFound, err.kind(), "{}", file);
                File::create(&file).unwrap_or_else(|err| {
                    panic!(format!("{}: {:?}", file, err));
                })
            });
            let msg = format!("{}: remove_file", file);
            fs::remove_file(&file).expect(&msg);
        }
        Ok(())
    }
    #[test]
    fn create_write_and_read() -> Result<(), Box<dyn Error>> {
        const NAME: &str = "create_and_write";
        struct Test {
            name: &'static str,
            data: u8,
            bufsiz: usize,
        }
        let tests = [
            Test {
                name: "1 bytes 'a'",
                data: b'a',
                bufsiz: 1,
            },
            Test {
                name: "100 bytes 'b'",
                data: b'b',
                bufsiz: 100,
            },
            Test {
                name: "1024 bytes 'x'",
                data: b'x',
                bufsiz: 1024,
            },
            Test {
                name: "1MiB bytes 'y'",
                data: b'y',
                bufsiz: 1024 * 1024,
            },
            Test {
                name: "4MiB 'z'",
                data: b'z',
                bufsiz: 4 * 1024 * 1024,
            },
        ];
        for t in &tests {
            use std::fs::File;
            use std::io::prelude::*;
            let file = format!("{}-{}", NAME, t.data);
            let f = File::create(&file)?;
            {
                // Use blocks, so that the BufWriter will flushes the buffer
                // before removing the file below.
                let mut w = std::io::BufWriter::new(f);
                let data = vec![t.data; t.bufsiz];
                w.write(&data)?;
            }
            let mut f = File::open(&file)?;
            // Initialize the buffer with 0 so that f.read() will be happy.
            let mut got = vec![0u8; t.bufsiz];
            let n = f.read(&mut got)?;
            assert_eq!(t.bufsiz, n, "{}: unexpected read length", t.name);
            for got in &got {
                assert_eq!(t.data, *got, "{}: unexpected read data", t.name);
            }
            std::fs::remove_file(&file)?;
        }
        Ok(())
    }
}
