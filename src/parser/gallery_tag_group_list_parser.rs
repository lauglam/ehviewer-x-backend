use std::str::FromStr;
use visdom::Vis;
use crate::{parser::ParseError, structures::{GalleryTagGroup, GalleryTagGroupList}};

impl FromStr for GalleryTagGroupList {
    type Err = ParseError;

    /// ```html
    /// <div id="taglist">
    ///     <table>
    ///         <tbody>
    ///             <tr>
    ///                 <td class="tc">language:</td>
    ///                 <td>
    ///                     <div id="td_language:chinese" class="gt" style="opacity:1.0"><a id="ta_language:chinese"
    ///                             href="https://e-hentai.org/tag/language:chinese" class=""
    ///                             onclick="return toggle_tagmenu('language:chinese',this)">chinese</a></div>
    ///                     <div id="td_language:translated" class="gt" style="opacity:1.0"><a id="ta_language:translated"
    ///                             href="https://e-hentai.org/tag/language:translated" class=""
    ///                             onclick="return toggle_tagmenu('language:translated',this)">translated</a></div>
    ///                 </td>
    ///             </tr>
    ///             ...
    ///         </tbody>
    ///     </table>
    /// </div>
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;

        let mut group_vec = Vec::new();
        let trs = root.find("tr");
        for tr in trs {
            let group = tr.outer_html().parse::<GalleryTagGroup>()?;
            group_vec.push(group);
        }

        Ok(GalleryTagGroupList { group_vec })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_test() {
        let ele = r#"
            <div id="taglist">
                <table>
                    <tbody>
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
                        <tr>
                            <td class="tc">parody:</td>
                            <td>
                                <div id="td_parody:dragon_quest_vi" class="gt" style="opacity:1.0"><a id="ta_parody:dragon_quest_vi"
                                        href="https://e-hentai.org/tag/parody:dragon+quest+vi" class=""
                                        onclick="return toggle_tagmenu('parody:dragon quest vi',this)">dragon quest vi</a></div>
                                <div id="td_parody:dragon_quest_vii" class="gtw" style="opacity:1.0"><a
                                        id="ta_parody:dragon_quest_vii" href="https://e-hentai.org/tag/parody:dragon+quest+vii"
                                        class="" onclick="return toggle_tagmenu('parody:dragon quest vii',this)">dragon quest
                                        vii</a></div>
                            </td>
                        </tr>
                        <tr>
                            <td class="tc">character:</td>
                            <td>
                                <div id="td_character:maribel" class="gt" style="opacity:1.0"><a id="ta_character:maribel"
                                        href="https://e-hentai.org/tag/character:maribel" class=""
                                        onclick="return toggle_tagmenu('character:maribel',this)">maribel</a></div>
                            </td>
                        </tr>
                    </tbody>
                </table>
            </div>
        "#;

        assert_eq!(ele.parse::<GalleryTagGroupList>().is_ok(), true);
    }
}
