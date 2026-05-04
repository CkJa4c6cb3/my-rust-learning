use std::io; //ioライブラリ stdは標準ライブラリ　use = import
use std::cmp::Ordering;
use rand::Rng;

fn main() {
    println!("Guess the number!");

    let secret_number = rand::rng().random_range(1..=100);
    //let apples = 5; //変数宣言　デフォルトだとimmutable
    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new(); // mut(mutable)をつけると、変更可能に　String::new()でString型のインスタンスを生成
        io::stdin()
            .read_line(&mut guess) //& 参照を表す &mutでmutableの参照。参照する場合も不変がデフォルトっぽい
            .expect("Failed to read line"); //Result型をキャッチして受け取れる。これを書かないとエラーを考慮していないというcomplineエラーが出る
    
        //なんかすごい変更なら変数を同名にできるらしいよ u32はu32型。なんだそれ
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, //_はスルーって意味
        } 
    
        println!("You guessed: {guess}");
    
        //cargo doc --openでクラス内の使用関数のドキュメントを全部作ってくれる。
    
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small"),
            Ordering::Greater => println!("too big"),
            Ordering::Equal => {
                println!("you win");
                break;
            }
        }
    }
    println!("ggwp");
}
