fn main() {
    println!("Hello, world!");

    let user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    }

    //値の変更をしたい場合は、user1(インスタンス)にmutをつけないといけない
    //特定のプロパティだけ可変にするとかも不可能
    user1.email = String::from("anotheremail@example.com");
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    let user = User {
        active: true,
        username: username,
        email: email,
        sign_in_count: 1
    };

    /*let user = User {
        active: true,
        username, //jsみたいな同盟の省略記法も可能
        email,
        sign_in_count: 1
    }*/

    //emailだけを独自に定義しつつ、残りのプロパティはuserのものを使う。もちろんuser.usernameみたいな書き方も可能。当然所有権は移る
    let user2 = User { 
        email: String::from("anotheremail@example.com"),
        ..user
    };

    user2

}

struct Color(i32, i32, i32);
struct Point(i32, i32, i32);
struct AlwaysEqual;
fn tuple_struct () {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
    let subject = AlwaysEqual;
}