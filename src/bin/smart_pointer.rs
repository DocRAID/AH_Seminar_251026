use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex};
use std::{mem, thread};
fn main_box() {
    println!("--------Box<T>--------");
    let mut x = Box::new(10); // 힙에 10 저장
    *x += 10;
    println!("x = {}", x); //x = 20
    println!("----------------------");
} // 자동 해제

fn main_rc() {
    println!("--------Rc<T>--------");
    // 공유 데이터 생성
    let shared_data = Rc::new(vec![1, 2, 3]);
    println!("Reference count: {}", Rc::strong_count(&shared_data)); // 1

    {
        let _clone1 = Rc::clone(&shared_data);
        println!("Reference count: {}", Rc::strong_count(&shared_data)); // 2
        {
            let _clone2 = Rc::clone(&shared_data);
            println!("Reference count: {}", Rc::strong_count(&shared_data)); // 3
        }
        println!("Reference count: {}", Rc::strong_count(&shared_data)); // 2
    } // _clone1과 _clone2가 스코프를 벗어나면서 참조 카운트가 감소

    println!("Reference count: {}", Rc::strong_count(&shared_data)); // 1
    println!("---------------------");
}

fn main_arc() {
    println!("--------Arc<T>--------");
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
            println!("Thread {i}, total = {}", *num);
        });
        handles.push(handle);
    }

    for h in handles {
        h.join().unwrap();
    }
    println!("\ncounter = {:?}", counter.lock().unwrap());
    println!("----------------------");
}

fn main_refcell() {
    println!("------RefCell<T>------");
    let x = RefCell::new(5);
    *x.borrow_mut() = 10; // 가변 참조 대여 (런타임 검사)
    println!("x = {}", x.borrow());
    println!("----------------------");
}

fn main() {
    // 문제 상황: 여러 곳에서 호율적으로 같은 데이터를 공유하고 싶을 때
    {
        let data = String::from("important data");
        let user1 = data; // data의 소유권이 user1으로 이동
        // let user2 = data;  // 컴파일 에러! data는 이미 이동됨
    }

    // 메모리를 추가로 사용하는 경우
    {
        let data = String::from("important data");
        let user1 = data.clone();
        let user2 = data.clone();
        println!(
            "total memory usage = {:?}",
            mem::size_of_val(&data) + mem::size_of_val(&user1) + mem::size_of_val(&user2)
        );
    }

    // 포인터를 사용하는 경우
    {
        let data = Box::new(String::from("important data"));
        let user1 = data.clone();
        let user2 = data.clone();
        println!(
            "total memory usage = {:?}",
            mem::size_of_val(&data) + mem::size_of_val(&user1) + mem::size_of_val(&user2)
        );
    }

    //unsafe pointer
    let mut num = 10;
    let po: *mut i32 = &mut num;
    unsafe {
        *po = 20;
    }
    println!("num = {}", num);

    main_box();
    main_rc();
    main_arc();
    main_refcell();
}
// 연결 리스트를 만들고 싶다면?
struct List1 {
    value: i32, // 컴파일 에러. 무한 크기
                // next: List1
}

// 해결: Box를 사용한 포인터
struct List2 {
    value: i32,
    next: Box<List2>, // 포인터는 고정 크기
}
