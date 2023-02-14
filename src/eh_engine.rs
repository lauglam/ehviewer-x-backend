use crate::{eh_url, EhResult};
use crate::structures::SignIn;

#[derive(Debug)]
pub struct EhEngine {
    client_builder: reqwest::ClientBuilder,
}

impl EhEngine {
    pub fn new() -> EhEngine {
        let client_builder = reqwest::Client::builder()
            .cookie_store(true)
            .referer(true);

        EhEngine { client_builder }
    }

    pub async fn sign_in(self, username: &str, password: &str) -> EhResult<SignIn> {
        let referer = "https://forums.e-hentai.org/index.php?act=Login&CODE=0";
        let client = self.client_builder.build()?;

        let params = [
            ("referer", referer),
            ("b", ""),
            ("bt", ""),
            ("UserName", username),
            ("PassWord", password),
            ("CookieDate", "1"),
        ];

        let response = client
            .get(eh_url::API_SIGN_IN)
            .form(&params)
            .send()
            .await?
            .text()
            .await?;

        let sign_in = response.parse::<SignIn>();

        todo!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn sign_in_test() {
        let engine = EhEngine::new();
        let sign_in = engine.sign_in("xxxx", "xxxx").await;

    }
}
