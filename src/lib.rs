pub mod api;
pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn get() {
        let body = reqwest::blocking::get("https://anondns.net").unwrap()
            .text().unwrap();

        assert_eq!(body.len() > 1, true, "GET request body was empty");
    }

    #[test]
    fn register() -> Result<(), crate::error::DnsApiError> {
        const TEST_SUBDOMAIN: &str = "INSERT_NAME_HERE";
        const TEST_TARGET: std::net::Ipv4Addr = std::net::Ipv4Addr::new(127, 0, 0, 1);

        let mut service = crate::api::Service::new();
        let result = service.register(TEST_SUBDOMAIN, TEST_TARGET);

        result.map(|_| ())
    }
    
    #[test]
    fn update() -> Result<(), crate::error::DnsApiError> {
        const TEST_SUBDOMAIN: &str = "INSERT_NAME_HERE";
        const TEST_UPDATE_TARGET: std::net::Ipv4Addr = std::net::Ipv4Addr::new(255, 255, 255, 255);
        const TEST_TOKEN: &str = "INSERT_TOKEN_HERE";

        let mut service = crate::api::Service::new();
        let result = service.update(TEST_SUBDOMAIN, TEST_UPDATE_TARGET, String::from(TEST_TOKEN));

        result.map(|_| ())
    }
}