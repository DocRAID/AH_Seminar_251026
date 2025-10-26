fn lifetime() {
    let r; // 아직 초기화 X
    {
        let x: i32 = 5;
        // r = &x;      //오류! r이 x를 빌리려 했으나 r보다 lifetime이 짧음
        r = x; //그래서 복사
    } // x는 여기서 drop됨
    println!("{}", r);
}

fn main() {
    lifetime();
}
