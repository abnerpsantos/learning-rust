fn main() {

    // defining_enum();
    storing_data_in_enum();
}

#[allow(dead_code)]
fn defining_enum() {
    println!("While structs give you a way of grouping related fiedls and data");
    println!("Enums give you a way of say a value is one of all posible values");

    /*
    Let's look at a situation with IP addressess
    IP's can be V4 or V6, with enums it's easy to define it's data
     */

    #[derive(Debug)]
    enum IpAddrKind {
        V4,
        V6
    }

    let _ip_v4: IpAddrKind = IpAddrKind::V4;
    let _ip_v6: IpAddrKind = IpAddrKind::V6;

    /*
    both variables are typed as IpAddrKind, so if a function receive it's data type, both variables can be used.
     */

    fn get_ip(ip: IpAddrKind) {
        println!("{:?}", ip)
    }

    get_ip(_ip_v4);
    get_ip(_ip_v6);
}

#[allow(dead_code)]
fn storing_data_in_enum() {
    println!("We can store data in structs as we already know...");
    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddr {
        kind: IpAddrKind,
        address: String,
    }

    let _home: IpAddr = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let _loopback: IpAddr = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };

    println!("but we can achieve the same behavior with enums:");

    #[derive(Debug)]
    enum IpAddrKindData {
        V4(String),
        V6(String)
    }

    let home: IpAddrKindData = IpAddrKindData::V4(String::from("127.0.0.1"));
    let loopback: IpAddrKindData = IpAddrKindData::V6(String::from("::1"));

    println!("{:?}", home);
    println!("{:?}", loopback)
}