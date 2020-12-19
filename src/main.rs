extern crate pnet;

use pnet::datalink;
use pnet::datalink::Channel::Ethernet;
use pnet::packet::ethernet::EtherTypes;
use pnet::packet::ethernet::EthernetPacket;
use pnet::packet::ip::IpNextHeaderProtocols;
use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;
use std::env;

mod packets;
use packets::GettableEndPoints;

#[macro_use]
extern crate log;

fn main() {
    env::set_var("RUST_LOG", "debug");
    env_logger::init();
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        error!("Please specify target interface name");
        std::process::exit(1);
    }
    let interface_name = &args[1];

    // interfaceの選択
    let interfaces = datalink::interfaces();
    // expect()は値がSomeだった場合はその中身を取り出し、そうでない場合は引数に渡された文字列を表示してpanicを起こす
    // https://qiita.com/garkimasera/items/f39d2900f20c90d13259
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.name == *interface_name)
        .expect("Failed to get interface");

    // データリンクのchannelを取得
    let (_tx, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(Ethernet(tx, rx)) => (tx, rx),
        Ok(_) => panic!("Unhandled channel type"),
        Err(e) => panic!("Failed to create datalink channel {}", e),
    };

    loop {
        match rx.next() {
            Ok(frame) => {
                // unwrapはOkなら中の値を、Errならpanicを返す
                // url: https://qiita.com/nirasan/items/321e7cc42e0e0f238254#unwrap-%E3%83%A1%E3%82%BD%E3%83%83%E3%83%89
                let frame = EthernetPacket::new(frame).unwrap();
                match frame.get_ethertype() {
                    EtherTypes::Ipv4 => ipv4_handler(&frame),
                    EtherTypes::Ipv6 => ipv6_handler(&frame),
                    _ => {
                        info!("Not an Ipv4 or Ipv6 packet");
                    }
                }
            }
            Err(e) => {
                error!("Failed to read: {}", e);
            }
        }
    }
}

fn ipv4_handler(ethernet: &EthernetPacket) {
    // if letは列挙型をmatchする時に使う書き方
    // url: https://doc.rust-jp.rs/rust-by-example-ja/flow_control/if_let.html
    if let Some(packet) = Ipv4Packet::new(ethernet.payload()) {
        match packet.get_next_level_protocol() {
            IpNextHeaderProtocols::Tcp => tcp_handler(&packet),
            IpNextHeaderProtocols::Udp => udp_handler(&packet),
            _ => {
                info!("Not a Tcp or Udp packet");
            }
        }
    }
}
fn ipv6_handler(ethernet: &EthernetPacket) {
    if let Some(packet) = Ipv6Packet::new(ethernet.payload()) {
        match packet.get_next_header() {
            IpNextHeaderProtocols::Tcp => tcp_handler(&packet),
            IpNextHeaderProtocols::Udp => udp_handler(&packet),
            _ => {
                info!("Not a Tcp or Udp packet");
            }
        }
    }
}

// traitオブジェクトにはdynをつけるのがお作法
// ※つけないとwarningがでる
fn tcp_handler(packet: &dyn GettableEndPoints) {
    let tcp = TcpPacket::new(packet.get_payload());
    if let Some(tcp) = tcp {
        print_packet_info(packet, &tcp, "TCP");
    }
}

fn udp_handler(packet: &dyn GettableEndPoints) {
    let udp = UdpPacket::new(packet.get_payload());
    if let Some(udp) = udp {
        print_packet_info(packet, &udp, "TCP");
    }
}

fn print_packet_info(l3: &dyn GettableEndPoints, l4: &dyn GettableEndPoints, proto: &str) {
    println!(
        "Captured a {} packet from {}|{} to {}|{} \n",
        proto,
        l3.get_source(),
        l4.get_source(),
        l3.get_destination(),
        l4.get_destination()
    );
    let payload = l4.get_payload();
    let len = payload.len();
    const WIDTH: usize = 20;
    for i in 0..len {
        print!("{:<02X} ", payload[i]);
        if i % WIDTH == WIDTH - 1 || i == len - 1 {
            for _j in 0..WIDTH - 1 - (i % WIDTH) {
                print!("   ");
            }
            print!("| ");
            // 「..=i」の部分は何かの特殊な構文？
            // 1..=100 で 1~100という意味
            // url: https://doc.rust-jp.rs/rust-by-example-ja/flow_control/for.html
            for j in i - i % WIDTH..=i {
                if payload[j].is_ascii_alphabetic() {
                    print!("{}", payload[j] as char);
                } else {
                    // 非ascii文字は.で表示
                    print!(".");
                }
            }
        }
    }
    println!("{}", "=".repeat(WIDTH * 3));
    println!();
}
