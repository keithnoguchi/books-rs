// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    #[test]
    fn open_and_create() -> Result<(), std::io::Error> {
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
    fn open_and_create_unwrap_or_else() -> Result<(), std::io::Error> {
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
                debug_assert_eq!(ErrorKind::NotFound, err.kind(), "{}", file);
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
    fn create_write_and_read() -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "create_and_write";
        const BUFSIZ: usize = 100;
        struct Test {
            file: &'static str,
            data: [u8; BUFSIZ],
        }
        let tests = [
            Test {
                file: "testa.txt",
                data: [b'a'; BUFSIZ],
            },
            Test {
                file: "testb.txt",
                data: [b'b'; BUFSIZ],
            },
            Test {
                file: "testc.txt",
                data: [b'c'; BUFSIZ],
            },
            Test {
                file: "testd.txt",
                data: [b'd'; BUFSIZ],
            },
            Test {
                file: "teste.txt",
                data: [b'e'; BUFSIZ],
            },
            Test {
                file: "testf.txt",
                data: [b'f'; BUFSIZ],
            },
        ];
        for t in &tests {
            use std::fs::{self, File};
            use std::io::Read;
            let file = format!("{}-{}", NAME, t.file);
            let f = File::create(&file)?;
            {
                // Use blocks, so that the BufWriter will flushes the buffer
                // before removing the file below.
                use std::io::{BufWriter, Write};
                let mut w = BufWriter::new(f);
                w.write(&t.data)?;
            }
            let mut f = File::open(&file)?;
            // Initialize the buffer with 0 so that f.read() will be happy.
            let mut got = [0; BUFSIZ];
            let n = f.read(&mut got)?;
            debug_assert_eq!(BUFSIZ, n, "{}: unexpected read length", file);
            for (i, got) in t.data.iter().enumerate() {
                debug_assert_eq!(t.data[i], *got, "{}: unexpected read data", file);
            }
            fs::remove_file(&file)?;
        }
        Ok(())
    }
}
