// В Rust можно разделить данные на два типа хранения. Первые это stack-allocated данные размер
// которых известен заранее. Такие данные храняться в стеке и при присвоение таких данных сначала
// одной переменной а потом присвоение этой переменной другой переменной данные копируются в стеке.
// Второй тип это heap-allocated их точный размер не известен заранее, а значит в памяти нужно
// заранее выделить место под их хранение. Такое возможно только в куче. Такие данные не
// копируются, права на их владение передаются от одной переменной к другой. Причём первая права
// на владение теряет и становиться недоступна.

pub fn run_examples() {
    let s1 = String::from("hello");
    let s2 = s1;

    // println!("{s1}, world!")
    // This code causes error "borrow of moved value"

    let x = 5;
    let y = x;
    println!("x = {x}, y = {y}");
    // This one works fine
    ownership_and_functions();
}

fn ownership_and_functions() {
    let s = String::from("hello"); // s comes into scope

    takes_ownership(s); // s's value moves into the function...
    // ... and so is no longer valid here
    // println!("{s}"); Error!

    let x = 5; // x comes into scope

    makes_copy(x); // Because i32 implements the Copy trait,
    // x does NOT move into the function,
    // so it's okay to use x afterward.
    println!("{x}");
} // Here, x goes out of scope, then s. However, because s's value was moved,
// nothing special happens.

fn takes_ownership(some_string: String) {
    // some_string comes into scope
    println!("{some_string}");
} // Here, some_string goes out of scope and `drop` is called. The backing
// memory is freed.

fn makes_copy(some_integer: i32) {
    // some_integer comes into scope
    println!("{some_integer}");
} // Here, some_integer goes out of scope. Nothing special happens.
