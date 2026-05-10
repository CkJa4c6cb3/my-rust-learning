use std::{
    fs::{self, File},
    io::{self, ErrorKind, Read},
};

fn main() {
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}

fn unwrap_example() {
    //unwrap()はResult型からOKのものを取り出す。ErrorだとPanicになる
    let greeting_file = File::open("hello.txt").unwrap();

    //expect() catchみたいなもん。エラーが来ることを予測している時に使う。panicを防止してくれる。
    let greeting_file = File::open("hello.txt").expect("no file found bitch!");
}

//エラーの移譲。throwsに近いかもしれない。成功の時はResultのString、失敗の時はErrorを返す
fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");

    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut username = String::new();

    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}
//簡単な書き方バージョン ?をつけると、matchなしでもerrorを返してくれる。
fn read_username_from_file_easy() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

//もっと簡単な書き方。標準機能で存在
fn read_username_from_file_with_library() -> Result<String, io::Error> {
    fs:read_to_string("hello.txt")
}

//?演算子はResult, Option等の対応しているやつでしか使えない

//panic!とResultの使い分け。プログラムで制御しきれないものはpanic!(例：外部APIのレスポンス, 配列以上の要素アクセス)
//コードの修正等で回復可能にする場合はResult。分かりきっているものもResult