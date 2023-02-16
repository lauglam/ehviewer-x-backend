use std::str::FromStr;
use visdom::Vis;
use crate::{parser::ParseError, structures::GalleryTagGroup};

impl ToString for GalleryTagGroup {
    fn to_string(&self) -> String {
        format!("{} ({})", self.tag_group_name, self.tag_vec.len())
    }
}

impl FromStr for GalleryTagGroup {
    type Err = ParseError;

    /// ```html
    /// <tr><td class="tc">parody:</td><td><div class="gtl" title="parody:senran kagura">senran kagura</div><div class="gtl" title="parody:the idolmaster">the idolmaster</div></td></tr>
    ///                    ^                                                             ^                                                                 ^
    ///                    tag_group_name                                                tag_vec[0]                                                        tag_vec[1]
    /// ```
    /// Or
    /// ```html
    /// <tr><td class="tc">parody:</td><td><div id="td_parody:the_idolmaster" class="gtl" style="opacity:1.0"><a id="ta_parody:the_idolmaster" href="https://e-hentai.org/tag/parody:the+idolmaster" class="" onclick="return toggle_tagmenu('parody:the idolmaster',this)">the idolmaster</a></div></td></tr>
    ///                    ^                                                                                                                                                                                                                                                ^
    ///                    tag_group_name                                                                                                                                                                                                                                   tag_vec[0]
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let tag_group_name = root.find(".tc").text();
        let tag_group_name = String::from(&tag_group_name[..tag_group_name.len() - 1]);

        // gt or gtl.
        let tag_vec = root.find("[class^=gt]").map(|_, ele| ele.text());

        Ok(GalleryTagGroup {
            tag_group_name,
            tag_vec,
        })
    }
}

// const PATTERN_TAG_GROUP: &str = r#"<tr><td[^<>]+>([\w\s]+):</td><td>(?:<div[^<>]+><a[^<>]+>[\w\s]+</a></div>)+</td></tr>"#;
// const PATTERN_TAG: &str = r#"<div[^<>]+><a[^<>]+>([\w\s]+)</a></div>"#;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_tag_group_test() {
        let ele = r#"
            <tr>
                <td class="tc">parody:</td>
                <td>
                    <div class="gtl" title="parody:senran kagura">senran kagura</div>
                    <div class="gtl" title="parody:the idolmaster">the idolmaster</div>
                </td>
            </tr>
        "#;

        let tag_group = ele.parse::<GalleryTagGroup>().unwrap();
        assert_eq!(tag_group.tag_vec, vec![r#"senran kagura"#, r#"the idolmaster"#]);
        assert_eq!(tag_group.tag_group_name, "parody");

        let ele = r#"
            <tr>
                <td class="tc">language:</td>
                <td>
                    <div id="td_language:chinese" class="gt" style="opacity:1.0"><a id="ta_language:chinese"
                            href="https://e-hentai.org/tag/language:chinese" class=""
                            onclick="return toggle_tagmenu('language:chinese',this)">chinese</a></div>
                    <div id="td_language:translated" class="gt" style="opacity:1.0"><a id="ta_language:translated"
                            href="https://e-hentai.org/tag/language:translated" class=""
                            onclick="return toggle_tagmenu('language:translated',this)">translated</a></div>
                </td>
            </tr>
        "#;

        let tag_group = ele.parse::<GalleryTagGroup>().unwrap();
        assert_eq!(tag_group.tag_vec, vec!["chinese", "translated"]);
        assert_eq!(tag_group.tag_group_name, "language");
    }
}
