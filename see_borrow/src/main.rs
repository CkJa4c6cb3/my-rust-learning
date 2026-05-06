fn main() {
    let s1 = String::from("hello");

    let len = calculate_length(&s1); //&は参照するという意味。所有権は移らない

    println!("the length of '{}' is {}.", s1, len);
    let hello = String::from("hello");
    change(&hello);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &String) {
    some_string.push_str("string"); //参照したものを変更するのは不可能。
}

//これはOK。参照する時を変更する場合は、引数時点で可変であることを宣言しないといけない
fn mut_change(some_string: &mut String) {
    some_string.push_str("string");
}

fn double_borrow() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    let r2 = &mut s; //NG：r1がこの時点だと参照の権利を持っているから、解放しないと使えない
    println!("{}, {}", r1, r2);

    let mut b = String::from("byebye");
    let b1 = &mut b;
    println!("{b1}"); //こっちはOK。ここでb1はdropするから
    let b2 = &mut b;
    println!("{b2}");

    let mut s = String::from("hello");

    let r1 = &s; // 問題なし
    let r2 = &s; // 問題なし
    let r3 = &mut s; // 大問題！　不変なものに対して、可変にするのは無理

    let mut b = String::from("byebye");
    let s1 = &b; // 問題なし
    let s2 = &b; // 問題なし
    println!("{s1}, {s2}");
    let s3 = &mut b; // これはOK。printでs1,s2がdropされたから
}
