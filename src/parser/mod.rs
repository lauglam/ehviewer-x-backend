mod input;
mod unescape;
mod archive_parser;
mod gallery_detail_parser;
mod event_pane_parser;
mod favorite_slot_parser;
mod favorites_parser;
mod forums_parser;
mod gallery_list_parser;
mod gallery_info_parser;
mod gallery_api_parser;
mod gallery_identity_parser;
mod gallery_multi_page_viewer_p_token_parser;
mod gallery_not_available_parser;
mod gallery_page_api_parser;
mod gallery_page_parser;
mod gallery_page_url_parser;
mod gallery_tag_group_parser;
mod gallery_tag_group_list_parser;
mod gallery_token_api_parser;
mod profile_parser;
mod rate_gallery_parser;
mod rating_parser;
mod search_nav_parser;
mod sign_in_parser;
mod thumb_parser;
mod torrent_parser;
mod vote_comment_parser;
mod vote_tag_parser;
mod category_parser;

// result

pub type EhParseResult<T> = Result<T, ParseError>;

pub trait Parser: Sized {
    fn parse(s: &str) -> EhParseResult<Self>;
}

// error

pub type BoxDynError = Box<dyn std::error::Error + Send + Sync + 'static>;

#[derive(Debug)]
pub enum ParseError {
    RegexMatchFailed,
    OutOfRange,
    SignInRequired,
    AttributeNotFound,
    DomNotFound,
    FromServer(String),
    Other(BoxDynError),
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ParseError::RegexMatchFailed => write!(f, "regular expression matching failed"),
            ParseError::OutOfRange => write!(f, "input is out of range"),
            ParseError::SignInRequired => write!(f, "this page requires you to log on"),
            ParseError::AttributeNotFound => write!(f, "attribute cannot be found"),
            ParseError::DomNotFound => write!(f, "dom cannot be found"),
            ParseError::FromServer(s) => write!(f, "error from server: {}", s),
            ParseError::Other(e) => e.fmt(f),
        }
    }
}

impl std::error::Error for ParseError {}

const REGEX_MATCH_FAILED: ParseError = ParseError::RegexMatchFailed;
const OUT_OF_RANGE: ParseError = ParseError::OutOfRange;
const SIGN_IN_REQUIRED: ParseError = ParseError::SignInRequired;
const ATTRIBUTE_NOT_FOUND: ParseError = ParseError::AttributeNotFound;
const DOM_NOT_FOUND: ParseError = ParseError::DomNotFound;

impl From<BoxDynError> for ParseError {
    fn from(value: BoxDynError) -> Self {
        ParseError::Other(value)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(value: std::num::ParseIntError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(value: std::num::ParseFloatError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<chrono::ParseError> for ParseError {
    fn from(value: chrono::ParseError) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<String> for ParseError {
    fn from(value: String) -> Self {
        ParseError::Other(value.into())
    }
}

impl From<serde_json::Error> for ParseError {
    fn from(value: serde_json::Error) -> Self {
        ParseError::Other(value.into())
    }
}
