mod eh_url;
mod settings;
mod eh_engine;
mod parser;
mod test_helper;
mod eh_config;
mod structures;

#[derive(Debug)]
pub enum EhError {
    ParseError(parser::ParseError),
    EngineError(reqwest::Error),
    FromServerError(parser::ParseError),
}

impl std::fmt::Display for EhError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EhError::ParseError(e) => e.fmt(f),
            EhError::EngineError(e) => e.fmt(f),
            EhError::FromServerError(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for EhError {}

impl From<reqwest::Error> for EhError {
    fn from(value: reqwest::Error) -> Self {
        EhError::EngineError(value)
    }
}

impl From<parser::ParseError> for EhError {
    fn from(value: parser::ParseError) -> Self {
        match value {
            parser::ParseError::FromServer(_) => EhError::FromServerError(value),
            _ => EhError::ParseError(value),
        }
    }
}

type EhResult<T> = Result<T, EhError>;

// CONCAT

pub const unsafe fn transmute<From, To>(from: From) -> To {
    union Transmute<From, To> {
        from: std::mem::ManuallyDrop<From>,
        to: std::mem::ManuallyDrop<To>,
    }

    std::mem::ManuallyDrop::into_inner(Transmute { from: std::mem::ManuallyDrop::new(from) }.to)
}

pub const unsafe fn concat<First, Second, Out>(a: &[u8], b: &[u8]) -> Out
    where
        First: Copy,
        Second: Copy,
        Out: Copy,
{
    #[repr(C)]
    #[derive(Copy, Clone)]
    struct Both<A, B>(A, B);

    let arr: Both<First, Second> = Both(
        *transmute::<_, *const First>(a.as_ptr()),
        *transmute::<_, *const Second>(b.as_ptr()),
    );

    transmute(arr)
}

#[allow(non_snake_case)]
#[macro_export]
macro_rules! CONCAT {
    () => {
        ""
    };
    ($a:expr) => {
        $a
    };
    ($a:expr, $b:expr) => {{
        let bytes: &'static [u8] = unsafe {
            &$crate::concat::<
                [u8; $a.len()],
                [u8; $b.len()],
                [u8; $a.len() + $b.len()],
            >($a.as_bytes(), $b.as_bytes())
        };

        unsafe { $crate::transmute::<_, &'static str>(bytes) }
    }};
    ($a:expr, $($rest:expr),*) => {{
        const TAIL: &str = CONCAT!($($rest),*);
        CONCAT!($a, TAIL)
    }};
    ($a:expr, $($rest:expr),*,) => {
        CONCAT!($a, $($rest),*)
    };
}

#[cfg(test)]
mod tests {
    #[test]
    fn top_level_constants() {
        const SALUTATION: &str = "Hello";
        const TARGET: &str = "world";
        const GREETING: &str = CONCAT!(SALUTATION, ", ", TARGET, "!");
        const GREETING_TRAILING_COMMA: &str = CONCAT!(SALUTATION, ", ", TARGET, "!",);

        assert_eq!(GREETING, "Hello, world!");
        assert_eq!(GREETING_TRAILING_COMMA, "Hello, world!");
    }
}
