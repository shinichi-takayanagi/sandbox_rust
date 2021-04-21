use std::sync::{Arc, Mutex};
use std::thread;
use sandbox_rust::module::a;

struct Person{
    name: String,
    age: u32,
}
// add method to Person
impl Person{
    fn name(&self){
        println!("I am {}", self.name);
    }
    fn age(self: &Person){
        println!("{}", self.age)
    }
}

//要するにInterfaceや
trait Tweet{
    fn tweet(&self);
    fn shout(&self){
        println!("Uhooooo!!");
    }
}
struct Dove;
struct Duck;
impl Tweet for Dove{
    fn tweet(&self){println!("Cool")}
}
impl Tweet for Duck{
    fn tweet(&self){println!("Coooooool")}
}

//一致チェック可能
// PartialXXXは反射律を満たさない（NaNとかのため）
#[derive(Eq, PartialEq)]
struct A(i32);
//比較可能
#[derive(PartialEq, PartialOrd)]
struct B(f32);


#[derive(Debug)] 
struct Droppable;
//デストラクタとしてのDrop traits
impl Drop for Droppable{
    fn drop(&mut self){
        println!("Resource will be released")
    }
}

fn try_mutex(){
    //スレッド, mutex
    let mut handles = Vec::new();
    let data = Arc::new(Mutex::new(vec![1; 10]));

    for x in 0..10 {
        let data_ref = data.clone();
        handles.push(thread::spawn(move || {
            let mut data = data_ref.lock().unwrap();
            data[x] += x;
        }));
    }
    for handle in handles{
        let _ = handle.join();
    }
    dbg!(data);
}

fn main() {
    println!("Hello, world!");
    let v = vec![1,2,3,3];
    for e in &v{
        println!("{}", e);
    }
    //ヒープにメモリ取るコンパイル時にサイズがわからん型を使う時
    let ba = [b'a', b'd'];
    print(Box::new(ba));
    // mutable v.s. immutable
    let x  =10;
    let mut mx  = 10;
    mx += x;
    println!("{}", mx);

    // <=と<が明示的に区別できる
    for n in 1..=4 {
        println!("{}", n);
    }
    for n in 1..4 {
        println!("{}", n);
    }
    // Personのinstance化
    let p = Person{name: String::from("hoge"), age: 111};
    p.name();
    p.age();
    //match
    let i = 2;
    match i {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("unknown")
    };
    // struct Comparison
    println!("{:?}", A(0) == A(1));
    println!("{:?}", B(0.0) <= B(1.0));
    
    //
    let dove = Dove{};
    dove.tweet();
    dove.shout();
    Duck{}.tweet();
    {
        let d = Droppable{};
        println!("{:#?}", d)
    }
    println!("The droppable should be released");

    //Mutex
    try_mutex();

    // Call the other module a
    println!("{}", a::add(1,1))
}

fn print(s: Box<[u8]>){
    println!("{:?}", s);
}