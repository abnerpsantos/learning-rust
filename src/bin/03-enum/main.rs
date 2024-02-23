fn main() {

    defining_enum();
}

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