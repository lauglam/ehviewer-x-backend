use reqwest::header;
use reqwest::header::HeaderMap;
use crate::{
    EhResult,
    settings::Settings,
    eh_url::{self, EhUrl},
    structures::{GalleryList, SignIn, GalleryDetail},
};

#[derive(Debug)]
pub struct EhClient {
    client: reqwest::Client,
    eh_url: EhUrl,
    settings: Settings,
}

impl EhClient {
    pub fn new() -> EhClient {
        let client = reqwest::Client::builder()
            .cookie_store(true)
            .referer(true)
            .build()
            .unwrap();

        let settings = Settings::new();
        let eh_url = EhUrl::new(settings.clone());

        EhClient { client, settings, eh_url }
    }

    pub async fn sign_in(&self, username: &str, password: &str) -> EhResult<SignIn> {
        let referer = "https://forums.e-hentai.org/index.php?act=Login&CODE=0";
        let origin = "https://forums.e-hentai.org";
        ;

        let params = [
            ("referer", referer),
            ("b", ""),
            ("bt", ""),
            ("UserName", username),
            ("PassWord", password),
            ("CookieDate", "1"),
        ];

        let mut headers = HeaderMap::from_iter([
            (header::REFERER, referer.parse().unwrap()),
            (header::ORIGIN, origin.parse().unwrap())
        ]);

        let res = self.client
            .post(eh_url::API_SIGN_IN)
            .headers(headers)
            .form(&params)
            .send()
            .await?
            .text()
            .await?;

        Ok(res.parse::<SignIn>()?)
    }

    pub async fn get_gallery_list(&self, url: &str) -> EhResult<GalleryList> {
        let headers = HeaderMap::from_iter([
            (header::REFERER, self.eh_url.referer().parse().unwrap()),
        ]);

        let res = self.client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        Ok(res.parse::<GalleryList>()?)
    }

    pub async fn get_gallery_detail(&self, url: &str) -> EhResult<GalleryDetail> {
        let headers = HeaderMap::from_iter([
            (header::REFERER, self.eh_url.referer().parse().unwrap()),
        ]);

        let res = self.client
            .get(url)
            .headers(headers)
            .send()
            .await?
            .text()
            .await?;

        Ok(res.parse::<GalleryDetail>()?)
    }
}

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Method {
    MethodSignIn,
    MethodGetGalleryList,
    MethodGetGalleryDetail,
    MethodGetPreviewSet,
    MethodGetRateGallery,
    MethodGetCommentGallery,
    MethodGetGalleryToken,
    MethodGetFavorites,
    MethodAddFavorites,
    MethodAddFavoritesRange,
    MethodModifyFavorites,
    MethodGetTorrentList,
    MethodGetProfile,
    MethodVoteComment,
    MethodImageSearch,
    MethodArchiveList,
    MethodDownloadArchive,
    MethodVoteTag,

    //Added by EHentaiAPI.
    MethodGetGalleryPageApi,
    MethodGetGalleryPage,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sign_in_test() {
        let engine = EhClient::new();
        assert_eq!(engine.sign_in("xxxx", "xxxx").await.is_ok(), true)
    }

    #[tokio::test]
    async fn get_gallery_list_test() {
        let engine = EhClient::new();
        assert_eq!(engine.get_gallery_list("https://e-hentai.org").await.is_ok(), true);
    }

    #[tokio::test]
    async fn get_gallery_detail_test() {
        let engine = EhClient::new();
        assert_eq!(engine.get_gallery_detail("https://e-hentai.org/g/2062067/588c82702b/").await.is_ok(), true);
    }
}
