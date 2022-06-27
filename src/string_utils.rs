use std::str::FromStr;

pub trait StringUtils {
    fn before<'a>(&'a self, string: &str) -> Option<&'a str>;

    fn after<'a>(&'a self, string: &str) -> Option<&'a str>;

    fn between<'a>(&'a self, a: &str, b: &str) -> Option<&'a str>;

    fn after_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str>;

    fn before_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str>;

    fn to_type<'a, T: FromStr>(&'a self) -> Option<T>;

    fn trim_<'a>(&'a self) -> Option<&'a str>;

    fn replace_<'a>(&'a self, a: &str, b: &str) -> Option<String>;
}

impl StringUtils for String {
    fn before<'a>(&'a self, string: &str) -> Option<&'a str> {
        Some(
            &self[0..if let Some(e) = self.find(string) {
                e
            } else {
                self.len()
            }],
        )
    }

    fn after<'a>(&'a self, string: &str) -> Option<&'a str> {
        Some(&self[self.find(string)? + string.len()..self.len()])
    }

    fn after_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str> {
        let p = self.find(string)? + string.len();
        Some(&self[p..p + i])
    }

    fn before_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str> {
        let p = self.find(string)?;
        Some(&self[p - i..p])
    }

    fn between<'a>(&'a self, a: &str, b: &str) -> Option<&'a str> {
        Some(&self[self.find(a)? + a.len()..self.find(b)?])
    }

    fn to_type<'a, T: FromStr>(&'a self) -> Option<T> {
        self.parse().ok()
    }

    fn trim_<'a>(&'a self) -> Option<&'a str> {
        Some(self.trim())
    }

    fn replace_<'a>(&'a self, a: &str, b: &str) -> Option<String> {
        Some(self.replace(a, b))
    }
}

impl StringUtils for &str {
    fn before<'a>(&'a self, string: &str) -> Option<&'a str> {
        Some(
            &self[0..if let Some(e) = self.find(string) {
                e
            } else {
                self.len()
            }],
        )
    }

    fn after<'a>(&'a self, string: &str) -> Option<&'a str> {
        Some(&self[self.find(string)? + string.len()..self.len()])
    }

    fn after_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str> {
        let p = self.find(string)? + string.len();
        Some(&self[p..p + i])
    }

    fn before_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str> {
        let p = self.find(string)?;
        Some(&self[p - i..p])
    }

    fn between<'a>(&'a self, a: &str, b: &str) -> Option<&'a str> {
        Some(&self[self.find(a)? + a.len()..self.find(b)?])
    }

    fn to_type<'a, T: FromStr>(&'a self) -> Option<T> {
        self.parse().ok()
    }

    fn trim_<'a>(&'a self) -> Option<&'a str> {
        Some(self.trim())
    }

    fn replace_<'a>(&'a self, a: &str, b: &str) -> Option<String> {
        Some(self.replace(a, b))
    }
}

impl StringUtils for Option<String> {
    fn before<'a>(&'a self, string: &str) -> Option<&'a str> {
        if let Some(e) = self {
            Some(
                &e[0..if let Some(e) = e.find(string) {
                    e
                } else {
                    e.len()
                }],
            )
        } else {
            None
        }
    }

    fn after<'a>(&'a self, string: &str) -> Option<&'a str> {
        if let Some(e) = self {
            Some(&e[e.find(string)? + string.len()..e.len()])
        } else {
            None
        }
    }

    fn after_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str> {
        if let Some(e) = self {
            let p = e.find(string)? + string.len();
            Some(&e[p..p + i])
        } else {
            None
        }
    }

    fn before_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str> {
        if let Some(e) = self {
            let p = e.find(string)?;
            Some(&e[p - i..p])
        } else {
            None
        }
    }

    fn between<'a>(&'a self, a: &str, b: &str) -> Option<&'a str> {
        if let Some(e) = self {
            Some(&e[e.find(a)? + a.len()..e.find(b)?])
        } else {
            None
        }
    }

    fn to_type<'a, T: FromStr>(&'a self) -> Option<T> {
        if let Some(e) = self {
            e.parse().ok()
        } else {
            None
        }
    }

    fn trim_<'a>(&'a self) -> Option<&'a str> {
        if let Some(e) = self {
            Some(e.trim())
        } else {
            None
        }
    }

    fn replace_<'a>(&'a self, a: &str, b: &str) -> Option<String> {
        if let Some(e) = self {
            Some(e.replace(a, b))
        } else {
            None
        }
    }
}

impl StringUtils for Option<&str> {
    fn before<'a>(&'a self, string: &str) -> Option<&'a str> {
        if let Some(e) = self {
            Some(
                &e[0..if let Some(e) = e.find(string) {
                    e
                } else {
                    e.len()
                }],
            )
        } else {
            None
        }
    }

    fn after<'a>(&'a self, string: &str) -> Option<&'a str> {
        if let Some(e) = self {
            Some(&e[e.find(string)? + string.len()..e.len()])
        } else {
            None
        }
    }

    fn after_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str> {
        if let Some(e) = self {
            let p = e.find(string)? + string.len();
            Some(&e[p..p + i])
        } else {
            None
        }
    }

    fn before_take<'a>(&'a self, string: &str, i: usize) -> Option<&'a str> {
        if let Some(e) = self {
            let p = e.find(string)?;
            Some(&e[p - i..p])
        } else {
            None
        }
    }

    fn between<'a>(&'a self, a: &str, b: &str) -> Option<&'a str> {
        if let Some(e) = self {
            Some(&e[e.find(a)? + a.len()..e.find(b)?])
        } else {
            None
        }
    }

    fn to_type<'a, T: FromStr>(&'a self) -> Option<T> {
        if let Some(e) = self {
            e.parse().ok()
        } else {
            None
        }
    }

    fn trim_<'a>(&'a self) -> Option<&'a str> {
        if let Some(e) = self {
            Some(e.trim())
        } else {
            None
        }
    }

    fn replace_<'a>(&'a self, a: &str, b: &str) -> Option<String> {
        if let Some(e) = self {
            Some(e.replace(a, b))
        } else {
            None
        }
    }
}
