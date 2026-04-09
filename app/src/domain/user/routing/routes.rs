pub struct UserRoutes;

impl UserRoutes {
    pub fn profile_url(username: &str) -> String {
        format!("/{username}")
    }
}
