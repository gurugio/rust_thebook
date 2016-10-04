struct Philosopher {
    name: String,
}

impl Philosopher {
    fn new(name: &str) -> Philosopher {
        Philosopher {
            name: name.to_string(),
        }
    }
}


fn main() {
    let p1 = Philosopher::new("butler");
    let p2 = Philosopher::new("p2");
    let p3 = Philosopher::new("p3");
    let p4 = Philosopher::new("p4");
    let p5 = Philosopher::new("p5");
}
