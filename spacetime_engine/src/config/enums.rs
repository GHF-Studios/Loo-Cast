use std::ffi::OsString;
use std::path::PathBuf;

#[derive(Debug, Clone)]
pub enum ConfigValue {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
}
impl TryFrom<ConfigValue> for i8 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => {
                if i >= i8::MIN as i64 && i <= i8::MAX as i64 {
                    Ok(i as i8)
                } else {
                    Err(format!("Value {} out of range for i8", i))
                }
            }
            _ => Err(format!("Cannot convert {:?} to i8", value)),
        }
    }
}
impl TryFrom<ConfigValue> for u8 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => {
                if i >= 0 && i <= u8::MAX as i64 {
                    Ok(i as u8)
                } else {
                    Err(format!("Value {} out of range for u8", i))
                }
            }
            _ => Err(format!("Cannot convert {:?} to u8", value)),
        }
    }
}
impl TryFrom<ConfigValue> for i16 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => {
                if i >= i16::MIN as i64 && i <= i16::MAX as i64 {
                    Ok(i as i16)
                } else {
                    Err(format!("Value {} out of range for i16", i))
                }
            }
            _ => Err(format!("Cannot convert {:?} to i16", value)),
        }
    }
}
impl TryFrom<ConfigValue> for u16 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => {
                if i >= 0 && i <= u16::MAX as i64 {
                    Ok(i as u16)
                } else {
                    Err(format!("Value {} out of range for u16", i))
                }
            }
            _ => Err(format!("Cannot convert {:?} to u16", value)),
        }
    }
}
impl TryFrom<ConfigValue> for i32 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => {
                if i >= i32::MIN as i64 && i <= i32::MAX as i64 {
                    Ok(i as i32)
                } else {
                    Err(format!("Value {} out of range for i32", i))
                }
            }
            _ => Err(format!("Cannot convert {:?} to i32", value)),
        }
    }
}
impl TryFrom<ConfigValue> for u32 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => {
                if i >= 0 && i <= u32::MAX as i64 {
                    Ok(i as u32)
                } else {
                    Err(format!("Value {} out of range for u32", i))
                }
            }
            _ => Err(format!("Cannot convert {:?} to u32", value)),
        }
    }
}
impl TryFrom<ConfigValue> for i64 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => Ok(i),
            _ => Err(format!("Cannot convert {:?} to i64", value)),
        }
    }
}
impl TryFrom<ConfigValue> for u64 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => {
                if i >= 0 {
                    Ok(i as u64)
                } else {
                    Err(format!("Value {} out of range for u64", i))
                }
            }
            _ => Err(format!("Cannot convert {:?} to u64", value)),
        }
    }
}
impl TryFrom<ConfigValue> for i128 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => Ok(i as i128),
            _ => Err(format!("Cannot convert {:?} to i128", value)),
        }
    }
}
impl TryFrom<ConfigValue> for u128 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Integer(i) => {
                if i >= 0 {
                    Ok(i as u128)
                } else {
                    Err(format!("Value {} out of range for u128", i))
                }
            }
            _ => Err(format!("Cannot convert {:?} to u128", value)),
        }
    }
}
impl TryFrom<ConfigValue> for f32 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Float(f) => {
                if f >= f32::MIN as f64 && f <= f32::MAX as f64 {
                    Ok(f as f32)
                } else {
                    Err(format!("Value {} out of range for f32", f))
                }
            }
            ConfigValue::Integer(i) => Ok(i as f32),
            _ => Err(format!("Cannot convert {:?} to f32", value)),
        }
    }
}
impl TryFrom<ConfigValue> for f64 {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::Float(f) => Ok(f),
            ConfigValue::Integer(i) => Ok(i as f64),
            _ => Err(format!("Cannot convert {:?} to f64", value)),
        }
    }
}
impl TryFrom<ConfigValue> for String {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::String(s) => Ok(s),
            _ => Err(format!("Cannot convert {:?} to String", value)),
        }
    }
}
impl TryFrom<ConfigValue> for OsString {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::String(s) => Ok(OsString::from(s)),
            _ => Err(format!("Cannot convert {:?} to OsString", value)),
        }
    }
}
impl TryFrom<ConfigValue> for PathBuf {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::String(s) => Ok(PathBuf::from(s)),
            _ => Err(format!("Cannot convert {:?} to PathBuf", value)),
        }
    }
}
impl TryFrom<ConfigValue> for char {
    type Error = String;

    fn try_from(value: ConfigValue) -> Result<Self, Self::Error> {
        match value {
            ConfigValue::String(s) => {
                if s.chars().count() == 1 {
                    Ok(s.chars().next().unwrap())
                } else {
                    Err(format!("String {:?} cannot be converted to char", s))
                }
            }
            _ => Err(format!("Cannot convert {:?} to char", value)),
        }
    }
}
