use std::mem::*;

fn lifetime() {
    let r; // 아직 초기화 X
    {
        let x: i32 = 5;
        // r = &x;      //오류! r이 x를 빌리려 했으나 r보다 lifetime이 짧음
        r = x; //그래서 복사
    } // x는 여기서 drop됨
    println!("{}", r); // ??
}

fn function1() {
    println!("function1");
    let f_memory = "DATA1";
    println!("value of memory is {:?}", f_memory);
}

fn function2() {
    println!("function2");
    let f_memory = Box::new("DATA2");
    println!("value of memory is {:?}", f_memory);
    drop(f_memory);
    // !!ERROR HERE!!
    //println!("value of memory is {:?}", f_memory);

    let girl = "man";

    let man = "a";
}

fn function3() -> Box<&'static str> {
    println!("function3");
    let f_memory = Box::new("DATA3");
    println!("value of memory is {:?}", f_memory);
    f_memory
}
fn main() {
    let value = 12345;
    println!("{:?}", value);
    function1();
    // !!ERROR HERE!!
    //println!("value of memory is {}", f_memory.value());
    function2();
    let value = function3();
    println!("value of value is {:?}", value);
}
