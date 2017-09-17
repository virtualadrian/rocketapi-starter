
pub mod feature_service;
pub mod feature_models;




    use rocket_contrib::{Json};

    #[get("/")]
    pub fn index() -> String {
        format!("Hello world!")
    }

    #[get("/howdy")]
    pub fn howdy_index() -> String {
        format!("Hello 2.0 from the {} !", feature_service::get_module_path())
    }

    #[get("/howdy/sysload")]
    pub fn howdy_load() -> Json<feature_models::SystemInfo> {
        Json(feature_service::sysinf())
    }

    #[get("/howdy/format")]
    pub fn howdy_format() -> String {
        format!("Hello 2.0 there, {}th world!", feature_service::plus_one(49))
    }

    #[get("/howdy/<name>")]
    pub fn howdy_name(name: String) -> String {
        format!("Howdy there! You told me your name is: {}.", name)
    }

    #[get("/howdy?<person>")]
    pub fn howdy_person_query(person: feature_models::Person) -> String {
        format!("Howdy there! You told me your name is: {}, and you are: {} years old.",
                person.name,
                person.age)
    }
