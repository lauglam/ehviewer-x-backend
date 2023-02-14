use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;
use crate::eh_client::Cookie;
use crate::eh_request::{Method};
use crate::eh_url::EhUrl;

pub struct EhTask<T>
    where T: Debug + Send + Sync + 'static
{
    task: Option<Pin<Box<dyn Future<Output=T>>>>,
    method: Method,
    eh_url: EhUrl,
    cookies: Vec<Cookie>,
    callback: Box<dyn FnOnce()>,
}

impl<T> EhTask<T>
    where T: Debug + Send + Sync + 'static
{
    pub fn new(
        method: Method,
        eh_url: EhUrl,
        cookies: Vec<Cookie>,
        callback: Box<dyn FnOnce()>,
    ) -> EhTask<T>
    {
        EhTask {
            method,
            eh_url,
            cookies,
            callback,
            task: None,
        }
    }

    pub fn execute_on_executor(){

    }
}
