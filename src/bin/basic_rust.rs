fn declaration() {
    let mut intager: i32 = 1; // int 32bit
    intager = 2;
    let unsigned_int: u32 = 10; // unsigned int 32bit
    let long_int: i64; // int 64bit
    let float: f32; //float 32bit

    let str: &str = "some String"; //static_string
    let string: String = "hello".to_string(); //dynamic_string
    let char: char = 'c'; //char
    let boolean: bool = true; //boolean
}

struct CacheStruct {
    key: i32,
    value: String,
}

fn struct_example() {
    let c1: CacheStruct = CacheStruct {
        key: 0,
        value: "first".to_string(),
    };

    assert_eq!(c1.key, 0);
    assert_eq!(c1.value, "first");
}

fn mutable_example() {
    let mut value = 12345;
    value = 10;
    assert_eq!(value, 10);
}

fn return_function(foo: i32) -> String {
    // return format!("input number is : {}",foo);
    format!("input number is : {}", foo)
}

fn optional_value() {
    let option1: Option<i32> = Some(5);
    let option2: Option<i32> = None;

    println!("{:?}", option1);
    println!("{:?}", option2);

    match option1 {
        Some(x) => println!("{}", x),
        None => println!("nothing"),
    }

    match option2 {
        Some(x) => println!("{}", x),
        None => println!("nothing"),
    }
}
fn main() {
    println!("{}", return_function(32));
}
