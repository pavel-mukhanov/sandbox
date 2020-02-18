use libp2p::{tcp::TcpConfig, Multiaddr, Transport};
use structopt::StructOpt;
use tokio;

#[derive(Debug, StructOpt)]
#[structopt(name = "Node")]
struct Opt {
    /// Dial port
    #[structopt(short = "d", long = "dial_port", default_value = "0")]
    dial_port: u16,

    /// Listen port port
    #[structopt(short = "l", long = "listen_port", default_value = "0")]
    listen_port: u16,
}

fn main() {
    println!("starting node...");
    let opt = Opt::from_args();

    let tcp = TcpConfig::new();

    let base_address = "/ip4/127.0.0.1/tcp/";

    if opt.dial_port > 0 {
        let addr: Multiaddr = format!("{}{}", base_address, opt.dial_port)
            .parse()
            .expect("invalid multiaddr");
        let _conn = tcp.clone().dial(addr);
    }

    if opt.listen_port > 0 {
        let addr: Multiaddr = format!("{}{}", base_address, opt.listen_port)
            .parse()
            .expect("invalid multiaddr");
        let conn = tcp.listen_on(addr);

        //        tokio::run(conn.unwrap());
    }
}
