// SPDX-License-Identifier: GPL-2.0
#[cfg(test)]
mod tests {
    #[test]
    fn slice() {
        struct Test {
            name: &'static str,
            data: String,
            range: (usize, usize),
            want: &'static str,
        };
        let tests = [
            Test {
                name: &"first character",
                data: String::from("This is an ASCII string"),
                range: (0, 1),
                want: &"T",
            },
            Test {
                name: &"first and second character",
                data: String::from("This is an ASCII string"),
                range: (0, 2),
                want: &"Th",
            },
            Test {
                name: &"Whole string",
                data: String::from("This is an ASCII string"),
                range: (0, 23),
                want: &"This is an ASCII string",
            },
        ];
        for t in &tests {
            let got = &t.data[t.range.0..t.range.1];
            debug_assert_eq!(got, t.want, "{}", t.name);
        }
    }
}
