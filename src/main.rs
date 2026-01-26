struct Book<'a> {
    name: &'a str,
    age: i32,
}

fn another_year(book: &mut Book) {
    book.age = book.age + 1;
}


fn main() {
    let mut ceramic_stones = Book{name: "Aa", age: 6};
    println!("Book age is {}", ceramic_stones.age);
    another_year(&mut ceramic_stones);
    println!("New book age is {}", ceramic_stones.age);
}
