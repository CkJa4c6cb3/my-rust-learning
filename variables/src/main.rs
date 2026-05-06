fn main() {
    let x = 5;
    let x = x + 1; //全く新しい let xを生成しているので、コンパイルできる

    {
        let x = x * 2; //このスコープだけいける。
        println!("The value of x in the inner scope is: {x}");
    }

    println!("The value of x is: {x}");
    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}");

    let x: f32 = 36.55555;

    let calculate = 1 + 2 - 3 * 30 / 5 % 6;

    let b: bool = true;

    let c = 'a'; //一文字固定
    let s = "huga"; //文字列

    let tup: (i32, f64, u8) = (500, 6.4, 1);
    //let (o, p, q) = tup;
    let tup_1 = tup.0;
    let tup_2 = tup.1;
    let tup_3 = tup.2;

    let a = [1,2,2,3];
    let months = ["January", "February", "March", "April", "May", "June", "July",
              "August", "September", "October", "November", "December"];

    let a: [i32; 5] = [1, 2, 3, 4, 5];  
    let a = [3; 5];    // same as let a = [3, 3, 3, 3, 3]; 
    let first = a[0];
    let secound = a[1];

    //let error_index = a[9]; //index out of bounds: the len is 5 but the index is 9みたいなエラーが出るらしい

    let args = five();
    another_function(args);
}

fn another_function(x: i32) {
    println!("another_function argument is {x}");
    
}

fn five() -> i32 { // -> はreturn
    5
}
