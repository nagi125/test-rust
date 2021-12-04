fn main() {
    let s1 = "こんにちは";
    let s2 = "hello";

    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    println!("static memory address of s2 is: {:?}", s1.as_ptr());
    println!("static memory address of s2 is: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());

    // 動的に文字列を増やしたい場合はstring型を使う
    let mut s1 = String::from("hello");
    let mut s2 = String::from("helloworld");
    println!("Stack address of s1 is: {:p}", &s1);
    println!("Stack address of s2 is: {:p}", &s2);
    println!("Heap memory address of s2 is: {:?}", s1.as_ptr());
    println!("heap memory address of s2 is: {:?}", s2.as_ptr());
    println!("Len of s1 is: {}", s1.len());
    println!("Len of s2 is: {}", s2.len());
    println!("Capacity of s1 is: {}", s1.capacity());
    println!("Capacity of s2 is: {}", s2.capacity());

    s1.push_str("_new1");
    s2.push_str("_new2");

    println!("{} {}", s1, s2);

}
