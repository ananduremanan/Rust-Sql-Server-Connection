pub struct Dog {
    pub name: String,
    pub age: i32,
}

impl Dog {
    pub fn say_something(&self) {
        println!(
            "Hey my name is {} and I'm {} years old, woof, woof...",
            self.name, self.age
        );
    }
}