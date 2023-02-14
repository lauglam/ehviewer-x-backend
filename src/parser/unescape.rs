use std::borrow::Cow;

#[inline]
pub fn unescape(str: &str) -> Cow<str> {
    let (escape_vec, unescape_vec) = (ESCAPE_CHARACTER_LIST, UNESCAPE_CHARACTER_LIST);

    let mut result = Cow::Borrowed(str);
    for idx in 0..escape_vec.len() {
        let (escape, unescape) = (escape_vec[idx], unescape_vec[idx]);
        if str.contains(escape) {
            result = Cow::Owned(result.replace(escape, unescape));
        }
    }

    result
}

const ESCAPE_CHARACTER_LIST: [&str; 7] = ["&amp;", "&lt;", "&gt;", "&quot;", "&#039;", "&times;", "&nbsp;"];
const UNESCAPE_CHARACTER_LIST: [&str; 7] = ["&", "<", ">", "\"", "'", "×", " "];

#[cfg(test)]
mod tests {
    use super::*;
    use rand::Rng;

    #[test]
    fn unescape_test() {
        for _ in 0..10 {
            let mut rng = rand::thread_rng();
            let start = rng.gen_range(0..6);
            let end = rng.gen_range(start..7);

            let input = &ESCAPE_CHARACTER_LIST[start..end].join("");

            assert_eq!(&unescape(input), &UNESCAPE_CHARACTER_LIST[start..end].join(""));
        }
    }
}
