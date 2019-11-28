// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    #[test]
    fn open_and_create_with_match() -> Result<(), std::io::Error> {
        const NAME: &str = "open_and_create_with_match";
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
            let _f = match File::open(file.clone()) {
                Ok(f) => f,
                Err(err) => match err.kind() {
                    ErrorKind::NotFound => match File::create(file.clone()) {
                        Ok(f) => f,
                        Err(err) => {
                            panic!("{}: create: {:?}", file, err);
                        }
                    },
                    other_error => {
                        panic!("{}: open: {:?}", file, other_error);
                    }
                },
            };
            match fs::remove_file(file.clone()) {
                Ok(_) => continue,
                Err(err) => {
                    eprintln!("{}: remove_file: {:?}", file, err);
                    return Err(err);
                }
            };
        }
        Ok(())
    }
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
            let _f = File::open(file.clone()).unwrap_or_else(|err| {
                debug_assert_eq!(ErrorKind::NotFound, err.kind(), "{}", file);
                File::create(file.clone()).unwrap_or_else(|err| {
                    panic!(format!("{}: {:?}", file, err));
                })
            });
            // Can't use unwrap_or_else here because it expects
            // unit type and doesn't allow return Err(err).  And
            // also, expect macro expectes the string slice, which
            // doesn't work well with the following error message.
            fs::remove_file(file.clone()) {
                Ok(_) => continue,
                Err(err) => {
                    eprintln!("{}: remove_file: {:?}", file, err);
                    return Err(err);
                }
            };
        }
        Ok(())
    }
}
