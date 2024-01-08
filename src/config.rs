use std::net::Ipv4Addr;

pub struct Server {
    host: Ipv4Addr,
    port: u16,
}

impl Server {
    pub fn new() -> Self {
        // FIXME: 上位環境ごとに設定を変える
        let localhost: Ipv4Addr = Ipv4Addr::new(127, 0, 0, 1);
        let port: u16 = 8080;

        return Self {
            host: localhost,
            port,
        };
    }

    // pub fn get_host(&self) -> Ipv4Addr {
    //     return self.host;
    // }

    // pub fn get_port(&self) -> u16 {
    //     return self.port;
    // }

    pub fn get_address(&self) -> String {
        return format!("{}:{}", self.host, self.port);
    }
}
