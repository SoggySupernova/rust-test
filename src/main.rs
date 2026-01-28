use std::borrow::Cow;

use ratatui::symbols;
pub mod tui;
trait HasAge {
    fn increment_age(&mut self);
}


#[derive(Debug)]
struct Human {
    name: String,
    age: u8,
}


#[derive(Debug)]
struct FlexibleHuman<'a> {
    name: Cow<'a, str>,
    age: u8,
    arbitrary_data: Vec<(String, String)>,
}

impl HasAge for Human {
    fn increment_age(&mut self) {
        self.age += 1;
    }
}

impl HasAge for FlexibleHuman<'_> {
    fn increment_age(&mut self) {
        self.age += 1;
    }
}

fn increment_age<T: HasAge>(entity: &mut T) {
    entity.increment_age();
}

fn main() {
    let mut child = Human{name: String::from("Aa"), age: 6};
    println!("Child's name is {}", child.name);
    println!("Age is {}", child.age);
    increment_age(&mut child);
    println!("New age is {}", child.age);
    println!("Entire struct: {:?}", child);
    println!("Entire struct with pretty debug: {:#?}", child);
    let mut flex_human = FlexibleHuman {
        name: Cow::from("Bb"),
        age: 30,
        arbitrary_data: vec![
            (String::from("Hobby"), String::from("Reading")),
            (String::from("City"), String::from("New York")),
            (String::from("Birthplace"), String::from("Los Angeles")),
        ],
    };

    increment_age(&mut flex_human);
    println!("Flexible human new age: {}", flex_human.age);
    println!("Flexible human entire struct: {:?}", flex_human);
    println!("Flexible human entire struct with pretty debug: {:#?}", flex_human);
    println!("Flexible human hobby: {}", flex_human.arbitrary_data[0].1);
    let city_equals_birthplace = flex_human.arbitrary_data[1].1 == flex_human.arbitrary_data[2].1;
    println!("City equals Birthplace: {}", city_equals_birthplace);
    tui::main(
        String::from(
            format!(
                "Hello, {0}! You are {1} years old.\nYou live in {2} and were born in {3}.",
                flex_human.name,
                flex_human.age,
                flex_human.arbitrary_data[1].1,
                flex_human.arbitrary_data[2].1
            )
        ),
        String::from(" Flexible Human created "),
        symbols::border::ROUNDED
    ).unwrap();
}
