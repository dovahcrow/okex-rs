use base64::encode;
use http::Method;
use ring::hmac;
use url::Url;

#[derive(Clone, Debug)]
pub struct Credential {
    key: String,
    secret: String,
    passphrase: String,
}

impl Credential {
    pub(crate) fn new(key: &str, secret: &str, password: &str) -> Self {
        Self {
            key: key.into(),
            secret: secret.into(),
            passphrase: password.into(),
        }
    }

    pub(crate) fn passphrase(&self) -> &str {
        &self.passphrase
    }

    pub(crate) fn signature(
        &self,
        method: Method,
        timestamp: &str,
        url: &Url,
        body: &str,
    ) -> (&str, String) {
        // sign=CryptoJS.enc.Base64.stringify(CryptoJS.HmacSHA256(timestamp + 'GET' + '/users/self/verify' + body, SecretKey))
        let signed_key = hmac::Key::new(hmac::HMAC_SHA256, self.secret.as_bytes());
        let sign_message = match url.query() {
            Some(query) => format!(
                "{}{}{}?{}{}",
                timestamp,
                method.as_str(),
                url.path(),
                query,
                body
            ),
            None => format!("{}{}{}{}", timestamp, method.as_str(), url.path(), body),
        };

        let signature = encode(hmac::sign(&signed_key, sign_message.as_bytes()).as_ref());
        (self.key.as_str(), signature)
    }
}
