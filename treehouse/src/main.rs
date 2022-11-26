use std::io::stdin;

#[derive(Debug)]
struct Visitor{
    name: String,
    action: VisitorAction,
    age: i8
}

impl Visitor {
    fn new(name: &str, action: VisitorAction, age: i8) -> Self {
        Self {
            name: name.to_lowercase(),
            action,
            age
        }
    }

    fn greet_visitor(&self){
        match &self.action {
            VisitorAction::Accept => println!("Welcome to the treehouse {}!", self.name),
            VisitorAction::AcceptWithNote { note } => {
                println!("Welcome to the treehouse {}!", self.name);
                println!("{}", note);
                if self.age < 21{
                    println!("Don't let {} get near the pop.", self.name);
                }
            }
            VisitorAction::Probation => println!("{} is a probationary member.", self.name),
            VisitorAction::Refuse => println!("Don't let {} in!", self.name),
        }
    }
}


fn what_is_your_name() -> String {
    let mut your_name = String::new();
    stdin()
        .read_line(&mut your_name)
        .expect("Failed to read line");
        your_name
            .trim()
            .to_lowercase()
}

#[derive(Debug)]
enum VisitorAction {
    Accept,
    AcceptWithNote { note: String},
    Refuse,
    Probation,
}

fn main() {
    
    let mut visitor_list = vec![
        Visitor::new("Bert", VisitorAction::Accept, 45),
        Visitor::new("Steve", VisitorAction::AcceptWithNote { note: String::from("Lactose-free milk is in the fridge.") }, 15),
        Visitor::new("Bob", VisitorAction::Refuse, 30),
    ];

    loop {
        println!("Hello, what's your name?");
        let name = what_is_your_name();
        let known_visitor = visitor_list.iter().find(| visitor | visitor.name == name);
        match known_visitor {
            Some(visitor) => visitor.greet_visitor(),
            None => {
                if name.is_empty() {
                    break;
                } else {
                    println!("{} is not on the visitor list.", name);
                    visitor_list.push(Visitor::new(&name, VisitorAction::Probation, 0));
                }
            }
        }
        break
    }

    println!("The final list of visitors is: ");
    println!("{:#?}", visitor_list);

}
