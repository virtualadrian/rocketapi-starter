

pub mod feature_module {


    #[derive(FromForm)]
    pub struct Person {
        name: String,
        age: String
    }

    #[get("/")]
    pub fn index() -> String {
         format!("Hello world!")
    }

    #[get("/howdy")]
    pub fn howdy_index() -> String {
         format!("Hello 2.0 from the {} !", get_module_path())
    }

    #[get("/howdy/format")]
    pub fn howdy_format() -> String {
        format!("Hello 2.0 there, {}th world!", plus_one(49))
    }

    #[get("/howdy/<name>")]
    pub fn howdy_name(name: String) -> String {
        format!("Howdy there! You told me your name is: {}.", name)
    }

    #[get("/howdy?<person>")]
    pub fn howdy_person_query(person: Person) -> String {
        format!("Howdy there! You told me your name is: {}, and you are: {} years old.",
                    person.name,
                    person.age)
    }

    // private methods
    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    fn get_module_path() -> &'static str {
        "src/service/feature_module"
    }
}
