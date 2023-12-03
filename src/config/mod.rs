use std::{net::Ipv4Addr, str::FromStr};

#[derive(Debug)]
pub struct Config {
    pub port: u16,
    pub addr: Ipv4Addr,
}

impl Config {
    // This method will help users to discover the builder
    pub fn builder() -> ConfigBuilder {
        ConfigBuilder::default()
    }
}


#[derive(Default, Debug)]
pub struct ConfigBuilder {
    port: u16,
    addr: String,
}

impl ConfigBuilder{
    pub fn port(mut self, port: u16) -> ConfigBuilder {
        self.port = port;
        self
    }

    pub fn addr(mut self, addr: String) -> ConfigBuilder {
        self.addr = addr;
        self
    }

    pub fn build(self) -> Config {

        let addr = Ipv4Addr::from_str(&self.addr).expect("Invalid IP address");

        Config {
            port: self.port,
            addr: addr,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_config_builder() {
        let config = Config::builder()
            .port(3030)
            .addr("0.0.0.0".to_string())
            .build();

        assert_eq!(config.port, 3030);
        assert_eq!(config.addr, Ipv4Addr::from_str("0.0.0.0").unwrap());
    }

    #[test]
    #[should_panic]
    fn test_config_builder_invalid_ip() {
        let config = Config::builder()
            .port(3030)
            .addr("invalid".to_string())
            .build();
        
        assert_eq!(config.port, 3030);
    }

}
