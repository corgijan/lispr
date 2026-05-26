use std::fmt::{Display, Formatter};
use std::ops::Index;

#[derive(Debug, Clone, PartialEq)]
pub enum Lispex {
    ATOM(String),
    LIST(Vec<Lispex>),
    NUMBER(i32),
    STRING(String),
    OBJECT(Vec<(String, Lispex)>), // ordered field list
}

impl Display for Lispex {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Lispex::ATOM(a) => format!("'{a}"),
                Lispex::LIST(a) => {
                    let x = a
                        .iter()
                        .map(|x| format!("{}", x))
                        .collect::<Vec<_>>()
                        .join(" ");
                    return write!(f, "#({})", x);
                }
                Lispex::NUMBER(a) => format!("{a}"),
                Lispex::STRING(a) => format!("\"{a}\""),
                Lispex::OBJECT(fields) => {
                    let x = fields
                        .iter()
                        .map(|(k, v)| format!("{}: {}", k, v))
                        .collect::<Vec<_>>()
                        .join(", ");
                    return write!(f, "{{{}}}", x);
                }
            }
        )
    }
}

impl Lispex {
    pub fn internal(&self) -> i32 {
        if let Lispex::NUMBER(a) = self {
            *a
        } else {
            0
        }
    }

    pub fn get_field(&self, name: &str) -> Lispex {
        if let Lispex::OBJECT(fields) = self {
            for (k, v) in fields {
                if k == name {
                    return v.clone();
                }
            }
            panic!("Field '{}' not found in object", name);
        } else {
            panic!("Cannot access field on non-object")
        }
    }

    pub fn set_field(&mut self, name: &str, value: Lispex) {
        if let Lispex::OBJECT(fields) = self {
            for (k, v) in fields.iter_mut() {
                if k == name {
                    *v = value;
                    return;
                }
            }
            fields.push((name.to_string(), value));
        } else {
            panic!("Cannot set field on non-object")
        }
    }

    pub fn remove(&mut self, n: usize) {
        if let Lispex::LIST(vec) = self {
            vec.remove(n);
        } else {
            panic!("cant remove from a non-List")
        }
    }
}

impl Index<usize> for Lispex {
    type Output = Self;

    fn index(&self, n: usize) -> &Self {
        if let Lispex::LIST(vec) = &self {
            &vec[n]
        } else {
            panic!("Cant index a non indexable thing")
        }
    }
}

impl From<&str> for Lispex {
    fn from(s: &str) -> Lispex {
        Lispex::STRING(s.to_string())
    }
}

impl From<String> for Lispex {
    fn from(s: String) -> Lispex {
        Lispex::STRING(s.to_string())
    }
}

impl From<i32> for Lispex {
    fn from(n: i32) -> Lispex {
        Lispex::NUMBER(n)
    }
}

impl From<Vec<Lispex>> for Lispex {
    fn from(v: Vec<Lispex>) -> Lispex {
        Lispex::LIST(v)
    }
}
