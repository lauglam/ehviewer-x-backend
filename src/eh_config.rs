#![allow(dead_code, non_upper_case_globals)]

use std::sync::Mutex;
use once_cell::sync::Lazy;

/**
 * The Cookie key of uconfig
 */
pub const KEY_UCONFIG: &str = "uconfig";
/**
 * The Cookie key of lofi resolution
 */
pub const KEY_LOFI_RESOLUTION: &str = "xres";
/**
 * The Cookie key of show warning
 */
pub const KEY_CONTENT_WARNING: &str = "nw";
/**
 * load images through the Hentai@Home Network
 */
pub const LOAD_FROM_HAH_YES: &str = "y";
/**
 * do not load images through the Hentai@Home Network
 */
pub const LOAD_FROM_HAH_NO: &str = "n";
/**
 * Image Size Auto
 */
pub const IMAGE_SIZE_AUTO: &str = "a";
/**
 * Image Size 780x
 */
pub const IMAGE_SIZE_780X: &str = "780";
/**
 * Image Size 980x
 */
pub const IMAGE_SIZE_980X: &str = "980";
/**
 * Image Size 1280x
 */
pub const IMAGE_SIZE_1280X: &str = "1280";
/**
 * Image Size 1600x
 */
pub const IMAGE_SIZE_1600X: &str = "1600";
/**
 * Image Size 2400x
 */
pub const IMAGE_SIZE_2400X: &str = "2400";
/**
 * Manual Accept, Manual Start
 */
pub const ARCHIVER_DOWNLOAD_MAMS: &str = "0";
/**
 * >Manual Accept, Auto Start
 */
pub const ARCHIVER_DOWNLOAD_AAMS: &str = "1";
/**
 * Auto Accept, Manual Start
 */
pub const ARCHIVER_DOWNLOAD_MAAS: &str = "2";
/**
 * Auto Accept, Auto Start
 */
pub const ARCHIVER_DOWNLOAD_AAAS: &str = "3";
/**
 * List View on the front and search pages
 */
pub const LAYOUT_MODE_LIST: &str = "l";
/**
 * Thumbnail View on the front and search pages
 */
pub const LAYOUT_MODE_THUMB: &str = "t";

pub const MISC: u32 = 0x1;
pub const DOUJINSHI: u32 = 0x2;
pub const MANGA: u32 = 0x4;
pub const ARTIST_CG: u32 = 0x8;
pub const GAME_CG: u32 = 0x10;
pub const IMAGE_SET: u32 = 0x20;
pub const COSPLAY: u32 = 0x40;
pub const ASIAN_PORN: u32 = 0x80;
pub const NON_H: u32 = 0x100;
pub const WESTERN: u32 = 0x200;
pub const ALL_CATEGORY: u32 = 0x3ff;
pub const NAMESPACES_RECLASS: u32 = 0x1;
pub const NAMESPACES_LANGUAGE: u32 = 0x2;
pub const NAMESPACES_PARODY: u32 = 0x4;
pub const NAMESPACES_CHARACTER: u32 = 0x8;
pub const NAMESPACES_GROUP: u32 = 0x10;
pub const NAMESPACES_ARTIST: u32 = 0x20;
pub const NAMESPACES_MALE: u32 = 0x40;
pub const NAMESPACES_FEMALE: u32 = 0x80;

pub const JAPANESE_ORIGINAL: &str = "0";
pub const JAPANESE_TRANSLATED: &str = "1024";
pub const JAPANESE_REWRITE: &str = "2048";
pub const ENGLISH_ORIGINAL: &str = "1";
pub const ENGLISH_TRANSLATED: &str = "1025";
pub const ENGLISH_REWRITE: &str = "2049";
pub const CHINESE_ORIGINAL: &str = "10";
pub const CHINESE_TRANSLATED: &str = "1034";
pub const CHINESE_REWRITE: &str = "2058";
pub const DUTCH_ORIGINAL: &str = "20";
pub const DUTCH_TRANSLATED: &str = "1044";
pub const DUTCH_REWRITE: &str = "2068";
pub const FRENCH_ORIGINAL: &str = "30";
pub const FRENCH_TRANSLATED: &str = "1054";
pub const FRENCH_REWRITE: &str = "2078";
pub const GERMAN_ORIGINAL: &str = "40";
pub const GERMAN_TRANSLATED: &str = "1064";
pub const GERMAN_REWRITE: &str = "2088";
pub const HUNGARIAN_ORIGINAL: &str = "50";
pub const HUNGARIAN_TRANSLATED: &str = "1074";
pub const HUNGARIAN_REWRITE: &str = "2098";
pub const ITALIAN_ORIGINAL: &str = "60";
pub const ITALIAN_TRANSLATED: &str = "1084";
pub const ITALIAN_REWRITE: &str = "2108";
pub const KOREAN_ORIGINAL: &str = "70";
pub const KOREAN_TRANSLATED: &str = "1094";
pub const KOREAN_REWRITE: &str = "2118";
pub const POLISH_ORIGINAL: &str = "80";
pub const POLISH_TRANSLATED: &str = "1104";
pub const POLISH_REWRITE: &str = "2128";
pub const PORTUGUESE_ORIGINAL: &str = "90";
pub const PORTUGUESE_TRANSLATED: &str = "1114";
pub const PORTUGUESE_REWRITE: &str = "2138";
pub const RUSSIAN_ORIGINAL: &str = "100";
pub const RUSSIAN_TRANSLATED: &str = "1124";
pub const RUSSIAN_REWRITE: &str = "2148";
pub const SPANISH_ORIGINAL: &str = "110";
pub const SPANISH_TRANSLATED: &str = "1134";
pub const SPANISH_REWRITE: &str = "2158";
pub const THAI_ORIGINAL: &str = "120";
pub const THAI_TRANSLATED: &str = "1144";
pub const THAI_REWRITE: &str = "2168";
pub const VIETNAMESE_ORIGINAL: &str = "130";
pub const VIETNAMESE_TRANSLATED: &str = "1154";
pub const VIETNAMESE_REWRITE: &str = "2178";
pub const NA_ORIGINAL: &str = "254";
pub const NA_TRANSLATED: &str = "1278";
pub const NA_REWRITE: &str = "2302";
pub const OTHER_ORIGINAL: &str = "255";
pub const OTHER_TRANSLATED: &str = "1279";
pub const OTHER_REWRITE: &str = "2303";
/**
 * 25 results per page for the index/search page and torrent search pages
 */
pub const RESULT_COUNT_25: &str = "0";
/**
 * 50 results per page for the index/search page and torrent search pages
 */
pub const RESULT_COUNT_50: &str = "1";
/**
 * 100 results per page for the index/search page and torrent search pages
 */
pub const RESULT_COUNT_100: &str = "2";
/**
 * 200 results per page for the index/search page and torrent search pages
 */
pub const RESULT_COUNT_200: &str = "3";
/**
 * On mouse-over
 */
pub const MOUSE_OVER_YES: &str = "m";
/**
 * On page load
 */
pub const MOUSE_OVER_NO: &str = "p";
/**
 * Preview normal size
 */
pub const PREVIEW_SIZE_NORMAL: &str = "m";
/**
 * Preview large size
 */
pub const PREVIEW_SIZE_LARGE: &str = "l";
/**
 * 4 row preview per page
 */
pub const PREVIEW_ROW_4: &str = "2";
/**
 * 10 row preview per page
 */
pub const PREVIEW_ROW_10: &str = "5";
/**
 * 20 row preview per page
 */
pub const PREVIEW_ROW_20: &str = "10";
/**
 * 40 row preview per page
 */
pub const PREVIEW_ROW_40: &str = "20";
/**
 * Oldest comments first
 */
pub const COMMENTS_SORT_OLDEST_FIRST: &str = "a";
/**
 * Recent comments first
 */
pub const COMMENTS_SORT_RECENT_FIRST: &str = "d";
/**
 * By highest score
 */
pub const COMMENTS_SORT_HIGHEST_SCORE_FIRST: &str = "s";
/**
 * Show gallery_list comment votes On score hover or click
 */
pub const COMMENTS_VOTES_POP: &str = "0";
/**
 * Always show gallery_list comment votes
 */
pub const COMMENTS_VOTES_ALWAYS: &str = "1";
/**
 * Sort order for gallery_list tags alphabetically
 */
pub const TAGS_SORT_ALPHABETICAL: &str = "a";
/**
 * Sort order for gallery_list tags by tag power
 */
pub const TAGS_SORT_POWER: &str = "p";
/**
 * Show gallery_list page numbers
 */
pub const SHOW_GALLERY_INDEX_YES: &str = "1";
/**
 * Do not show gallery_list page numbers
 */
pub const SHOW_GALLERY_INDEX_NO: &str = "0";
/**
 * Enable Tag Flagging
 */
pub const ENABLE_TAG_FLAGGING_YES: &str = "y";
/**
 * Do not enable Tag Flagging
 */
pub const ENABLE_TAG_FLAGGING_NO: &str = "n";
/**
 * Always display the original images
 */
pub const ALWAYS_ORIGINAL_YES: &str = "y";
/**
 * Do not Always display the original images
 */
pub const ALWAYS_ORIGINAL_NO: &str = "n";
/**
 * Enable the Multi-Page Viewe
 */
pub const MULTI_PAGE_YES: &str = "y";
/**
 * Do not enable the Multi-Page Viewe
 */
pub const MULTI_PAGE_NO: &str = "n";
/**
 * Align left, only scale if image is larger than browser width
 */
pub const MULTI_PAGE_STYLE_N: &str = "n";
/**
 * Align center, only scale if image is larger than browser width
 */
pub const MULTI_PAGE_STYLE_C: &str = "c";
/**
 * Align center, Always scale images to fit browser width
 */
pub const MULTI_PAGE_STYLE_Y: &str = "y";
/**
 * Show Multi-Page Viewer Thumbnail Pane
 */
pub const MULTI_PAGE_THUMB_SHOW: &str = "n";
/**
 * Hide Multi-Page Viewer Thumbnail Pane
 */
pub const MULTI_PAGE_THUMB_HIDE: &str = "y";
/**
 * 460x for lofi resolution
 */
pub const LOFI_RESOLUTION_460X: &str = "1";
/**
 * 780X for lofi resolution
 */
pub const LOFI_RESOLUTION_780X: &str = "2";
/**
 * 980X for lofi resolution
 */
pub const LOFI_RESOLUTION_980X: &str = "3";
/**
 * show warning
 */
pub const CONTENT_WARNING_SHOW: &str = "0";
/**
 * not show warning
 */
pub const CONTENT_WARNING_NOT_SHOW: &str = "1";
/**
 * Default gallery_list title
 */
const GALLERY_TITLE_DEFAULT: &str = "r";
/**
 * Show popular
 */
const POPULAR_YES: &str = "y";
/**
 * Sort favorites by last gallery_list update time
 */
const FAVORITES_SORT_FAVORITED_TIME: &str = "f";
/**
 * Load images through the Hentai@Home Network<br/>
 * key: {@link #KEY_LOAD_FROM_HAH}<br/>
 * value: {@link #LOAD_FROM_HAH_YES}, {@link #LOAD_FROM_HAH_NO}
 */
pub static mut loadFromHAH: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(LOAD_FROM_HAH_YES));

/**
 * Image Size<br/>
 * key: {@link #KEY_IMAGE_SIZE}<br/>
 * value: {@link #IMAGE_SIZE_AUTO}, {@link #IMAGE_SIZE_780X}, {@link #IMAGE_SIZE_980X},
 * {@link #IMAGE_SIZE_1280X}, {@link #IMAGE_SIZE_1600X}, {@link #IMAGE_SIZE_2400X}
 */
pub static imageSize: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(IMAGE_SIZE_AUTO));

/**
 * Scale width<br/>
 * key: {@link #KEY_SCALE_WIDTH}<br/>
 * value: 0 for no limit
 */
pub static scaleWidth: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

/**
 * Scale height<br/>
 * key: {@link #KEY_SCALE_HEIGHT}<br/>
 * value: 0 for no limit
 */
pub static scaleHeight: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

/**
 * Gallery title<br/>
 * key: {@link #KEY_GALLERY_TITLE}<br/>
 * value: {@link #GALLERY_TITLE_DEFAULT}, {@link #GALLERY_TITLE_JAPANESE}
 */
pub static galleryTitle: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(GALLERY_TITLE_DEFAULT));

/**
 * The default behavior for downloading an archiver<br/>
 * key: {@link #KEY_ARCHIVER_DOWNLOAD}<br/>
 * value: {@link #ARCHIVER_DOWNLOAD_MAMS}, {@link #ARCHIVER_DOWNLOAD_AAMS},
 * {@link #ARCHIVER_DOWNLOAD_MAAS}, {@link #ARCHIVER_DOWNLOAD_AAAS}
 */
pub static archiverDownload: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(ARCHIVER_DOWNLOAD_MAMS));

/**
 * Display mode used on the front and search pages<br/>
 * false for list, true for thumb<br/>
 * key: {@link #KEY_LAYOUT_MODE}<br/>
 * value: {@link #LAYOUT_MODE_LIST}, {@link #LAYOUT_MODE_THUMB}
 */
pub static layoutMode: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(LAYOUT_MODE_LIST));

/**
 * Show popular or not<br/>
 * key: {@link #KEY_POPULAR}<br/>
 * value: {@link #POPULAR_YES}, {@link #POPULAR_NO}
 */
pub static popular: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(POPULAR_YES));

/**
 * Default categories on the front page<br/>
 * key: {@link #KEY_DEFAULT_CATEGORIES}<br/>
 * value: the value of categories, for multiple use & operation,
 * 0 for none
 */
pub static defaultCategories: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

/**
 * <br/>
 * key: {@link #KEY_FAVORITES_SORT}<br/>
 * value: {@link #FAVORITES_SORT_GALLERY_UPDATE_TIME}, {@link #FAVORITES_SORT_FAVORITED_TIME}
 */
pub static favoritesSort: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(FAVORITES_SORT_FAVORITED_TIME));

/**
 * Certain namespaces excluded from a default tag search<br/>
 * key: {@link #KEY_EXCLUDED_NAMESPACES}<br/>
 * value: the value of namespaces, for multiple use & operation,
 * 0 for none
 */
pub static excludedNamespaces: Lazy<Mutex<u32>> = Lazy::new(|| Mutex::new(0));

/**
 * Certain languages excluded from list and searches<br/>
 * key: {@link #KEY_EXCLUDED_LANGUAGES}<br/>
 * value: {@link #JAPANESE_TRANSLATED}, {@link #JAPANESE_REWRITE} ...
 * For multiple languages, use <code>x<code/> to combine them, like 1x1024x2048
 */
pub static excludedLanguages: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(""));

/**
 * How many results would you like per page for the index/search page
 * and torrent search pages<br/>
 * key: {@link #KEY_RESULT_COUNT}<br/>
 * value: {@link #RESULT_COUNT_25}, {@link #RESULT_COUNT_50},
 * {@link #RESULT_COUNT_100}, {@link #RESULT_COUNT_200}<br/>
 * Require <code>Hath Perk:Paging Enlargement</code>
 */
pub static resultCount: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(RESULT_COUNT_25));

/**
 * mouse-over thumb<br/>
 * key: {@link #KEY_MOUSE_OVER}<br/>
 * value: {@link #MOUSE_OVER_YES}, {@link #MOUSE_OVER_NO}
 */
pub static mouseOver: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(MOUSE_OVER_YES));

/**
 * Default preview mode<br/>
 * key: {@link #KEY_PREVIEW_SIZE}<br/>
 * value: {@link #PREVIEW_SIZE_NORMAL}, {@link #PREVIEW_SIZE_LARGE}
 */
pub static previewSize: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(PREVIEW_SIZE_LARGE));

/**
 * Preview row<br/>
 * key: {@link #KEY_PREVIEW_ROW}<br/>
 * value: {@link #PREVIEW_ROW_4}, {@link #PREVIEW_ROW_10},
 * {@link #PREVIEW_ROW_20}, {@link #PREVIEW_ROW_40}
 */
pub static previewRow: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(PREVIEW_ROW_4));

/**
 * Sort order for gallery_list comments<br/>
 * key: {@link #KEY_COMMENTS_SORT}<br/>
 * value: {@link #COMMENTS_SORT_OLDEST_FIRST}, {@link #COMMENTS_SORT_RECENT_FIRST},
 * {@link #COMMENTS_SORT_HIGHEST_SCORE_FIRST}
 */
pub static commentSort: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(COMMENTS_SORT_OLDEST_FIRST));

/**
 * Show gallery_list comment votes mode<br/>
 * key: {@link #KEY_COMMENTS_VOTES}<br/>
 * value: {@link #COMMENTS_VOTES_POP}, {@link #COMMENTS_VOTES_ALWAYS}
 */
pub static commentVotes: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(COMMENTS_VOTES_POP));


/**
 * Sort order for gallery_list tags<br/>
 * key: {@link #KEY_TAGS_SORT}<br/>
 * value: {@link #TAGS_SORT_ALPHABETICAL}, {@link #TAGS_SORT_POWER}
 */
pub static tagSort: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(TAGS_SORT_ALPHABETICAL));

/**
 * Show gallery_list page numbers<br/>
 * key: {@link #KEY_SHOW_GALLERY_INDEX}<br/>
 * value: {@link #SHOW_GALLERY_INDEX_YES}, {@link #SHOW_GALLERY_INDEX_NO}
 */
pub static showGalleryIndex: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(SHOW_GALLERY_INDEX_YES));

/**
 * The IP of a proxy-enabled Hentai@Home Client
 * to load all images<br/>
 * key: {@link #KEY_HAH_CLIENT_IP_PORT}<br/>
 */
pub static hahClientIp: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(""));

/**
 * The PORT of a proxy-enabled Hentai@Home Client
 * to load all images<br/>
 * key: {@link #KEY_HAH_CLIENT_IP_PORT}<br/>
 */
pub static hahClientPort: Lazy<Mutex<i8>> = Lazy::new(|| Mutex::new(-1));

/**
 * The passkey of a proxy-enabled Hentai@Home Client
 * to load all images<br/>
 * key: {@link #KEY_HAH_CLIENT_PASSKEY}<br/>
 */
pub static hahClientPasskey: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(""));

/**
 * Enable tag flagging
 * key: {@link #KEY_ENABLE_TAG_FLAGGING}<br/>
 * value: {@link #ENABLE_TAG_FLAGGING_YES}, {@link #ENABLE_TAG_FLAGGING_NO}<br/>
 * <code>Bronze Star</code> or <code>Hath Perk: Tag Flagging</code> Required
 */
pub static enableTagFlagging: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(ENABLE_TAG_FLAGGING_NO));

/**
 * Always display the original images instead of the resampled versions<br/>
 * key: {@link #KEY_ALWAYS_ORIGINAL}<br/>
 * value: {@link #ALWAYS_ORIGINAL_YES}, {@link #ALWAYS_ORIGINAL_NO}<br/>
 * <code>Silver Star</code> or <code>Hath Perk: Source Nexus</code> Required
 */
pub static alwaysOriginal: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(ALWAYS_ORIGINAL_NO));

/**
 * Enable the multi-Page Viewer<br/>
 * key: {@link #KEY_MULTI_PAGE}<br/>
 * value: {@link #MULTI_PAGE_YES}, {@link #MULTI_PAGE_NO}<br/>
 * <code>Gold Star</code> or <code>Hath Perk: Multi-Page Viewer</code> Required
 */
pub static multiPage: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(MULTI_PAGE_NO));

/**
 * Multi-Page Viewer Display Style<br/>
 * key: {@link #KEY_MULTI_PAGE_STYLE}<br/>
 * value: {@link #MULTI_PAGE_STYLE_N}, {@link #MULTI_PAGE_STYLE_C},
 * {@link #MULTI_PAGE_STYLE_Y}<br/>
 * <code>Gold Star</code> or <code>Hath Perk: Multi-Page Viewer</code> Required
 */
pub static multiPageStyle: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(MULTI_PAGE_STYLE_N));

/**
 * Multi-Page Viewer Thumbnail Pane<br/>
 * key: {@link #KEY_MULTI_PAGE_THUMB}<br/>
 * value: {@link #MULTI_PAGE_THUMB_SHOW}, {@link #MULTI_PAGE_THUMB_HIDE}<br/>
 * <code>Gold Star</code> or <code>Hath Perk: Multi-Page Viewer</code> Required
 */
pub static multiPageThumb: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(MULTI_PAGE_THUMB_SHOW));

/**
 * Lofi resolution
 * key: {@link #KEY_LOFI_RESOLUTION}<br/>
 * value: {@link #LOFI_RESOLUTION_460X}, {@link #LOFI_RESOLUTION_780X},
 * {@link #LOFI_RESOLUTION_980X}
 */
pub static lofiResolution: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(LOFI_RESOLUTION_980X));

/**
 * Show content warning
 * key: {@link #KEY_CONTENT_WARNING}<br/>
 * value: {@link #CONTENT_WARNING_SHOW}, {@link #CONTENT_WARNING_NOT_SHOW}
 */
pub static contentWarning: Lazy<Mutex<&str>> = Lazy::new(|| Mutex::new(CONTENT_WARNING_NOT_SHOW));
