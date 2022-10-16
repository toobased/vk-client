pub mod web {
    use reqwest::Url;

    pub static BASE_URL: &str = "https://oauth.vk.com/authorize";
    pub static DEFAULT_REDIRECT: &str = "https://oauth.vk.com/blank.html";
    pub static DEFAULT_RESPONSE_TYPE: &str = "token";
    pub static DEFAULT_SCOPE: &str = "notify,friends,photos,audio,video,stories,pages,status,notes,wall,ads,offline,docs,groups,notifications,stats,email,market";

    #[derive(Default)]
    pub struct WebOAuthUrlParams {
        pub app_id: u32,
        pub scope: Option<&'static str>,
        pub redirect: Option<&'static str>,
        pub response_type: Option<&'static str>
    }

    pub fn get_token_url (c: WebOAuthUrlParams) -> Url {
        let scope = c.scope.unwrap_or(DEFAULT_SCOPE);
        let redirect = c.redirect.unwrap_or(DEFAULT_REDIRECT);
        let response_type = c.response_type.unwrap_or(DEFAULT_RESPONSE_TYPE);
        let mut url = Url::parse(BASE_URL).unwrap();
        url.query_pairs_mut()
            .clear()
            .append_pair("client_id", &c.app_id.to_string())
            .append_pair("scope", scope)
            .append_pair("redirect_uri", redirect)
            .append_pair("response_type", response_type)
            .append_pair("display", "page");
        url
    }
}
