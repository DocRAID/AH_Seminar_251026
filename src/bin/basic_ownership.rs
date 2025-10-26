//소유권 이동
fn main1() {
    let s1 = String::from("hello");
    // let s2 = s1; // s1의 소유권이 s2로 이동
    let s2 = s1.clone();
    println!("{}", s1); // 에러! s1은 더 이상 유효하지 않음
}

//대여
fn main2() {
    let s = String::from("hi");
    print_length(&s); // 불변 참조(대여)
    println!("{}", s); // 여전히 사용 가능
}

fn print_length(str_ref: &String) {
    println!("len = {}", str_ref.len());
}

//대여 후 예견된 조작 및 함수로부터 새 소유권 할당
fn main3() {
    let mut s = String::from("hi");
    add_and_print_length1(&mut s); // 불변 참조(대여)
    println!("{}", s); // 여전히 사용 가능
    let result = add_and_print_length2(s); //s 는 소유권이 넘어감으로써 사용 불가
    println!("{}", result);
}

fn add_and_print_length1(str_ref: &mut String) {
    str_ref.push_str(" and bye");
    println!("len = {}", str_ref.len());
}

fn add_and_print_length2(str_ref: String) -> String {
    let mut result_str = str_ref;
    result_str.push_str(" and bye");
    println!("len = {}", result_str.len());
    result_str
}

fn main() {
    // main1();
    // main2();
    main3();
}
