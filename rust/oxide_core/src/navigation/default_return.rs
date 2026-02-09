use crate::navigation::RouteReturn;

/// A route return type that can provide a default value.
///
/// How: a default is used when a route pop completes without a result or when the result cannot
/// be decoded.
pub trait DefaultReturn: RouteReturn {
    /// Creates a default return value for this type.
    fn default_return() -> Self;
}

impl DefaultReturn for bool {
    fn default_return() -> Self {
        false
    }
}

impl DefaultReturn for i32 {
    fn default_return() -> Self {
        0
    }
}

impl DefaultReturn for i64 {
    fn default_return() -> Self {
        0
    }
}

impl DefaultReturn for u32 {
    fn default_return() -> Self {
        0
    }
}

impl DefaultReturn for u64 {
    fn default_return() -> Self {
        0
    }
}

impl DefaultReturn for String {
    fn default_return() -> Self {
        String::new()
    }
}

impl<T> DefaultReturn for Option<T>
where
    T: RouteReturn,
{
    fn default_return() -> Self {
        None
    }
}
