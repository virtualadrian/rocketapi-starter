

#[derive(FromForm)]
struct Person {
    name: String,
    age: String
}

#[get("/")]
fn howdy() -> String {
     format!("Howdy there, {}th world!", plus_one(49))
}

#[get("/doody")]
fn doody() -> &'static str {
    "Howdy doody 2.0, world!"
}

#[get("/myname/<name>")]
fn myname(name: String) -> String {
    format!("Howdy there! You told me your name is: {}", name)
}

#[get("/hello?<person>")]
fn theirs(person: Person) -> String {
    format!("Howdy there! You told me your name is {} and you are {} years old.", person.name, person.age)
}

fn plus_one(x: i32) -> i32 {
    x + 1
}
