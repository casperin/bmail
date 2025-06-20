use axum_extra::extract::{cookie::Cookie, CookieJar};

pub(crate) struct Cook {
    items: Vec<(String, String)>,
}

impl Cook {
    pub fn new() -> Self {
        Self { items: vec![] }
    }

    pub fn add(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.items.push((key.into(), val.into()));
        self
    }

    pub fn jar(self) -> CookieJar {
        self.items
            .into_iter()
            .fold(CookieJar::new(), |jar, (k, v)| {
                jar.add(Cookie::build((k, v)).path("/").secure(true).http_only(true))
            })
    }
}
