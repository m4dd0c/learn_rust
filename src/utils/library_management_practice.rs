fn main () {

    // book creation
    let book = create_a_book();
    
    // creating a new member with some book content slice
    let mut mem = Member::new(String::from("Manish"), &book.content[..10]);
    
    // annoucement
    mem.announce();

    // status mutation
    let new_status = "inactive";
    
    if new_status == "inactive" {
        mem.deactive();
        println!("Deactivated: {:?}", mem);
    } else {
        mem.reactive();
        println!("Reactivated: {:?}", mem);
    }
}

#[derive(Debug)]
enum Status {
    Active, 
    Inactive
}

#[derive(Debug)]
struct Book {
    content: String,
}

#[derive(Debug)]
struct Member<'a> {
    name: String,
    borrowed_part: &'a str,
    status: Status
}

impl<'a> Member<'a> {
    fn new(name: String, borrowed_part: &'a str) -> Member<'a> {
        Member {
            name,
            borrowed_part,
            status: Status::Active
        }
    }
    fn announce(&self){
        let s;
        match self.status {
            Status::Active => s = "Active",
            Status::Inactive => s = "Inactive"
        }
        let name = &self.name;
        let borrowed_part = &self.borrowed_part;
        println!("Member {name} (Status: {s}) is currently reading: '{borrowed_part}'");
    }
    fn deactive(&mut self){
        self.status = Status::Inactive;
    }
    fn reactive(&mut self){
        self.status = Status::Active;
    }
}

impl Book {
    fn new(content: String) -> Book {
        Book {
            content
        }
    }
}

fn create_a_book() -> Book {
    // creating a new book
    let book_content = String::from("The example function body uses the assert_eq! macro to assert that result, which contains the result of calling add with 2 and 2, equals 4. This assertion serves as an example of the format for a typical test. Let’s run it to see that this test passes.");
    let book = Book::new(book_content);
    book
}


