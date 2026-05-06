fn main() {
    // コピーや参照NG メモリのサイズが確定しないから
    let s1 = String::from("hello");
    //let s2 = s1; //NG
    let s2 = s1.clone(); //OK

    println!("{}, world!", s1);
    println!("{}, world!", s2);

    // OK　メモリのサイズがコンパイル前から確定しているから
    // 整数、book, float, char tuple(複合はNG)はコピー可能
    let x = 5;
    let y = x;

    let s = String::from("hello");

    takes_ownership(s); //ここでsのスコープは終わる。変数の所有者はtakes_ownershipに移る
    println!("{s}"); //なのでここでエラーになる

    let five = 5;
    makes_copy(five); //コピー可能なので、呼び出し後もfiveは呼べる
    println!("{five}");
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

//所有権の移動

fn move_ownership() {
    let s1 = gives_ownership(); //所有者はs1

    let s2 = String::from("hello"); //所有者はs2

    let s3 = takes_and_gives_back(s2); //s2の所有権はs3に

    //以降全ての変数はdropされる
}

fn gives_ownership() -> String {
    let some_string = String::from("yours");

    some_string // returnの時はセミコロンがいらない
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string
}
