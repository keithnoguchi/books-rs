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
    fn create_and_write() -> Result<(), Box<dyn std::error::Error>> {
        const NAME: &str = "create_and_write";
        struct Test {
            file: &'static str,
            data: [u8; 100],
        }
        let tests = [
            Test {
                file: "testa.txt",
                data: [b'a'; 100],
            },
            Test {
                file: "testb.txt",
                data: [b'b'; 100],
            },
            Test {
                file: "testc.txt",
                data: [b'c'; 100],
            },
            Test {
                file: "testd.txt",
                data: [b'd'; 100],
            },
            Test {
                file: "teste.txt",
                data: [b'e'; 100],
            },
            Test {
                file: "testf.txt",
                data: [b'f'; 100],
            },
        ];
        for t in &tests {
            use std::fs::{self, File};
            let file = format!("{}-{}", NAME, t.file);
            let f = File::create(&file)?;
            {
                use std::io::{BufWriter, Write};
                // Use blocks, so that the BufWriter will flushes the buffer
                // before removing the file below.
                let mut w = BufWriter::new(f);
                w.write(&t.data)?;
            }
            fs::remove_file(&file)?;
        }
        Ok(())
    }
}
