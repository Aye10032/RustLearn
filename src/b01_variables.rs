pub fn variables(){
    // 变量
    let mut  x = 5;
    println!("The value of x is: {x}");

    x = 6;
    println!("The value of x is: {x}");

    let a =6;

    {
        let a = 12;
        println!("The value of a in scope is: {a}");
    }

    println!("The value of a is: {a}");
}