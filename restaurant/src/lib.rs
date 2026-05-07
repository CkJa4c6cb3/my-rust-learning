//modはmoduleであることを宣言

mod front_of_house {
    fn deliver_order() {}

    //所属するhosting module
    //moduleおよびそこに所属する関数はデフォルトでprivate, pubをつけて明示的に公開しないといけない
    pub mod hosting {
        pub fn add_to_waitlist() {
            super::deliver_order(); //楽に引用できる
            seat_at_table();
        }

        pub fn seat_at_table() {}
    }

    //serving module
    mod serving {
        fn take_order() {}

        fn serve_order() {}

        fn take_payment() {}
    }
}

pub fn eat_at_restaurant() {
    //絶対パス
    crate::front_of_house::hosting::add_to_waitlist();

    //相対パス
    front_of_house::hosting::add_to_waitlist();
}

/*
こんな感じの構造になる
crate
 └── front_of_house
     ├── hosting
     │   ├── add_to_waitlist
     │   └── seat_at_table
     └── serving
         ├── take_order
         ├── serve_order
         └── take_payment
 */

mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {}
    }
}

use crate::front_of_house::hosting; //要するにimport

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
}

use std::fmt::Result;
use std::io::Result as IoResult; //aliasもいける

fn function1() -> Result {
    // --snip--
}

fn function2() -> IoResult<()> {
    // --snip--
}
