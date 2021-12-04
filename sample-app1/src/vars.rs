pub mod sub_a;
pub mod sub_b;

const MAX_POINTS: u32 = 100_000;

pub fn run() {
    println!("Here is vars module!!");
    // sub_a::func_a();
    // sub_b::func_b();

    // mutと定義すると再代入ができるようになる。
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    // 使っていないと明示できる
    let _i1 = 3;
    let _f1 = 0.1;

    println!("{}", usize::BITS);
    println!("Memory address of const is: {:p}", &MAX_POINTS);

    // アドレス確認用
    let i2: i64 = 1;
    let i3: i64 = 2;

    // 8byteぶんずれてる
    println!("Stack address of i2 is: {:p}", &i2);
    println!("Stack address of i3 is: {:p}", &i3);

    // 新しくStackされていく
    let y = 5;
    println!("Stack address of y is: {:p}", &y);

    let y = y + 1;
    println!("Stack address of y is: {:p}", &y);

    let y = y * 2;
    println!("Stack address of y is: {:p}", &y);
    println!("The value of y is: {}", y);

    // ブロック内は別の変数として扱われる
    {
        let y = 0;
        println!("The value of y is: {}", y);
    }

    println!("The value of y is: {}", y);

    // Tupleサンプル
    let t1 = (500, 6.4, "dummy");
    let (x, y, z) = t1;
    println!("The value of t1 is: {} {} {}", t1.0, t1.1, t1.2);

    // ポインタの値も分割して取得したい場合はrefを使う
    // _で後半を省略できる
    let mut t2 = ((0, 1), (2, 3));
    let ((ref mut x1_ptr, ref mut y1_ptr ), _) = t2;

    // ポインタの参照値を変更(参照外し)
    *x1_ptr = 5;
    *y1_ptr = -5;

    // Tupleとか構造体の場合は:?を指定すると幸せになれる
    println!("{:?}", t2);

    // 配列サンプル
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [0; 10];
    println!("{:?} {:?} {} {}", a1, a2, a1[2], a1[3]);


}
