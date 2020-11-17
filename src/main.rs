

#[derive(Debug)]
struct Person {
    name: String,
    age: i32
}

impl Person {
    fn new(name: String, age: i32) -> Person {
        Self {
            name,
            age
        }
    }

    fn get_older(&mut self) {
        self.age = self.age + 1;

    }
}

#[derive(Debug)]
struct Club {
    name: String,
    count: i64,
    members : Vec<Person>
}

impl Club {
    fn new(name: String) -> Self { 
        Self {
            name,
            count: 0,
            members: Vec::new()
          } 
    }

    fn add_member(&mut self, person: Person) {
            self.count =  self.count+1;
            self.members.push(person);
    }
}

fn main() {

    let mut p = Person::new("Kevin".to_string(), 21);
    p.get_older();

    let mut chess_club = Club::new("chess".to_string());

    chess_club.add_member(p);

    println!("Club details : {:#?}", chess_club);
}
