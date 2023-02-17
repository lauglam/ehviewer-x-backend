use std::collections::HashMap;
use std::iter::zip;
use std::str::FromStr;
use chrono::DateTime;
use regex::Regex;
use visdom::Vis;
use crate::{
    parser::{ATTRIBUTE_NOT_FOUND, DOM_NOT_FOUND, ParseError, REGEX_MATCH_FAILED, unescape::unescape},
    structures::{Category, FavoriteSlot, GalleryComment, GalleryCommentList, GalleryDetail,
                 GalleryDetailDetail, GalleryIdentity, GalleryPreviewLarge, GalleryPreviewMedium,
                 GalleryPreviewSet, GalleryTagGroupList},
};

impl FromStr for GalleryDetail {
    type Err = ParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.contains(OFFENSIVE_STRING) {
            return Err(ParseError::FromServer(String::from("if you choose to ignore this warning, you lose all rights to complain about it in the future.")));
        }

        if s.contains(PINING_STRING) {
            return Err(ParseError::FromServer(String::from("this gallery_list is pining for the fjords.")));
        }

        let regex = Regex::new(PATTERN_ERROR).unwrap();
        if let Some(cap) = regex.captures(s) {
            return Err(ParseError::FromServer(String::from(&cap[1])));
        }

        let regex = Regex::new(PATTERN_DETAIL).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let gid = captures[1].parse::<u64>()?;
        let api_uid = captures[5].parse::<u64>()?;
        let token = String::from(&captures[3]);
        let api_key = String::from(&captures[7]);

        let regex = Regex::new(PATTERN_TORRENT).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let torrent_url = String::from(unescape(&captures[1]));
        let torrent_count = captures[2].parse::<u32>()?;

        let regex = Regex::new(PATTERN_ARCHIVE).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;
        let archive_url = String::from(unescape(&captures[1]));

        let root = Vis::load(s)?;
        let gm = root.find(".gm:not(#cdiv)");

        let cover = gm.find("#gd1 div:first-child");
        let style = cover.attr("style").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let style = style.to_string();
        let regex = Regex::new(PATTERN_COVER).unwrap();
        let captures = regex.captures(&style).ok_or(REGEX_MATCH_FAILED)?;
        let thumb = String::from(&captures[3]);

        let gn = gm.find("#gn");
        let title = gn.text();

        let gj = gm.find("#gj");
        let title_jpn = gj.text();

        let cs = gm.find("#gdc > .cs");
        let cs = cs.text();
        let category = cs.parse::<Category>()?.value;

        let gdn = gm.find("#gdn");
        let uploader = gdn.text();

        let gdd = gm.find("#gdd");
        let detail = gdd.html().parse::<GalleryDetailDetail>()?;

        let rat = gm.find("#rating_count");
        let rating_count = rat.text().parse::<u32>()?;

        let label = gm.find("#rating_label");
        let label_text = label.text();
        let mut rating_opt: Option<f32> = None;
        if label_text != "Not Yet Rated" {
            let regex = Regex::new(PATTERN_RATING).unwrap();
            let captures = regex.captures(&label_text).ok_or(REGEX_MATCH_FAILED)?;
            rating_opt = Some(captures[1].parse::<f32>()?);
        }

        let gdf = gm.find("#gdf");
        let favorite_link = gdf.find("#favoritelink");
        let is_favorited = !favorite_link.text().contains("Add to Favorites");

        let (favorite_slot_opt, favorite_name_opt) = if is_favorited {
            let i = gdf.find(".i");
            let style = i.attr("style").ok_or(ATTRIBUTE_NOT_FOUND)?;
            let favorite_slot = style.to_string().parse::<FavoriteSlot>()?;

            (Some(favorite_slot.value), Some(favorite_link.text()))
        } else {
            (None, None)
        };

        let gnd = root.find("#gnd");
        let newer_version_map_opt = if !gnd.is_empty() {
            let regex = Regex::new(PATTERN_NEWER_DATE).unwrap();
            let date_vec = regex.captures_iter(s)
                .map(|cap| String::from(&cap[1]))
                .collect::<Vec<String>>();

            let mut newer_version_map = HashMap::new();
            let hrefs = gnd.find("a");
            for (idx, href) in hrefs.into_iter().enumerate() {
                let href = href.get_attribute("href").ok_or(ATTRIBUTE_NOT_FOUND)?;
                let identity = href.to_string().parse::<GalleryIdentity>()?;
                newer_version_map.insert(date_vec[idx].clone(), identity);
            }
            Some(newer_version_map)
        } else {
            None
        };

        let c_div = root.find("#cdiv");
        let comment_list = c_div.outer_html().parse::<GalleryCommentList>()?;

        let last_page = root.find(".ptt td:nth-last-child(2) > a");
        let preview_pages = last_page.text().parse::<u32>()?;

        let first_page = root.find(".ptt td:nth-child(2) > a");
        let href = first_page.attr("href").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let url = href.to_string();

        let gdo4 = root.find("#gdo4");
        let can_click = gdo4.children("[onclick]");

        let gdt = root.find("#gdt");
        let preview_set = match can_click.text().as_str() {
            "Large" => GalleryPreviewSet::Large(parse_large(&gdt.outer_html())?),
            "Normal" => GalleryPreviewSet::Medium(parse_medium(&gdt.outer_html())?),
            _ => unreachable!(),
        };

        let tag_list = root.find("#taglist");
        let tag_group_list = tag_list.outer_html().parse::<GalleryTagGroupList>()?;
        let tag_group_vec = tag_group_list.group_vec;

        Ok(GalleryDetail {
            gid,
            token,
            api_uid,
            api_key,
            torrent_count,
            torrent_url,
            archive_url,
            thumb,
            newer_version_map_opt,
            is_favorited,
            favorite_slot_opt,
            favorite_name_opt,
            rating_count,
            tag_group_vec,
            comment_list,
            preview_pages,
            preview_set,
            url,
            title,
            title_jpn,
            category,
            uploader,
            rating_opt,
            detail,
        })
    }
}

impl FromStr for GalleryComment {
    type Err = ParseError;

    /// 1. Uploader Comment
    /// ```html
    /// <!-- uploader comment -->
    /// <a name="c0"></a>
    /// <div class="c1">
    ///     <div class="c2">
    ///         <div class="c3">Posted on 02 July 2019, 11:50 by: &nbsp; <a
    ///                 href="https:///e-hentai.org/uploader/qq3870990">qq3870990</a>&nbsp; &nbsp; <a
    ///                 href="https:///forums.e-hentai.org/index.php?showuser=1725168"><img class="ygm"
    ///                     src="https:///ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
    ///         <div class="c4 nosel"><a name="ulcomment"></a>Uploader Comment</div>
    ///         <div class="c"></div>
    ///     </div>
    ///     <div class="c6" id="comment_0">
    ///         =========================================================<br>不咕鸟欢迎各位甲方大佬委托汉化本子<br>感谢淘宝“涩谷成人玩具”对本组的大力赞助，有意向的可以去店内逛逛，多多关注。<br>备注咕咕咕有优惠<br><br>详情请联系：2820261867<br>特别注明：<br><br>禁止删除水印封面进行转载，禁止不带汉化组名进行转载，尤其是哔咔，再发现类似情况，外流版本将所有页全部打上水印，无水印版本只提供给金主。<br><br>=======================================================<br><br>RAW：<a
    ///             href="https:///e-hentai.org/g/1378957/7f626bf1d2/">https:///e-hentai.org/g/1378957/7f626bf1d2/</a></div>
    ///     <div class="c7" id="cvotes_0" style="display:none"></div>
    /// </div>
    /// ```
    /// Or
    /// 2. Others Comment
    /// ```html
    /// <a name="c3922745"></a>
    /// <div class="c1">
    ///     <div class="c2">
    ///         <div class="c3">Posted on 24 September 2020, 09:55 by: &nbsp; <a
    ///                 href="https://e-hentai.org/uploader/Kalinkawow">Kalinkawow</a>&nbsp; &nbsp; <a
    ///                 href="https://forums.e-hentai.org/index.php?showuser=4997064"><img class="ygm"
    ///                     src="https://ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
    ///         <div class="c4 nosel">[<a id="comment_vote_up_3922745" style="" href="#"
    ///                 onclick="vote_comment_up(3922745); this.blur(); return false">Vote+</a>] &nbsp; [<a
    ///                 id="comment_vote_down_3922745" style="" href="#"
    ///                 onclick="vote_comment_down(3922745); this.blur(); return false">Vote-</a>]</div>
    ///         <div class="c5 nosel" onmouseover="sument.getElementById('cvotes_3922745').style.display=''"
    ///             onclick="this.onmouseover(); this.onmouseout=undefined"
    ///             onmouseout="sument.getElementById('cvotes_3922745').style.display='none'">Score <span
    ///                 id="comment_score_3922745" style="opacity:1.0">+257</span></div>
    ///         <div class="c"></div>
    ///     </div>
    ///     <div class="c6" id="comment_3922745">猎 妈 人</div>
    ///     <div class="c7" id="cvotes_3922745" style="display:none">Base +3, <span>q171718988 +3</span>, <span>Igarashi
    ///             Shioya +6</span>, <span>suhaotian +6</span>, <span>as390393473 +2</span>, <span>Subara45 +4</span>,
    ///         <span>louis friend +6</span>, <span>52wy1314 +6</span>, <span>随缘的亚子 +6</span>, <span>Tchami_zz +2</span>,
    ///         <span>sakkijarven +2</span>, <span>无证萝莉控 +6</span>, <span>DaweiX +4</span>, and 38 more...</div>
    /// </div>
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const PATTERN_COMMENT_ID: &str = r#"<a name="c(\d+)"></a>"#;
        const PATTERN_COMMENT_DATETIME: &str = r#"Posted\s*on\s*(.+?)\s*by"#;

        let root = Vis::load(s)?;

        let regex = Regex::new(PATTERN_COMMENT_ID).unwrap();
        let captures = regex.captures(s).ok_or(REGEX_MATCH_FAILED)?;

        // c0 is uploader comment. cannot vote.
        // id.
        let id = captures[1].parse::<u64>()?;

        let c3 = root.find(".c3");
        let posted = c3.text();

        // posted_timestamp.
        let regex = Regex::new(PATTERN_COMMENT_DATETIME).unwrap();
        let captures = regex.captures(&posted).ok_or(REGEX_MATCH_FAILED)?;

        let fmt = "%d %B %Y, %H:%M:%S%.3f %z";
        let date_str = format!("{}:00.000 +0000", &captures[1]);
        let datetime = DateTime::parse_from_str(&date_str, fmt)?;
        let posted_timestamp = datetime.timestamp();

        // user.
        let a = c3.children("a");
        let user = a.text();

        // comment.
        let c6 = root.find(".c6");
        let comment = c6.html();

        // last_edited_timestamp_opt.
        let c8 = root.find(".c8");
        let last_edited_timestamp_opt = if !c8.is_empty() { Some(posted_timestamp) } else { None };

        // is_uploader.
        let c4 = root.find(".c4");
        let is_uploader = c4.text() == "Uploader Comment";

        let (
            mut vote_up_able,
            mut vote_up_ed,
            mut vote_down_able,
            mut vote_down_ed,
            mut editable,
            mut vote_state_opt,
            mut score_opt,
        ) = (false, false, false, false, false, None, None);

        if !is_uploader {
            for a in c4.children("a") {
                let text = a.text();
                if let Some(style) = a.get_attribute("style") {
                    match text.as_str() {
                        "Vote+" => {
                            // vote_up_able, vote_up_ed.
                            vote_up_able = true;
                            vote_up_ed = !style.to_string().is_empty();
                        }
                        "Vote-" => {
                            // vote_down_able, vote_down_ed.
                            vote_down_able = true;
                            vote_down_ed = !style.to_string().is_empty();
                        }
                        _ => {}
                    }
                } else if text == "Edit" {
                    // editable.
                    editable = true;
                }
            }

            // vote_state_opt.
            let c7 = root.find(".c7");
            vote_state_opt = Some(c7.text());

            // score_opt.
            let span = root.find(&format!(r#".c5 #comment_score_{}"#, id));
            score_opt = Some(span.text()[1..].parse::<u32>()?);
        }


        Ok(GalleryComment {
            id,
            score_opt,
            editable,
            vote_up_able,
            vote_up_ed,
            vote_down_able,
            vote_down_ed,
            is_uploader,
            vote_state_opt,
            posted_timestamp,
            user,
            comment,
            last_edited_timestamp_opt,
        })
    }
}

impl FromStr for GalleryCommentList {
    type Err = ParseError;

    /// ```html
    /// <div id="cdiv" class="gm">
    ///     <!-- uploader comment -->
    ///     <a name="c0"></a>
    ///     <div class="c1">...</div>
    ///
    ///     <a name="c3054522"></a>
    ///     <div class="c1">...</div>
    ///
    ///     <div id="chd">
    ///         <p>There is 1 more comment below the viewing threshold - <a
    ///             href="https://e-hentai.org/g/1740161/b90e67b628/?hc=1#comments" rel="nofollow">click to show all</a>.
    ///         </p>
    ///         <p id="postnewcomment">[<a href="#"
    ///                 onclick="display_comment_field(); sument.getElementById('postnewcomment').style.display='none'; return false">Post
    ///                 New Comment</a>]</p>
    ///     </div>
    ///     <a name="cnew"></a>
    ///     <div id="formdiv" style="display:none">
    ///         <form method="post" action="#cnew">
    ///             <textarea name="commenttext_new"
    ///                 placeholder="Enter your comments here, then hit Post Comment. If the last comment posted is yours, this will be appended to that post."></textarea>
    ///             <p><input type="submit" value="Post Comment"></p>
    ///         </form>
    ///     </div>
    /// </div>
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;

        let mut comment_vec = Vec::new();
        let cas = root.find(r#"a[name^=c][name!=cnew]"#);
        let c1s = root.find(".c1");

        for (ca, c1) in zip(cas, c1s) {
            let combine = &format!("{}{}", ca.outer_html(), c1.outer_html());
            let comment = combine.parse::<GalleryComment>()?;
            comment_vec.push(comment);
        }

        let show_all = root.find("#chd [rel=nofollow]");
        let has_more = !show_all.is_empty();

        Ok(GalleryCommentList {
            comment_vec,
            has_more,
        })
    }
}

impl FromStr for GalleryDetailDetail {
    type Err = ParseError;

    /// <table>
    ///     <tr>
    ///         <td class="gdt1">Posted:</td>
    ///         <td class="gdt2">2023-02-07 07:33</td>
    ///     </tr>
    ///     ...
    /// </table>
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        const PATTERN_PAGES: &str = r#"(\d+) pages"#;
        const PATTERN_FAVORITE_COUNT: &str = r#"(\d+) times"#;

        let root = Vis::load(s)?;
        let gdt1s = root.find(".gdt1");

        let (
            mut posted,
            mut parent_opt,
            mut visible,
            mut language,
            mut file_size,
            mut pages,
            mut favorite_count
        ) = (None, None, None, None, None, None, None);

        for gdt1 in gdt1s {
            match gdt1.text().as_str() {
                "Posted:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    posted = Some(gdt2.text());
                }
                "Parent:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();

                    if let Some(href) = gdt2.get_attribute("href") {
                        parent_opt = Some(href.to_string());
                    }
                }
                "Visible:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    visible = Some(gdt2.text());
                }
                "Language:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    language = Some(gdt2.text());
                }
                "File Size:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    file_size = Some(gdt2.text());
                }
                "Length:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    let gdt2 = gdt2.text();

                    let regex = Regex::new(PATTERN_PAGES).unwrap();
                    let captures = regex.captures(&gdt2).unwrap();
                    pages = Some(captures[1].parse::<u32>()?);
                }
                "Favorited:" => {
                    let gdt2 = gdt1.next_element_sibling().unwrap();
                    let gdt2 = gdt2.text();

                    let regex = Regex::new(PATTERN_FAVORITE_COUNT).unwrap();
                    let captures = regex.captures(&gdt2).unwrap();
                    favorite_count = Some(captures[1].parse::<u32>()?);
                }
                _ => unreachable!()
            }
        }

        if let (
            Some(posted),
            Some(visible),
            Some(language),
            Some(file_size),
            Some(pages),
            Some(favorite_count)
        ) = (posted, visible, language, file_size, pages, favorite_count) {
            Ok(GalleryDetailDetail {
                posted,
                parent_opt,
                visible,
                language,
                file_size,
                pages,
                favorite_count,
            })
        } else {
            Err(DOM_NOT_FOUND)
        }
    }
}

impl FromStr for GalleryPreviewLarge {
    type Err = ParseError;

    /// ```html
    /// <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
    ///     title="Page 1: AnMMSC_2_001_1.png"
    ///     src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>     ///
    /// </div>
    /// ```
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let root = Vis::load(s)?;

        let a = root.find("a");
        let href = a.attr("href").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let page_url = href.to_string();

        let img = a.children("img");
        let src = img.attr("src").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let image_url = src.to_string();

        let title = img.attr("title").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let title = title.to_string();
        let regex = Regex::new(PATTERN_FILENAME).unwrap();
        let captures = regex.captures(&title).ok_or(REGEX_MATCH_FAILED)?;
        let filename = String::from(&captures[1]);

        let alt = img.attr("alt").ok_or(ATTRIBUTE_NOT_FOUND)?;
        let position = alt.to_string().parse::<u32>()? - 1;

        Ok(GalleryPreviewLarge {
            position,
            filename,
            page_url,
            image_url,
        })
    }
}

/// ```html
/// <div id="gdt">
///     <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
///                 title="Page 1: AnMMSC_2_001_1.png"
///                 src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>
///     </div>
///     <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02"
///                 title="Page 2: AnMMSC_2_001_2.jpg"
///                 src="https://ehgt.org/ad/7a/ad7a3b7014372ce64193c118b1cfcbcf7ea68ee0-2507700-2458-3497-jpg_l.jpg"></a>
///     </div>
///     ...
///     <div class="c"></div>
/// </div>
/// ```
fn parse_large(s: &str) -> Result<Vec<GalleryPreviewLarge>, ParseError> {
    let root = Vis::load(s)?;
    let mut preview_vec = Vec::new();
    let gdt_larges = root.children(r#".gdtl"#);
    for gdt_large in gdt_larges {
        preview_vec.push(gdt_large.outer_html().parse::<GalleryPreviewLarge>()?);
    }

    Ok(preview_vec)
}

// const PATTERN_PREVIEW_PAGES: &str = r#"<td[^>]+><a[^>]+>([\d,]+)</a></td><td[^>]+>(?:<a[^>]+>)?&gt;(?:</a>)?</td>"#;
// const PATTERN_LARGE_PREVIEW: &str = r#"<div class="gdtl".+?<a href="(.+?)"><img alt="([\d,]+)".+?src="(.+?)""#;

/// ```html
/// <div id="gdt">
///     <div class="gdtm" style="height:167px">
///         <div
///             style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -0px 0 no-repeat">
///             <a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01" title="Page 1: AnMMSC_2_001_1.png"
///                     src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
///         </div>
///     </div>
///     <div class="gdtm" style="height:167px">
///         <div
///             style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -100px 0 no-repeat">
///             <a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02" title="Page 2: AnMMSC_2_001_2.jpg"
///                     src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
///         </div>
///     </div>
///     ...
///     <div class="c"></div>
/// </div>
/// ```
fn parse_medium(s: &str) -> Result<Vec<GalleryPreviewMedium>, ParseError> {
    let mut preview_vec = Vec::new();

    let regex = Regex::new(PATTERN_MEDIUM_PREVIEW).unwrap();
    for cap in regex.captures_iter(s) {
        let clip_width = cap[1].parse::<u32>()?;
        let clip_height = cap[2].parse::<u32>()?;
        let image_url = String::from(&cap[3]);
        let offset_x = cap[4].parse::<u32>()?;
        let offset_y = 0;
        let page_url = String::from(&cap[5]);
        let position = cap[6].parse::<u32>()? - 1;
        let filename = String::from(&cap[7]);

        preview_vec.push(
            GalleryPreviewMedium {
                position,
                filename,
                page_url,
                image_url,
                offset_x,
                offset_y,
                clip_width,
                clip_height,
            }
        );
    }

    Ok(preview_vec)
}

const OFFENSIVE_STRING: &str = "<p>(And if you choose to ignore this warning, you lose all rights to complain about it in the future.)</p>";
const PINING_STRING: &str = "<p>This gallery_list is pining for the fjords.</p>";
const PATTERN_ERROR: &str = "<div class=\"d\">\n<p>([^<]+)</p>";
const PATTERN_DETAIL: &str = "var gid = (\\d+);\\s*?(\n|\r|\r\n)?\\s*?var token = \"([a-f0-9]+)\";\\s*?(\n|\r|\r\n)?\\s*?var apiuid = ([\\-\\d]+);\\s*?(\n|\r|\r\n)?\\s*?var apikey = \"([a-f0-9]+)\";";
const PATTERN_TORRENT: &str = r#"<a[^<>]*onclick="return popUp\('([^']+)'[^)]+\)">Torrent Download[^<]+(\d+)[^<]+</a"#;
const PATTERN_ARCHIVE: &str = r#"<a[^<>]*onclick="return popUp\('([^']+)'[^)]+\)">Archive Download</a>"#;
const PATTERN_RATING: &str = r#"[+-]?([0-9]*[.]?[0-9]+)"#;
const PATTERN_NEWER_DATE: &str = ", added (.+?)<br />";
const PATTERN_COVER: &str = r#"width:(\d+)px; height:(\d+)px.+?url\((.+?)\)"#;
const PATTERN_FILENAME: &str = r#"Page \d+: ([\w\s]+.[\w]+)"#;
// const PATTERN_PAGES: &str = r#"<tr><td[^<>]*>Length:</td><td[^<>]*>([\d,]+) pages</td></tr>"#;
const PATTERN_MEDIUM_PREVIEW: &str = r#"<div class="gdtm"[^<>]*><div[^<>]*width:(\d+)[^<>]*height:(\d+)[^<>]*\((.+?)\)[^<>]*-(\d+)px[^<>]*><a[^<>]*href="(.+?)"[^<>]*><img alt="([\d,]+)"[^<>]*title="Page \d+: ([\w\s]+.[\w]+)""#;

#[cfg(test)]
mod tests {
    use crate::test_helper::read_test_file;
    use super::*;

    #[test]
    fn parse_detail_test() {
        let s = read_test_file("gallery_detail.html");
        assert_eq!(s.parse::<GalleryDetail>().is_ok(), true);
    }

    #[test]
    fn parse_detail_detail_test() {
        let table = r#"
        <table>
            <tr>
                <td class="gdt1">Posted:</td>
                <td class="gdt2">2023-02-07 07:33</td>
            </tr>
            <tr>
                <td class="gdt1">Parent:</td>
                <td class="gdt2">None</td>
            </tr>
            <tr>
                <td class="gdt1">Visible:</td>
                <td class="gdt2">Yes</td>
            </tr>
            <tr>
                <td class="gdt1">Language:</td>
                <td class="gdt2">Japanese &nbsp;</td>
            </tr>
            <tr>
                <td class="gdt1">File Size:</td>
                <td class="gdt2">225.5 MB</td>
            </tr>
            <tr>
                <td class="gdt1">Length:</td>
                <td class="gdt2">75 pages</td>
            </tr>
            <tr>
                <td class="gdt1">Favorited:</td>
                <td class="gdt2" id="favcount">23 times</td>
            </tr>
        </table>
        "#;

        assert_eq!(table.parse::<GalleryDetailDetail>().is_ok(), true);
    }

    #[test]
    fn parse_comment_list_test() {
        let ele = r##"
            <div id="cdiv" class="gm">
                <a name="c0"></a>
                <div class="c1">
                    <div class="c2">
                        <div class="c3">Posted on 23 September 2020, 14:04 by: &nbsp; <a
                                href="https://e-hentai.org/uploader/qq3870990">qq3870990</a>&nbsp; &nbsp; <a
                                href="https://forums.e-hentai.org/index.php?showuser=1725168"><img class="ygm"
                                    src="https://ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
                        <div class="c4 nosel"><a name="ulcomment"></a>Uploader Comment</div>
                        <div class="c"></div>
                    </div>
                    <div class="c6" id="comment_0">RAW：<a
                            href="https://e-hentai.org/g/1511310/8e568fd1b0/">https://e-hentai.org/g/1511310/8e568fd1b0/</a><br><br>鸣谢金主
                        沈阳大街等你嗷
                        出资汉化<br><br>感谢淘宝“涩谷成人玩具”对本组的大力赞助，有意向的可以去店内逛逛，多多关注。<br>下单报口令“不咕鸟”享优惠<br><br>=========================================================<br><br>不咕鸟汉化组招新<br><br>招募如下位置：<br>日译：要求日语水平N1或者有大量汉化经验。（同时也招募韩译）<br><br>注意，有入组考试，我们会通过考试内容来评估翻译能力是否满足我们的需求<br>关于报酬，通过入组考试会由接待进行告知。<br><br>同时也欢迎个人或组织与我们展开汉化合作<br>以上皆联系接待qq：2820261867<br><br>=========================================================
                    </div>
                    <div class="c7" id="cvotes_0" style="display:none"></div>
                </div>

                <a name="c3922745"></a>
                <div class="c1">
                    <div class="c2">
                        <div class="c3">Posted on 24 September 2020, 09:55 by: &nbsp; <a
                                href="https://e-hentai.org/uploader/Kalinkawow">Kalinkawow</a>&nbsp; &nbsp; <a
                                href="https://forums.e-hentai.org/index.php?showuser=4997064"><img class="ygm"
                                    src="https://ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
                        <div class="c4 nosel">[<a id="comment_vote_up_3922745" style="" href="#"
                                onclick="vote_comment_up(3922745); this.blur(); return false">Vote+</a>] &nbsp; [<a
                                id="comment_vote_down_3922745" style="" href="#"
                                onclick="vote_comment_down(3922745); this.blur(); return false">Vote-</a>]</div>
                        <div class="c5 nosel" onmouseover="sument.getElementById('cvotes_3922745').style.display=''"
                            onclick="this.onmouseover(); this.onmouseout=undefined"
                            onmouseout="sument.getElementById('cvotes_3922745').style.display='none'">Score <span
                                id="comment_score_3922745" style="opacity:1.0">+257</span></div>
                        <div class="c"></div>
                    </div>
                    <div class="c6" id="comment_3922745">猎 妈 人</div>
                    <div class="c7" id="cvotes_3922745" style="display:none">Base +3, <span>q171718988 +3</span>, <span>Igarashi
                            Shioya +6</span>, <span>suhaotian +6</span>, <span>as390393473 +2</span>, <span>Subara45 +4</span>,
                        <span>louis friend +6</span>, <span>52wy1314 +6</span>, <span>随缘的亚子 +6</span>, <span>Tchami_zz +2</span>,
                        <span>sakkijarven +2</span>, <span>无证萝莉控 +6</span>, <span>DaweiX +4</span>, and 38 more...</div>
                </div>
                <div id="chd">
                    <p>There is 1 more comment below the viewing threshold - <a
                            href="https://e-hentai.org/g/1740161/b90e67b628/?hc=1#comments" rel="nofollow">click to show all</a>.
                    </p>
                    <p id="postnewcomment">[<a href="#"
                            onclick="display_comment_field(); sument.getElementById('postnewcomment').style.display='none'; return false">Post
                            New Comment</a>]</p>
                </div>
                <a name="cnew"></a>
                <div id="formdiv" style="display:none">
                    <form method="post" action="#cnew">
                        <textarea name="commenttext_new"
                            placeholder="Enter your comments here, then hit Post Comment. If the last comment posted is yours, this will be appended to that post."></textarea>
                        <p><input type="submit" value="Post Comment"></p>
                    </form>
                </div>
            </div>
        "##;

        assert_eq!(ele.parse::<GalleryCommentList>().is_ok(), true);
    }

    #[test]
    fn parse_comment_test() {
        // uploader.
        let ele = r#"
            <a name="c0"></a>
            <div class="c1">
                <div class="c2">
                    <div class="c3">Posted on 02 July 2019, 11:50 by: &nbsp; <a
                            href="https:///e-hentai.org/uploader/qq3870990">qq3870990</a>&nbsp; &nbsp; <a
                            href="https:///forums.e-hentai.org/index.php?showuser=1725168"><img class="ygm"
                                src="https:///ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
                    <div class="c4 nosel"><a name="ulcomment"></a>Uploader Comment</div>
                    <div class="c"></div>
                </div>
                <div class="c6" id="comment_0">
                    =========================================================<br>不咕鸟欢迎各位甲方大佬委托汉化本子<br>感谢淘宝“涩谷成人玩具”对本组的大力赞助，有意向的可以去店内逛逛，多多关注。<br>备注咕咕咕有优惠<br><br>详情请联系：2820261867<br>特别注明：<br><br>禁止删除水印封面进行转载，禁止不带汉化组名进行转载，尤其是哔咔，再发现类似情况，外流版本将所有页全部打上水印，无水印版本只提供给金主。<br><br>=======================================================<br><br>RAW：<a
                        href="https:///e-hentai.org/g/1378957/7f626bf1d2/">https:///e-hentai.org/g/1378957/7f626bf1d2/</a></div>
                <div class="c7" id="cvotes_0" style="display:none"></div>
            </div>
        "#;
        assert_eq!(ele.parse::<GalleryComment>().is_ok(), true);

        // others.
        let ele = r##"
            <a name="c3922745"></a>
            <div class="c1">
                <div class="c2">
                    <div class="c3">Posted on 24 September 2020, 09:55 by: &nbsp; <a
                            href="https://e-hentai.org/uploader/Kalinkawow">Kalinkawow</a>&nbsp; &nbsp; <a
                            href="https://forums.e-hentai.org/index.php?showuser=4997064"><img class="ygm"
                                src="https://ehgt.org/g/ygm.png" alt="PM" title="Contact Poster"></a></div>
                    <div class="c4 nosel">[<a id="comment_vote_up_3922745" style="" href="#"
                            onclick="vote_comment_up(3922745); this.blur(); return false">Vote+</a>] &nbsp; [<a
                            id="comment_vote_down_3922745" style="" href="#"
                            onclick="vote_comment_down(3922745); this.blur(); return false">Vote-</a>]</div>
                    <div class="c5 nosel" onmouseover="sument.getElementById('cvotes_3922745').style.display=''"
                        onclick="this.onmouseover(); this.onmouseout=undefined"
                        onmouseout="sument.getElementById('cvotes_3922745').style.display='none'">Score <span
                            id="comment_score_3922745" style="opacity:1.0">+257</span></div>
                    <div class="c"></div>
                </div>
                <div class="c6" id="comment_3922745">猎 妈 人</div>
                <div class="c7" id="cvotes_3922745" style="display:none">Base +3, <span>q171718988 +3</span>, <span>Igarashi
                        Shioya +6</span>, <span>suhaotian +6</span>, <span>as390393473 +2</span>, <span>Subara45 +4</span>,
                    <span>louis friend +6</span>, <span>52wy1314 +6</span>, <span>随缘的亚子 +6</span>, <span>Tchami_zz +2</span>,
                    <span>sakkijarven +2</span>, <span>无证萝莉控 +6</span>, <span>DaweiX +4</span>, and 38 more...</div>
            </div>
        "##;
        assert_eq!(ele.parse::<GalleryComment>().is_ok(), true);
    }

    #[test]
    fn parse_preview_set_test() {
        let ele = r#"
            <div id="gdt">
                <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
                            title="Page 1: AnMMSC_2_001_1.png"
                            src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>
                </div>
                <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02"
                            title="Page 2: AnMMSC_2_001_2.jpg"
                            src="https://ehgt.org/ad/7a/ad7a3b7014372ce64193c118b1cfcbcf7ea68ee0-2507700-2458-3497-jpg_l.jpg"></a>
                </div>
                <div class="c"></div>
            </div>
        "#;

        assert_eq!(parse_large(ele).is_ok(), true);

        let ele = r#"
            <div id="gdt">
                <div class="gdtm" style="height:167px">
                    <div
                        style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -0px 0 no-repeat">
                        <a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01" title="Page 1: AnMMSC_2_001_1.png"
                                src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
                    </div>
                </div>
                <div class="gdtm" style="height:167px">
                    <div
                        style="margin:1px auto 0; width:100px; height:143px; background:transparent url(https://ehgt.org/m/001496/1496103-00.jpg) -100px 0 no-repeat">
                        <a href="https://e-hentai.org/s/ad7a3b7014/1496103-2"><img alt="02" title="Page 2: AnMMSC_2_001_2.jpg"
                                src="https://ehgt.org/g/blank.gif" style="width:100px; height:142px; margin:-1px 0 0 -1px"></a>
                    </div>
                </div>
                <div class="c"></div>
            </div>
        "#;

        assert_eq!(parse_medium(ele).is_ok(), true);
    }

    #[test]
    fn parse_preview_large_test() {
        let ele = r#"
            <div class="gdtl" style="height:307px"><a href="https://e-hentai.org/s/5bf9580b3b/1496103-1"><img alt="01"
                title="Page 1: AnMMSC_2_001_1.png"
                src="https://ehgt.org/5b/f9/5bf9580b3b1f63c508a8af85fc73c0567fe93722-12830376-2458-3497-png_l.jpg"></a>     ///
            </div>
        "#;

        assert_eq!(ele.parse::<GalleryPreviewLarge>().is_ok(), true);
    }
}
