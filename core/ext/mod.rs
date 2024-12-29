#[cfg(feature = "uuid")]
mod uuid;
#[cfg(feature = "uuid")]
pub use uuid::{exec_ts_from_uuid7, exec_uuid, exec_uuidblob, exec_uuidstr, UuidFunc};

#[derive(Debug, Clone, PartialEq)]
pub enum ExtFunc {
    #[cfg(feature = "uuid")]
    Uuid(UuidFunc),
}

#[allow(unreachable_patterns)] // TODO: remove when more extension funcs added
impl alloc::fmt::Display for ExtFunc {
    fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
        match self {
            #[cfg(feature = "uuid")]
            Self::Uuid(uuidfn) => write!(f, "{}", uuidfn),
            _ => write!(f, "unknown"),
        }
    }
}

#[allow(unreachable_patterns)]
impl ExtFunc {
    pub fn resolve_function(name: &str, num_args: usize) -> Option<ExtFunc> {
        match name {
            #[cfg(feature = "uuid")]
            name => UuidFunc::resolve_function(name, num_args),
            _ => None,
        }
    }
}
