// SPDX-License-Identifier: GPL-2.0
#[allow(dead_code)]
struct Point<T> {
    x: T,
    y: T,
}

#[allow(dead_code)]
enum Option<T> {
    Some(T),
    None,
}

#[allow(dead_code)]
enum Result<T, E> {
    Ok(T),
    Err(E),
}

#[cfg(test)]
mod tests {
    #[test]
    fn struct_point_u32() {
        use super::Point;
        struct Test {
            name: &'static str,
            data: Point<u32>,
            want: (u32, u32),
        }
        let tests = [
            Test {
                name: "Point(1, 2)",
                data: Point { x: 1, y: 2 },
                want: (1, 2),
            },
            Test {
                name: "Point(2, 1)",
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
    #[test]
    fn enum_option_some_u32() {
        struct Test {
            name: &'static str,
            data: super::Option<u32>,
            want: u32,
        };
        let tests = [
            Test {
                name: "value '1'",
                data: super::Option::Some(1),
                want: 1,
            },
            Test {
                name: "value '2'",
                data: super::Option::Some(2),
                want: 2,
            },
        ];
        for t in &tests {
            match t.data {
                super::Option::Some(got) => {
                    debug_assert_eq!(t.want, got, "{}", t.name);
                }
                super::Option::None => {
                    panic!("unexpected None");
                }
            }
        }
    }
    #[test]
    fn enum_result_ok_u32() {
        struct Test {
            name: &'static str,
            data: super::Result<u32, String>,
            want: u32,
        }
        let tests = [Test {
            name: "value 1",
            data: super::Result::Ok(1),
            want: 1,
        }];
        for t in &tests {
            match t.data {
                super::Result::Ok(v) => {
                    debug_assert_eq!(t.want, v, "{}", t.name);
                }
                super::Result::Err(_) => {
                    panic!("unexpected Result::Err() value");
                }
            }
        }
    }
}
