use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub(crate) struct Signicat {
    client_id: String,
    client_secret: String,
}

#[derive(Serialize, Deserialize)]
pub(crate) struct AccessToken {
    access_token: String,
    /// Seconds
    expires_in: u32,
    token_type: String,
    scope: String,
}

impl Signicat {
    pub fn new(client_id: String, client_secret: String) -> Self {
        Self {
            client_id,
            client_secret,
        }
    }

    pub async fn access_token(&self, client: &reqwest::Client) -> anyhow::Result<AccessToken> {
        // https://developer.signicat.com/docs/accessing-api-products/#obtaining-an-access-token
        let url = "https://api.signicat.com/auth/open/connect/token";
        let user = &self.client_id;
        let pass = Some(&self.client_secret);
        let params = [
            ("grant_type", "client_credentials"),
            ("scope", "signicat-api"),
        ];
        let resp = client
            .post(url)
            .basic_auth(user, pass)
            .form(&params)
            .send()
            .await?;

        let text = resp.text().await?;
        let token = serde_json::from_str(&text)?;
        Ok(token)
    }

    pub fn session_create_url(&self) -> String {
        format!(
            "https://api.signicat.com/auth/rest/sessions?signicat-accountId={}",
            &self.client_id
        )
    }

    pub fn session_create_body(&self) -> String {
        let body = SessionCreateBody {
            flow: "redirect",
            allow_providers: ["mitid"],
            requested_attributes: ["name"],
            callback_urls: SessionCreateBodyCallbackUrls {
                success: "http://localhost:3000/login/success",
                abort: "http://localhost:3000/login/success",
                error: "http://localhost:3000/login/success",
            },
        };
        serde_json::to_string_pretty(&body).unwrap()
    }
}

impl std::fmt::Debug for Signicat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "<Significat>")
    }
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct SessionCreateBody {
    flow: &'static str,
    allow_providers: [&'static str; 1],
    requested_attributes: [&'static str; 1],
    callback_urls: SessionCreateBodyCallbackUrls,
}

#[derive(Serialize)]
struct SessionCreateBodyCallbackUrls {
    success: &'static str,
    abort: &'static str,
    error: &'static str,
}
