use std::str::FromStr;
use regex::Regex;
use visdom::Vis;
use crate::{
    parser::{ATTRIBUTE_NOT_FOUND, ParseError, REGEX_MATCH_FAILED},
    structures::{Category, FavoriteSlot, GalleryIdentity, GalleryInfo, Rating, Thumb},
};

impl FromStr for GalleryInfo {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;

        // 1. identity
        let e_gl_name = root.find(r#".glname"#);
        let mut e_a = e_gl_name.find("a");
        if e_a.is_empty() {
            // Extended
            e_a = e_gl_name.parent("a");
        }

        let href = e_a.attr("href").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let identity = href.to_string().parse::<GalleryIdentity>()?;

        // 2. simple_tag_vec_opt
        let e_gt_vec = root.find(r#".glname .gt"#);
        let simple_tag_vec_opt = if e_gt_vec.is_empty() {
            // Minimal MinimalPlus Thumbnail
            None
        } else {
            let mut simple_tag_vec = Vec::new();
            for e_gt in e_gt_vec {
                let title_attr = e_gt.get_attribute("title").ok_or(ATTRIBUTE_NOT_FOUND)?;
                simple_tag_vec.push(title_attr.to_string());
            }

            Some(simple_tag_vec)
        };

        // 3. category
        let mut e_cs_or_cn = root.find(".gl1m > .cs");
        if e_cs_or_cn.is_empty() {
            // Compact Extended
            e_cs_or_cn = root.find(".cn");
        }

        let category = e_cs_or_cn.text().parse::<Category>()?;
        let category = category.value;

        // 4. pages
        // Tips: Minimal MinimalPlus Compact in `.glthumb div:contains('pages')`
        let e_div = root.find(r#"div:contains('pages')"#);
        let pages = e_div.text();

        let regex = Regex::new(PATTERN_PAGES).unwrap();
        let captures = regex.captures(&pages).ok_or(REGEX_MATCH_FAILED)?;

        let pages = captures[1].parse::<u32>()?;

        // 5. thumb
        // Tips: Minimal MinimalPlus Compact in `.glthumb img`
        let e_img = root.find("img");
        let thumb = e_img.outer_html().parse::<Thumb>()?;

        // 6. rating
        // Tips: Minimal MinimalPlus Compact in `.glthumb .ir`
        let e_ir = root.find(r#".ir"#).eq(0);
        let style = e_ir.attr("style").ok_or(ATTRIBUTE_NOT_FOUND)?;

        let rating = style.to_string().parse::<Rating>()?;
        let rating = rating.value;

        // 7. posted
        let e_div = root.find("[id^=posted_]");
        let posted = e_div.text();

        // 8. is_favorited
        let is_favorited = e_div.attr("style").is_some();

        // 9. favorite_slot_opt
        let favorite_slot_opt = if let Some(style) = e_div.attr("style") {
            let favorite_slot = style.to_string().parse::<FavoriteSlot>()?;
            Some(favorite_slot.value)
        } else {
            None
        };

        // 10. favorite_name_opt
        let favorite_name_opt = if let Some(title) = e_div.attr("title") {
            Some(title.to_string())
        } else {
            None
        };

        // 11. uploader_opt
        let prefix = r#""https://e-hentai.org/uploader/""#;
        let e_a = root.find(&format!("[href^={}]", prefix));

        let uploader_opt = if e_a.is_empty() {
            None
        } else {
            Some(e_a.text())
        };

        // 12. title
        let e_div = root.find(r#".glink"#);
        let title = e_div.text();

        // 13. simple_language_opt
        let simple_language_opt = if let Some(ref simple_tag_vec) = simple_tag_vec_opt {
            let mut simple_language_opt = None;
            for tag in simple_tag_vec {
                let idx_opt = S_LANG_TAGS.iter().position(|&t| t == tag);
                if let Some(idx) = idx_opt {
                    simple_language_opt = Some(String::from(S_LANGS[idx]));
                    break;
                }
            }
            simple_language_opt
        } else {
            let link = root.find(r#".glink"#);
            let idx_opt = S_LANG_PATTERNS.iter().position(|pattern| {
                let regex = Regex::new(pattern).unwrap();
                regex.is_match(&link.text())
            });

            let mut simple_language_opt = None;
            if let Some(idx) = idx_opt {
                simple_language_opt = Some(String::from(S_LANGS[idx]));
            }

            simple_language_opt
        };

        Ok(GalleryInfo {
            identity,
            title,
            thumb,
            category,
            posted,
            uploader_opt,
            simple_tag_vec_opt,
            rating,
            pages,
            simple_language_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
        })
    }
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
