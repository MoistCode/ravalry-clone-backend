pub mod page {
    pub const URL: &'static str = "http://127.0.0.1:8080/";
}

pub mod server {
    pub fn get_tunnel_url() -> String {
        let mut host = "0.0.0.0";

        if cfg!(debug_assertions) {
            host = "127.0.0.1";
        }

        format!("{}:8080", host,)
    }
}