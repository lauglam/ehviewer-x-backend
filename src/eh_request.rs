pub struct EhRequest {
    method: Method,
    callback: Option<Box<dyn FnOnce()>>,
}

impl EhRequest {
    pub fn new(method: Method, callback: Box<dyn FnOnce()>) -> EhRequest {
        EhRequest { method, callback: Some(callback) }
    }

    pub fn set_args(&self) -> &EhRequest {
        todo!()
    }

    pub fn set_task() {}

    pub fn method(&self) -> Method {
        self.method
    }

    pub fn callback(&mut self) -> Box<dyn FnOnce()> {
        self.callback.take().unwrap()
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
