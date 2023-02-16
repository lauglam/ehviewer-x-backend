use std::str::FromStr;
use regex::Regex;
use visdom::types::Elements;
use visdom::Vis;
use crate::{parser::{ATTRIBUTE_NOT_FOUND, EhParseResult, ParseError, REGEX_MATCH_FAILED},
            structures::{Category, FavoriteSlot, GalleryDetailUrl, GalleryInfoCompact,
                         GalleryInfoExtended, GalleryInfoMinimal, GalleryInfoMinimalPlus,
                         GalleryInfoSet, GalleryInfoThumbnail, GalleryList, Rating,
                         SearchNav, Thumb},
};

impl FromStr for GalleryList {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let nav = root.find(r#".searchnav"#).eq(0);
        let search_nav = nav.outer_html().parse::<SearchNav>()?;

        let selector = r#".searchnav select[onchange*=inline_set] > option[selected]"#;
        let selected = root.find(selector).last();

        let itg = root.find(".itg");
        let gallery_info_set = match selected.text().as_str() {
            "Minimal" => {
                let mut vec = Vec::new();
                let mut children = itg.children("tr").slice(1..);
                // if it is not log in, that 14th element of the array is an advertisement
                if children.length() > 25 {
                    children = children.slice(..13).add(children.slice(14..));
                }

                assert_eq!(children.length(), 25);
                for child in children {
                    vec.push(child.outer_html().parse::<GalleryInfoMinimal>()?);
                }
                GalleryInfoSet::Minimal(vec)
            }
            "Minimal+" => {
                let mut vec = Vec::new();
                let mut children = itg.children("tr").slice(1..);
                // if it is not log in, that 14th element of the array is an advertisement
                if children.length() > 25 {
                    children = children.slice(..13).add(children.slice(14..));
                }

                assert_eq!(children.length(), 25);
                for child in children {
                    vec.push(child.outer_html().parse::<GalleryInfoMinimalPlus>()?);
                }
                GalleryInfoSet::MinimalPlus(vec)
            }
            "Compact" => {
                let mut vec = Vec::new();
                let mut children = itg.children("tr").slice(1..);
                // if it is not log in, that 14th element of the array is an advertisement
                if children.length() > 25 {
                    children = children.slice(..13).add(children.slice(14..));
                }

                assert_eq!(children.length(), 25);
                for child in children {
                    vec.push(child.outer_html().parse::<GalleryInfoCompact>()?);
                }
                GalleryInfoSet::Compact(vec)
            }
            "Extended" => {
                let mut vec = Vec::new();
                let mut children = itg.children("tr");
                // if it is not log in, that 14th element of the array is an advertisement
                if children.length() > 25 {
                    children = children.slice(..13).add(children.slice(14..));
                }

                assert_eq!(children.length(), 25);
                for child in children {
                    vec.push(child.outer_html().parse::<GalleryInfoExtended>()?);
                }
                GalleryInfoSet::Extended(vec)
            }
            "Thumbnail" => {
                let mut vec = Vec::new();
                let children = itg.children(".gl1t");

                assert_eq!(children.length(), 25);
                for child in children {
                    vec.push(child.outer_html().parse::<GalleryInfoThumbnail>()?);
                }
                GalleryInfoSet::Thumbnail(vec)
            }
            _ => unreachable!(),
        };

        Ok(GalleryList { search_nav, gallery_info_set })
    }
}

impl FromStr for GalleryInfoMinimal {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let (gid, token) = parse_gid_and_token_1_2_3_5(&root)?;
        let category = parse_category_1_2_5(&root)?;
        let pages = parse_pages_1_2_3(&root)?;
        let thumb = parse_thumb_1_2_3(&root)?;
        let rating = parse_rating_1_2_3(&root)?;
        let posted = parse_posted(&root)?;
        let is_favorited = parse_is_favorited(&root)?;
        let favorite_slot_opt = parse_favorite_slot_opt(&root)?;
        let favorite_name_opt = parse_favorite_name_opt(&root)?;
        let uploader = parse_uploader_1_2_3_4(&root)?;
        let title = parse_title(&root)?;
        let simple_language_opt = parse_simple_language_opt_1_2_5(&root)?;

        Ok(GalleryInfoMinimal {
            gid,
            token,
            title,
            thumb,
            category,
            posted,
            uploader,
            rating,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
}

impl FromStr for GalleryInfoMinimalPlus {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let minimal = s.parse::<GalleryInfoMinimal>()?;
        Ok(unsafe { std::mem::transmute(minimal) })
    }
}

impl FromStr for GalleryInfoCompact {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let (gid, token) = parse_gid_and_token_1_2_3_5(&root)?;
        let category = parse_category_3_4(&root)?;
        let pages = parse_pages_1_2_3(&root)?;
        let thumb = parse_thumb_1_2_3(&root)?;
        let rating = parse_rating_1_2_3(&root)?;
        let posted = parse_posted(&root)?;
        let is_favorited = parse_is_favorited(&root)?;
        let favorite_slot_opt = parse_favorite_slot_opt(&root)?;
        let favorite_name_opt = parse_favorite_name_opt(&root)?;
        let uploader = parse_uploader_1_2_3_4(&root)?;
        let title = parse_title(&root)?;
        let simple_language_opt = parse_simple_language_opt_3_4(&root)?;
        let simple_tag_vec = parse_simple_tag_vec_3_4(&root)?;

        Ok(GalleryInfoCompact {
            gid,
            token,
            title,
            thumb,
            category,
            posted,
            uploader,
            rating,
            simple_tag_vec,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
}

impl FromStr for GalleryInfoExtended {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let (gid, token) = parse_gid_and_token_4(&root)?;
        let category = parse_category_3_4(&root)?;
        let pages = parse_pages_4(&root)?;
        let thumb = parse_thumb_4_5(&root)?;
        let rating = parse_rating_4_5(&root)?;
        let posted = parse_posted(&root)?;
        let is_favorited = parse_is_favorited(&root)?;
        let favorite_slot_opt = parse_favorite_slot_opt(&root)?;
        let favorite_name_opt = parse_favorite_name_opt(&root)?;
        let uploader = parse_uploader_1_2_3_4(&root)?;
        let title = parse_title(&root)?;
        let simple_language_opt = parse_simple_language_opt_3_4(&root)?;
        let simple_tag_vec = parse_simple_tag_vec_3_4(&root)?;

        Ok(GalleryInfoExtended {
            gid,
            token,
            title,
            thumb,
            category,
            posted,
            uploader,
            rating,
            simple_tag_vec,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
}

impl FromStr for GalleryInfoThumbnail {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;
        let (gid, token) = parse_gid_and_token_1_2_3_5(&root)?;
        let category = parse_category_1_2_5(&root)?;
        let pages = parse_pages_5(&root)?;
        let thumb = parse_thumb_4_5(&root)?;
        let rating = parse_rating_4_5(&root)?;
        let posted = parse_posted(&root)?;
        let is_favorited = parse_is_favorited(&root)?;
        let favorite_slot_opt = parse_favorite_slot_opt(&root)?;
        let favorite_name_opt = parse_favorite_name_opt(&root)?;
        let title = parse_title(&root)?;
        let simple_language_opt = parse_simple_language_opt_1_2_5(&root)?;

        Ok(GalleryInfoThumbnail {
            gid,
            token,
            title,
            thumb,
            category,
            posted,
            rating,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
}

// 1. Minimal
// 2. MinimalPlus
// 3. Compact
// 4. Extended
// 5. Thumbnail

fn parse_gid_and_token_1_2_3_5(root: &Elements) -> EhParseResult<(u64, String)> {
    let a = root.find(r#".glname a"#);
    let href = a.attr("href").ok_or(ATTRIBUTE_NOT_FOUND)?;
    let detail_url = href.to_string().parse::<GalleryDetailUrl>()?;
    Ok((detail_url.gid, detail_url.token))
}

fn parse_gid_and_token_4(root: &Elements) -> EhParseResult<(u64, String)> {
    let gl_name = root.find(r#".glname"#);
    let a = gl_name.parent("a");
    let href = a.attr("href").ok_or(ATTRIBUTE_NOT_FOUND)?;
    let detail_url = href.to_string().parse::<GalleryDetailUrl>()?;
    Ok((detail_url.gid, detail_url.token))
}

fn parse_simple_tag_vec_3_4(root: &Elements) -> EhParseResult<Vec<String>> {
    let gts = root.find(r#".glname .gt"#);
    let mut simple_tag_vec = Vec::new();
    for gt in gts {
        let title_attr = gt.get_attribute("title").ok_or(ATTRIBUTE_NOT_FOUND)?;
        simple_tag_vec.push(title_attr.to_string());
    }

    Ok(simple_tag_vec)
}

fn parse_category_1_2_5(root: &Elements) -> EhParseResult<u32> {
    let cs = root.find(".gl1m > .cs");
    let category = cs.text().parse::<Category>()?;

    Ok(category.value)
}

fn parse_category_3_4(root: &Elements) -> EhParseResult<u32> {
    let cn = root.find(".cn");
    let category = cn.text().parse::<Category>()?;

    Ok(category.value)
}

fn parse_pages_1_2_3(root: &Elements) -> EhParseResult<u32> {
    let ir = root.find(r#".glthumb .ir"#);

    let sibling = ir.siblings("div");
    let sibling_str = sibling.text();

    let regex = Regex::new(PATTERN_PAGES).unwrap();
    let captures = regex.captures(&sibling_str).ok_or(REGEX_MATCH_FAILED)?;

    Ok(captures[1].parse::<u32>()?)
}

fn parse_pages_4(root: &Elements) -> EhParseResult<u32> {
    let ir = root.find(".ir");
    let sibling = ir.next("").next("").text();

    let regex = Regex::new(PATTERN_PAGES).unwrap();
    let captures = regex.captures(&sibling).ok_or(REGEX_MATCH_FAILED)?;

    Ok(captures[1].parse::<u32>()?)
}

fn parse_pages_5(root: &Elements) -> EhParseResult<u32> {
    let ir = root.find(".ir");
    let sibling = ir.next("").text();

    let regex = Regex::new(PATTERN_PAGES).unwrap();
    let captures = regex.captures(&sibling).ok_or(REGEX_MATCH_FAILED)?;

    Ok(captures[1].parse::<u32>()?)
}

fn parse_thumb_1_2_3(root: &Elements) -> EhParseResult<Thumb> {
    let img = root.find(r#".glthumb img"#);
    Ok(img.outer_html().parse::<Thumb>()?)
}

fn parse_thumb_4_5(root: &Elements) -> EhParseResult<Thumb> {
    let img = root.find("img");
    Ok(img.outer_html().parse::<Thumb>()?)
}

fn parse_rating_1_2_3(root: &Elements) -> EhParseResult<f32> {
    let ir = root.find(r#".glthumb .ir"#);
    let style = ir.attr("style").ok_or(ATTRIBUTE_NOT_FOUND)?;
    let rating = style.to_string().parse::<Rating>()?;

    Ok(rating.value)
}

fn parse_rating_4_5(root: &Elements) -> EhParseResult<f32> {
    let ir = root.find(".ir");
    let style = ir.attr("style").ok_or(ATTRIBUTE_NOT_FOUND)?;
    let rating = style.to_string().parse::<Rating>()?;

    Ok(rating.value)
}

fn parse_posted(root: &Elements) -> EhParseResult<String> {
    let posted = root.find("[id^=posted_]");
    Ok(posted.text())
}

fn parse_is_favorited(root: &Elements) -> EhParseResult<bool> {
    let posted = root.find("[id^=posted_]");
    Ok(posted.attr("style").is_some())
}


fn parse_favorite_slot_opt(root: &Elements) -> EhParseResult<Option<u32>> {
    let posted = root.find("[id^=posted_]");
    if let Some(slot) = posted.attr("style") {
        let slot = slot.to_string().parse::<FavoriteSlot>()?;
        Ok(Some(slot.value))
    } else {
        Ok(None)
    }
}

fn parse_favorite_name_opt(root: &Elements) -> EhParseResult<Option<String>> {
    let posted = root.find("[id^=posted_]");
    if let Some(title) = posted.attr("title") {
        Ok(Some(title.to_string()))
    } else {
        Ok(None)
    }
}

fn parse_uploader_1_2_3_4(root: &Elements) -> EhParseResult<String> {
    let prefix = r#""https://e-hentai.org/uploader/""#;
    let a = root.find(&format!("[href^={}]", prefix));
    Ok(a.text())
}

fn parse_title(root: &Elements) -> EhParseResult<String> {
    let link = root.find(r#".glink"#);
    Ok(link.text())
}

fn parse_simple_language_opt_1_2_5(root: &Elements) -> EhParseResult<Option<String>> {
    let link = root.find(r#".glink"#);
    let idx_opt = S_LANG_PATTERNS.iter().position(|pattern| {
        let regex = Regex::new(pattern).unwrap();
        regex.is_match(&link.text())
    });

    let mut simple_language_opt = None;
    if let Some(idx) = idx_opt {
        simple_language_opt = Some(String::from(S_LANGS[idx]));
    }

    Ok(simple_language_opt)
}

fn parse_simple_language_opt_3_4(root: &Elements) -> EhParseResult<Option<String>> {
    let mut simple_language_opt = None;
    let simple_tag_vec = parse_simple_tag_vec_3_4(root)?;

    for tag in simple_tag_vec {
        let idx_opt = S_LANG_TAGS.iter().position(|&t| t == tag);
        if let Some(idx) = idx_opt {
            simple_language_opt = Some(String::from(S_LANGS[idx]));
            break;
        }
    }
    Ok(simple_language_opt)
}

const PATTERN_PAGES: &str = r#"(\d+) page"#;

const S_LANGS: [&str; 14] = [
    "S_LANG_EN",
    "S_LANG_ZH",
    "S_LANG_ES",
    "S_LANG_KO",
    "S_LANG_RU",
    "S_LANG_FR",
    "S_LANG_PT",
    "S_LANG_TH",
    "S_LANG_DE",
    "S_LANG_IT",
    "S_LANG_VI",
    "S_LANG_PL",
    "S_LANG_HU",
    "S_LANG_NL",
];

const S_LANG_PATTERNS: [&str; 14] = [
    r#"[(\[]eng(?:lish)?[)\]]|英訳"#,
    // r#[(（\[]ch(?:inese)?[)）\]]|[汉漢]化|中[国國][语語]|中文|中国翻訳#,
    r#"[(\uFF08\[]ch(?:inese)?[)\uFF09\]]|[汉漢]化|中[国國][语語]|中文|中国翻訳"#,
    r#"[(\[]spanish[)\]]|[(\[]Español[)\]]|スペイン翻訳"#,
    r#"[(\[]korean?[)\]]|韓国翻訳"#,
    r#"[(\[]rus(?:sian)?[)\]]|ロシア翻訳"#,
    r#"[(\[]fr(?:ench)?[)\]]|フランス翻訳"#,
    r#"[(\[]portuguese|ポルトガル翻訳"#,
    r#"[(\[]thai(?: ภาษาไทย)?[)\]]|แปลไทย|タイ翻訳"#,
    r#"[(\[]german[)\]]|ドイツ翻訳"#,
    r#"[(\[]italiano?[)\]]|イタリア翻訳"#,
    r#"[(\[]vietnamese(?: Tiếng Việt)?[)\]]|ベトナム翻訳"#,
    r#"[(\[]polish[)\]]|ポーランド翻訳"#,
    r#"[(\[]hun(?:garian)?[)\]]|ハンガリー翻訳"#,
    r#"[(\[]dutch[)\]]|オランダ翻訳"#,
];

const S_LANG_TAGS: [&str; 14] = [
    "language:english",
    "language:chinese",
    "language:spanish",
    "language:korean",
    "language:russian",
    "language:french",
    "language:portuguese",
    "language:thai",
    "language:german",
    "language:italian",
    "language:vietnamese",
    "language:polish",
    "language:hungarian",
    "language:dutch",
];

// const PATTERN_THUMB_SIZE: &str = r#"height:(\d+)px;width:(\d+)px"#;

// const S_LANG_JA: &str = "JA";
// const S_LANG_EN: &str = "EN";
// const S_LANG_ZH: &str = "ZH";
// const S_LANG_NL: &str = "NL";
// const S_LANG_FR: &str = "FR";
// const S_LANG_DE: &str = "DE";
// const S_LANG_HU: &str = "HU";
// const S_LANG_IT: &str = "IT";
// const S_LANG_KO: &str = "KO";
// const S_LANG_PL: &str = "PL";
// const S_LANG_PT: &str = "PT";
// const S_LANG_RU: &str = "RU";
// const S_LANG_ES: &str = "ES";
// const S_LANG_TH: &str = "TH";
// const S_LANG_VI: &str = "VI";


#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_gallery_list_test() {
        let s = read_test_file("gallery_list_minimal.html");
        let result = s.parse::<GalleryList>();

        let s = read_test_file("gallery_list_minimal_plus.html");
        let result = s.parse::<GalleryList>();

        let s = read_test_file("gallery_list_compact.html");
        let result = s.parse::<GalleryList>();

        let s = read_test_file("gallery_list_extended.html");
        let result = s.parse::<GalleryList>();

        let s = read_test_file("gallery_list_thumbnail.html");
        let result = s.parse::<GalleryList>();
    }
}
