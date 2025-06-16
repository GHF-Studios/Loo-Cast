use crate::log::types::LogPathSegment;

pub trait LogPathExt {
    fn crate_name(&self) -> &str;
    fn file_name(&self) -> Option<&str>;
    fn full_module_path(&self) -> Vec<&str>;
    fn submodule_path(&self) -> Option<Vec<&str>>;
    fn line_number(&self) -> Option<u32>;
    fn is_submodule_path(&self) -> bool;
    fn to_path_string(&self) -> String;
}

impl LogPathExt for [LogPathSegment] {
    fn crate_name(&self) -> &str {
        for seg in self {
            if let LogPathSegment::Crate(name) = seg {
                return name;
            }
        }
        panic!("No crate segment in path");
    }

    fn file_name(&self) -> Option<&str> {
        for seg in self {
            if let LogPathSegment::File(name) = seg {
                return Some(name);
            }
        }
        None
    }

    fn full_module_path(&self) -> Vec<&str> {
        self.iter()
            .filter_map(|seg| match seg {
                LogPathSegment::Module(name) => Some(name.as_str()),
                _ => None,
            })
            .collect()
    }

    fn submodule_path(&self) -> Option<Vec<&str>> {
        let mut subs = Vec::new();
        let mut in_subs = false;

        for seg in self {
            match seg {
                LogPathSegment::Submodule(name) => {
                    in_subs = true;
                    subs.push(name.as_str());
                }
                LogPathSegment::Line(_) if in_subs => break,
                _ if in_subs => break,
                _ => {}
            }
        }

        if subs.is_empty() {
            None
        } else {
            Some(subs)
        }
    }

    fn line_number(&self) -> Option<u32> {
        for seg in self {
            if let LogPathSegment::Line(n) = seg {
                return Some(*n);
            }
        }
        None
    }

    fn is_submodule_path(&self) -> bool {
        self.iter().any(|seg| matches!(seg, LogPathSegment::Submodule(_)))
    }

    fn to_path_string(&self) -> String {
        let mut parts = Vec::new();
        for seg in self {
            match seg {
                LogPathSegment::Crate(name) => parts.push(name.clone()),
                LogPathSegment::Module(name) => parts.push(name.clone()),
                LogPathSegment::File(name) => parts.push(name.clone()),
                LogPathSegment::Submodule(name) => parts.push(format!("::{name}")),
                LogPathSegment::Line(n) => parts.push(format!(":{}", n)),
            }
        }

        let mut s = String::new();
        for (i, part) in parts.iter().enumerate() {
            if part.starts_with("::") {
                s.push_str(part);
            } else if part.starts_with(':') {
                s.push_str(part);
            } else {
                if i > 0 {
                    s.push('/');
                }
                s.push_str(part);
            }
        }
        s
    }
}