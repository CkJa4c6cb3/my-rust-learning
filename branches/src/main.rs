fn main() {
    let number = 99;

    if number < 5 {
        println!("condition was true");
    } else if number % 99 == 0 {
        println!("else if condition");
    } else {
        println!("condition was false");
    }

    //jsだとOkな書き方はRustだとNG
    /*if number {
        println!("number was 3");
    }*/

    if number != 0 {
        println!("its ok");
    }

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("the value of number is {number}");

    loop_test();
    counting_label();
    while_test();
    array_loop();
    for_loop();
}

fn loop_test() {
    let mut counter = 0;
    loop {
        counter += 1;
        println!("again");

        if counter == 99 {
            break;
        }
    }

    println!("counter is over {counter}");
}

fn counting_label() {
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");
}

fn while_test() {
    let mut number = 3;

    while number != 0 {
        println!("{number}");

        number -= 1;
    }

    println!("take off");
}

fn array_loop() {
    let a = [10, 20, 30, 40, 50, 60];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }
}

fn for_loop() {
    //所謂拡張for文
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    for number in (1..4).rev() {
        println!("{number}");
    }
}
