fn main() {
    let width1 = 30;
    let height1 = 50;

    println!(
        // 長方形の面積は、{}平方ピクセルです
        "The area of the rectangle is {} square pixels.",
        area(width1, height1)
    );

    let rect_tuple = (30, 50);
    println!(
        "The area of the rectangle is {} square pixels.",
        area2(rect_tuple)
    );

    let rect_struct = Rectangle {
        width: 30,
        height: 50,
    };

    println!(
        "The area of the rectangle is {} square pixels.",
        area3(&rect_struct)
    );

    println!("rect_struct is {:?}", rect_struct);
    rect_struct.area();
}

//これでもいいけど、引数の役割がほぼ同じ。分ける意味がない
fn area(width: u32, height: u32) -> u32 {
    width * height
}

//これでグループ化できる but 引数の名前がわからない
fn area2(dimensions: (u32, u32)) -> u32 {
    dimensions.0 * dimensions.1
}

#[derive(Debug)] //これがないとデバッグできないらしい
struct Rectangle {
    width: u32,
    height: u32,
}
//Rectangleの実行部分
impl Rectangle {
    fn area(&self) -> u32 {
        //selfはインスタンス自体っぽい
        self.width * self.height
    }

    //self以外の物も受け取れる。読み取る時は基本参照っぽい
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}
//implは複数つくれる。オーバーロードかな？
impl Rectangle {
    fn can_hold2(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
}

fn area3(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
