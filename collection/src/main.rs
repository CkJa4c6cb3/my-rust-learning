fn main() {
    let v: Vec<i32> = Vec::new();

    let v = vec![1, 2, 3];

    let mut mv = Vec::new();

    mv.push(5);
    mv.push(6);
    mv.push(7);

    let third: &i32 = &mv[2];

    let third: Option<&i32> = v.get(2);

    match third {
        Some(third) => println!("third"),
        None => println!("none"),
    }

    let v = vec![1, 2, 3, 4, 5];

    let does_not_exist = &v[100]; //こっちだと存在しない場合はエラーになる
    let does_not_exist = v.get(100); //こっちだとNoneを返してくれる

    //ただ読み取るだけなので、&
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{i}");
    }

    //ループで加工する場合は、mutが必要
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}

enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn enum_vector() -> Vec<SpreadsheetCell> {
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
    row
}
