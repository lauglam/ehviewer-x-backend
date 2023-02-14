use std::fmt::Debug;
use crate::eh_request::EhRequest;
use crate::eh_task::EhTask;
use crate::eh_url::EhUrl;

use crate::settings::Settings;

#[derive(Debug, PartialEq, Clone)]
pub struct EhClient {
    eh_url: EhUrl,
    settings: Settings,
    pub cookies: Vec<Cookie>,
}

impl EhClient {
    pub fn new(settings: Settings) -> EhClient {
        let eh_url = EhUrl::new(settings.clone());

        EhClient {
            eh_url,
            settings,
            cookies: Vec::new(),
        }
    }
    pub fn execute<T>(&self, request: &mut EhRequest)
        where T: Debug + Send + Sync + 'static
    {
        // let task = EhTask::new(
        //     request.method(),
        //     self.eh_url.clone(),
        //     self.cookies.clone(),
        //     request.callback(),
        // );

        todo!()
    }

    // TODO more action
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cookie {
    name: String,
    value: String,
    domain: String,
    path: String,
}

impl Cookie {
    pub fn new(name: &str, value: &str, domain: &str, path: &str) -> Cookie {
        Cookie {
            name: String::from(name),
            value: String::from(value),
            domain: String::from(domain),
            path: String::from(path),
        }
    }
}
