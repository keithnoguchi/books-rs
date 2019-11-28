// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    #[test]
    fn open_and_create() -> Result<(), std::io::Error> {
        let tests = vec![
            "test.txt",
            "test1.txt",
            "test2.txt",
            "test3.txt",
            "test4.txt",
        ];
        for file in &tests {
            use std::fs::{self, File};
            use std::io::ErrorKind;
            let _f = File::open(file).unwrap_or_else(|err| {
                debug_assert_eq!(ErrorKind::NotFound, err.kind(), "{}", file);
                File::create(file).unwrap_or_else(|err| {
                    panic!(format!("{}: {:?}", file, err));
                })
            });
            fs::remove_file(file)?;
        }
        Ok(())
    }
}
