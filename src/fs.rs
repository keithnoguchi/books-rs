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
}
