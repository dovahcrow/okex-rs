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

#[cfg(test)]
mod test {
    use hyper::Method;
    use url::Url;

    use super::Credential;
    use failure::Fallible;

    #[test]
    fn test_signature_get() -> Fallible<()> {
        let tr = Credential(
            "LAqUlngMIQkIUjXMUreyu3qn".into(),
            "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO".into(),
        );
        let (_, sig) = tr.signature(
            Method::GET,
            1518064236,
            &Url::parse("http://a.com/api/v1/instrument")?,
            "",
        )?;
        assert_eq!(
            sig,
            "c7682d435d0cfe87c16098df34ef2eb5a549d4c5a3c2b1f0f77b8af73423bf00"
        );
        Ok(())
    }

    #[test]
    fn test_signature_get_param() -> Fallible<()> {
        let tr = Credential(
            "LAqUlngMIQkIUjXMUreyu3qn".into(),
            "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO".into(),
        );
        let (_, sig) = tr.signature(
            Method::GET,
            1518064237,
            &Url::parse_with_params(
                "http://a.com/api/v1/instrument",
                &[("filter", r#"{"symbol": "XBTM15"}"#)],
            )?,
            "",
        )?;
        assert_eq!(
            sig,
            "e2f422547eecb5b3cb29ade2127e21b858b235b386bfa45e1c1756eb3383919f"
        );
        Ok(())
    }

    #[test]
    fn test_signature_post() -> Fallible<()> {
        let tr = Credential(
            "LAqUlngMIQkIUjXMUreyu3qn".into(),
            "chNOOS4KvNXR_Xq4k4c9qsfoKWvnDecLATCRlcBwyKDYnWgO".into(),
        );
        let (_, sig) = tr.signature(
            Method::POST,
            1518064238,
            &Url::parse("http://a.com/api/v1/order")?,
            r#"{"symbol":"XBTM15","price":219.0,"clOrdID":"mm_bitmex_1a/oemUeQ4CAJZgP3fjHsA","orderQty":98}"#,
        )?;
        assert_eq!(
            sig,
            "1749cd2ccae4aa49048ae09f0b95110cee706e0944e6a14ad0b3a8cb45bd336b"
        );
        Ok(())
    }
}
