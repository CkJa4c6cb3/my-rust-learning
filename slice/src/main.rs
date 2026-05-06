use std::io::Stderr;

fn main() {
    let mut s = String::from("hello world");

    let word = first_word(&s);

    s.clear();
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes(); //stringをbytes配列に変換
    println!("{:?}", &bytes);

    //bytesからイテレータを生成、iterが各要素を返す enumerateがタプルを生成((0, &97),(1, &98),(2, &32)な感じ
    //iはloopのindex, itemがそれぞれの要素
    for (i, &item) in bytes.iter().enumerate() {
        println!("{:?}", item);
        println!("{}", i);
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

fn string_slice() {
    let s = String::from("hello");

    let len = s.len();

    //初めから特定行
    let slice = &[0..2];
    let slice = &[..2];

    //途中から終わり
    let slice = &[3..len];
    let slice = &[3..];

    //全体
    let slice = &[0..len];
    let slice = &[..];
}
