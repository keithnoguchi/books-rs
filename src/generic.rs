// SPDX-License-Identifier: GPL-2.0
struct Point<T> {
    x: T,
    y: T,
}

#[cfg(test)]
mod tests {
    #[test]
    fn struct_u32() {
        use super::Point;
        struct Test {
            name: &'static str,
            data: Point<u32>,
            want: (u32, u32),
        }
        let tests = [
            Test {
                name: &"Point(1, 2)",
                data: Point { x: 1, y: 2 },
                want: (1, 2),
            },
            Test {
                name: &"Point(2, 1)",
                data: Point { x: 2, y: 1 },
                want: (2, 1),
            },
        ];
        for t in &tests {
            let got = &t.data;
            debug_assert_eq!(t.want.0, got.x, "{}: x", t.name);
            debug_assert_eq!(t.want.1, got.y, "{}: y", t.name);
        }
    }
}
