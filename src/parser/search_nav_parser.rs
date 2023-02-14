use std::str::FromStr;
use regex::Regex;
use visdom::Vis;
use crate::parser::{ATTRIBUTE_NOT_FOUND, ParseError, REGEX_MATCH_FAILED};
use crate::structures::SearchNav;

impl FromStr for SearchNav {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let u_prev = root.find("#uprev");
        let prev_opt = if let Some(href) = u_prev.attr("href") {
            let href = href.to_string();
            let regex = Regex::new(PATTERN_PREV_PAGE).unwrap();
            let captures = regex.captures(&href).ok_or(REGEX_MATCH_FAILED)?;
            Some(String::from(&captures[1]))
        } else {
            None
        };


        let u_next = root.find("#unext");
        let next_opt = if let Some(href) = u_next.attr("href") {
            let href = href.to_string();
            let regex = Regex::new(PATTERN_NEXT_PAGE).unwrap();
            let captures = regex.captures(&href).ok_or(REGEX_MATCH_FAILED)?;
            Some(String::from(&captures[1]))
        } else {
            None
        };

        let select = root.find("select[onchange]");
        let onchange = select.attr("onchange").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let onchange_str = onchange.to_string();

        let regex = Regex::new(PATTERN_JUMP_PAGE).unwrap();
        let jump_opt = if let Some(cap) = regex.captures(&onchange_str) {
            Some(String::from(&cap[1]))
        } else {
            None
        };

        let regex = Regex::new(PATTERN_SEEK_PAGE).unwrap();
        let seek_opt = if let Some(cap) = regex.captures(&onchange_str) {
            Some(String::from(&cap[1]))
        } else {
            None
        };

        Ok(SearchNav {
            prev_opt,
            next_opt,
            jump_opt,
            seek_opt,
        })
    }
}

const PATTERN_PREV_PAGE: &str = r#"prev=([\d-]+)"#;
const PATTERN_NEXT_PAGE: &str = r#"next=([\d-]+)"#;
const PATTERN_JUMP_PAGE: &str = r#"jump=(\w+)"#;
const PATTERN_SEEK_PAGE: &str = r#"seek=([\w-]+)"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ele = r#"
            <div class="searchnav">
                <div></div>
                <div><a id="ufirst" href="https://e-hentai.org/">&lt;&lt; First</a></div>
                <div><a id="uprev" href="https://e-hentai.org/?prev=2458771">&lt; Prev</a></div>
                <div id="ujumpbox" class="jumpbox"><a id="ujump" href="javascript:enable_jump_mode('u')">Jump/Seek</a></div>
                <div><a id="unext" href="https://e-hentai.org/?next=2458743">Next &gt;</a></div>
                <div><a id="ulast" href="https://e-hentai.org/?prev=1">Last &gt;&gt;</a></div>
                <div><select onchange="sument.location='https://e-hentai.org/?inline_set=dm_'+this.value+'&amp;prev=2458732'">
                        <option value="m" selected="selected">Minimal</option>
                        <option value="p">Minimal+</option>
                        <option value="l">Compact</option>
                        <option value="e">Extended</option>
                        <option value="t">Thumbnail</option>
                    </select></div>
            </div>
        "#;
        assert_eq!(ele.parse::<SearchNav>().is_ok(), true);

        let jump_ele = r#"
            <div class="searchnav">
                <div></div>
                <div><a id="ufirst" href="https://e-hentai.org/">&lt;&lt; First</a></div>
                <div><a id="uprev" href="https://e-hentai.org/?prev=2458732">&lt; Prev</a></div>
                <div id="ujumpbox" class="jumpbox"><a id="ujump" href="javascript:enable_jump_mode('u')">Jump/Seek</a></div>
                <div><a id="unext" href="https://e-hentai.org/?next=2458691">Next &gt;</a></div>
                <div><a id="ulast" href="https://e-hentai.org/?prev=1">Last &gt;&gt;</a></div>
                <div><select
                        onchange="sument.location='https://e-hentai.org/?inline_set=dm_'+this.value+'&amp;next=2458743&amp;jump=1d'">
                        <option value="m" selected="selected">Minimal</option>
                        <option value="p">Minimal+</option>
                        <option value="l">Compact</option>
                        <option value="e">Extended</option>
                        <option value="t">Thumbnail</option>
                    </select></div>
            </div>
        "#;
        let result = jump_ele.parse::<SearchNav>();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().jump_opt.is_some(), true);

        let seek_ele = r#"
            <div class="searchnav">
                <div></div>
                <div><a id="ufirst" href="https://e-hentai.org/">&lt;&lt; First</a></div>
                <div><a id="uprev" href="https://e-hentai.org/?prev=2458732">&lt; Prev</a></div>
                <div id="ujumpbox" class="jumpbox"><a id="ujump" href="javascript:enable_jump_mode('u')">Jump/Seek</a></div>
                <div><a id="unext" href="https://e-hentai.org/?next=2458691">Next &gt;</a></div>
                <div><a id="ulast" href="https://e-hentai.org/?prev=1">Last &gt;&gt;</a></div>
                <div><select
                        onchange="sument.location='https://e-hentai.org/?inline_set=dm_'+this.value+'&amp;next=2464570&amp;seek=2023-02-06'">
                        <option value="m" selected="selected">Minimal</option>
                        <option value="p">Minimal+</option>
                        <option value="l">Compact</option>
                        <option value="e">Extended</option>
                        <option value="t">Thumbnail</option>
                    </select></div>
            </div>
        "#;
        let result = seek_ele.parse::<SearchNav>();
        assert_eq!(result.is_ok(), true);
        assert_eq!(result.unwrap().seek_opt.is_some(), true);
    }
}
