use std::{collections::HashMap, io};

fn main() {
    const GAME_END_WORD: &str = "end";

    let mut employee_list: HashMap<String, Vec<String>> = HashMap::new();

    loop {
        let mut department = String::new();
        let mut employee_name = String::new();

        println!("部門を入力してください");
        io::stdin().read_line(&mut department).expect("failed");
        let department = department.trim().to_string();

        if department == GAME_END_WORD {
            break;
        }

        println!("従業員名を入力してください");
        io::stdin().read_line(&mut employee_name).expect("failed"); //read_lineはどんどん追記される
        let employee_name = employee_name.trim().to_string(); //trimで&strになるから。immutable, Stringがmutable

        match employee_interface(department, employee_name, &mut employee_list) {
            Ok(_) => println!("追加しました"),
            Err(err) => println!("error: {}", err),
        }

        for employees in employee_list.values_mut() {
            employees.sort();
        }

        println!("--- 部署一覧 ---");

        for (department, employees) in &employee_list {
            println!("{}: {:?}", department, employees);
        }
    }
}

/*ハッシュマップとベクタを使用して、ユーザに会社の部署に雇用者の名前を追加させられるテキストインターフェイスを作ってください。
例えば、"Add Sally to Engineering"(開発部門にサリーを追加)や"Add Amir to Sales"(販売部門にアミールを追加)などです。
れからユーザに、ある部署にいる人間の一覧や部署ごとにアルファベット順で並べ替えられた会社の全人間の一覧を扱わせてあげてください。*/
fn employee_interface(
    department: String,
    employee_name: String,
    employee_list: &mut HashMap<String, Vec<String>>,
) -> Result<(), String> {
    if department.trim().is_empty() {
        return Err(String::from("department is empty"));
    }

    if employee_name.trim().is_empty() {
        return Err(String::from("employee name is empty"));
    }

    employee_list
        .entry(department)
        .or_insert(Vec::new())
        .push(employee_name);

    Ok(())
}
