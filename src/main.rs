use std::borrow::Cow;

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
        ],
    };

    increment_age(&mut flex_human);
    println!("Flexible human new age: {}", flex_human.age);
}
