fn main() {

    struct Person {
        name: String,
        age: u32,
    }

    let p = Person {
        name: String::from("John"),
        age: 8
    };

    println!("構造体チェック: 名前:{} 年齢:{}歳", p.name, p.age);
}
