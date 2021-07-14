trait Animal {
    fn call() -> String;
}

struct Test{
    name: String,
}

impl Test {
    pub fn new() -> Test {
        Test{
            name: String::from("")
        }
    }

    pub fn test(&self) {
        println!("{:?}",self.name);
    }
}

impl Animal for Test {
    fn call() -> String {
        String::new()
    }
}