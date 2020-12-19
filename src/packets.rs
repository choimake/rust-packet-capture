use pnet::packet::ipv4::Ipv4Packet;
use pnet::packet::ipv6::Ipv6Packet;
use pnet::packet::tcp::TcpPacket;
use pnet::packet::udp::UdpPacket;
use pnet::packet::Packet;

// trait
// url: https://doc.rust-jp.rs/rust-by-example-ja/trait.html
pub trait GettableEndPoints {
    fn get_source(&self) -> String;
    fn get_destination(&self) -> String;
    fn get_payload(&self) -> &[u8];
}

// impl<'a>の<'a>の意味がわからん
// とりあえずlifetime(寿命）のことを示しているらしい
// lifetime annotationという概念があるそうな
// url: https://doc.rust-lang.org/stable/book/ch10-03-lifetime-syntax.html#lifetime-annotation-syntax
// ドキュメントだとここに書かれている
// url: https://doc.rust-jp.rs/rust-by-example-ja/scope/lifetime/explicit.html
// 'aはラベル的なノリで捉えておけばいいのかな？スコープを示すラベル
impl<'a> GettableEndPoints for Ipv4Packet<'a> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }
    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }
    fn get_payload(&self) -> &[u8] {
        self.payload()
    }
}
impl<'a> GettableEndPoints for Ipv6Packet<'a> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }
    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }
    fn get_payload(&self) -> &[u8] {
        self.payload()
    }
}

impl<'a> GettableEndPoints for TcpPacket<'a> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self) -> &[u8] {
        self.payload()
    }
}

impl<'a> GettableEndPoints for UdpPacket<'a> {
    fn get_source(&self) -> String {
        self.get_source().to_string()
    }

    fn get_destination(&self) -> String {
        self.get_destination().to_string()
    }

    fn get_payload(&self) -> &[u8] {
        self.payload()
    }
}
