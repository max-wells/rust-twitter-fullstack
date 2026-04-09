pub struct HomeRoutes;

impl HomeRoutes {
    pub fn base_url() -> &'static str {
        "/"
    }

    pub fn following_url() -> &'static str {
        "/following"
    }

    pub fn following_segment() -> &'static str {
        "following"
    }

    pub fn label() -> &'static str {
        "Home"
    }
}
