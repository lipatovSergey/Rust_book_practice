pub fn run_examples() {
    example_1();
    example_2();
}
// Из-за того что heap_allocated данные передаются а не копируются, то без дополнительного
// механизма каждый раз когда передаваемые значение были бы необходимы далее в коде, то пришлось бы
// каждый раз возвращать их из функции в которую они были переданны

fn example_1() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{s2}' is {len}.");

    fn calculate_length(s: String) -> (String, usize) {
        let length = s.len(); // len() returns the length of a String

        (s, length)
    }
}

// Для таких случаев Rust используются ссылки. Для обозначения ссылок используются амперсанд.
// Обрати внимание, что амперсанд добавляется и к типу дынных, указывая, что это ссылка на
// строку а не строка.
fn example_2() {
    let s1 = String::from("hello");

    let len = calculate_length_1(&s1);

    println!("The length of '{s1}' is {len}.");

    fn calculate_length_1(s: &String) -> usize {
        s.len()
    } // Here, s goes out of scope. But because s does not have ownership of what
    // it refers to, the String is not dropped.
}
