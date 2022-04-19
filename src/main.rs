#![allow(dead_code, unused_variables, non_camel_case_types)]

#[derive(Clone)]
struct TLSCert {
    key: String,
    cert: String,
}

type ms = u32;

struct Server {
    host: String,
    port: u16,
    tls: Option<TLSCert>,
    hot_reload: bool,
    timeout: ms,
}

// ---- Without builder pattern ----
// impl Server {
//     fn new(host: String, port: u16) -> Self {
//         Self { host, port, tls: None, hot_reload: false, timeout: 2000 }
//     }
//     fn new_tls(host: String, port: u16, tls: TLSCert) -> Self {
//         Self { host, port, tls: Some(tls), hot_reload: false, timeout: 2000 }
//     }
//     fn new_advanced(
//         host: String,
//         port: u16,
//         tls: Option<TLSCert>,
//         hot_reload: bool,
//         timeout: ms) -> Self {
//         Self { host, port, tls, hot_reload, timeout }
//     }
// }

// ----- With builder pattern -----
impl Server {
    fn new(host: String, port: u16) -> ServerBuilder {
        ServerBuilder {
            host,
            port,
            tls: None,
            hot_reload: None,
            timeout: None,
        }
    }
}

pub struct ServerBuilder {
    host: String,
    port: u16,
    tls: Option<TLSCert>,
    hot_reload: Option<bool>,
    timeout: Option<ms>,
}

impl ServerBuilder {
    fn tls(&mut self, tls: TLSCert) -> &mut Self {
        self.tls = Some(tls);
        self
    }
    fn hot_reload(&mut self, hot_reload: bool) -> &mut Self {
        self.hot_reload = Some(hot_reload);
        self
    }
    fn timeout(&mut self, timeout: ms) -> &mut Self {
        self.timeout = Some(timeout);
        self
    }
    fn build(&mut self) -> Server {
        Server {
            host: self.host.clone(),
            port: self.port,
            tls: self.tls.clone(),
            hot_reload: self.hot_reload.unwrap_or_default(),
            timeout: self.timeout.unwrap_or(2000),
        }
    }
}

fn main() {
    let host = "localhost".to_owned();
    let port = 8080;

    let cert = TLSCert {
        key: "...".to_owned(),
        cert: "...".to_owned(),
    };

    // ---- Without builder pattern ----

    // // Basic server
    // let basic_server = Server::new(host.clone(), port);

    // // Server with TLS
    // let tls_server = Server::new_tls(host.clone(), port, cert.clone());

    // // Fully configured server
    // let server = Server::new_advanced(
    //     host.clone(),
    //     port,
    //     Some(cert.clone()),
    //     true,
    //     5000
    // );

    // ----- With builder pattern -----

    // Basic server
    let basic_server = Server::new(host.clone(), port).build();

    // Server with TLS
    let tls_server = Server::new(host.clone(), port).tls(cert.clone()).build();

    // Fully configured server
    let server = Server::new(host.clone(), port)
        .tls(cert.clone())
        .hot_reload(true)
        .timeout(5000)
        .build();
}
