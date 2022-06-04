pub fn enums_match_and_option() {
    fn plus_one_with_option(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    #[derive(Debug)]
    enum UsState {
        State1,
        State2,
    }

    enum Coin {
        Penny,
        Nickel,
        Dime,
        Quarter(UsState),
    }

    fn is_quarter(coin: Coin) {
        if let Coin::Quarter(state) = coin {
            println!("Got a quarter from {:?}", state);
        }
    }

    fn value_in_cents(coin: Coin) -> u8 {
        match coin {
            Coin::Penny => {
                println!("Lucky penny!");
                1
            }
            Coin::Nickel => 5,
            Coin::Dime => 10,
            Coin::Quarter(state) => {
                println!("A quareter from {:?} ", state);
                25
            }
        }
    }

    fn route(_ip_kind: IpAddrKind) {}

    enum IpAddrKind {
        V4,
        V6,
    }

    struct IpAddrMy {
        kind: IpAddrKind,
        address: String,
    }

    enum IpAddrSimple {
        V4(String),
        V6(String),
    }

    let _home = IpAddrMy {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    let _loopback = IpAddrSimple::V6(String::from("::1"));
}
