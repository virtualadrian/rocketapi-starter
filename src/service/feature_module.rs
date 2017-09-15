
pub mod feature_module {

    use rocket_contrib::{Json};

    use sys_info::{LoadAvg, MemInfo};
    use sys_info::{loadavg, mem_info, cpu_num};

    #[derive(FromForm)]
    pub struct Person {
        name: String,
        age: String
    }

    #[derive(Serialize)]
    pub struct CpuInf {
        one: f64,
        five: f64,
        fifteen: f64
    }

    #[derive(Serialize)]
    pub struct MemInf {
        total: u64,
        free: u64,
        avail: u64,
        buffers: u64,
        cached: u64,
        swap_total: u64,
        swap_free: u64,
    }

    #[derive(Serialize)]
    pub struct SystemInfo {
        c_cnt: u32,
        cpu: CpuInf,
        mem: MemInf
    }

    #[get("/")]
    pub fn index() -> String {
         format!("Hello world!")
    }

    #[get("/howdy")]
    pub fn howdy_index() -> String {
         format!("Hello 2.0 from the {} !", get_module_path())
    }

    #[get("/howdy/load")]
    pub fn howdy_load() -> Json<SystemInfo> {
        Json(sysinf())
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
    fn sysinf() -> SystemInfo {
        let load: LoadAvg = loadavg().unwrap();
        let mem_inf: MemInfo = mem_info().unwrap();
        let cpu_cnt: u32 = cpu_num().unwrap();

        let cpu: CpuInf = CpuInf::from(CpuInf{
            one: load.one,
            five: load.five,
            fifteen: load.fifteen
        });

        let mem: MemInf = MemInf::from(MemInf{
            total: mem_inf.total,
            free: mem_inf.free,
            avail: mem_inf.avail,
            buffers: mem_inf.buffers,
            cached: mem_inf.cached,
            swap_total: mem_inf.swap_total,
            swap_free: mem_inf.swap_free,
        });

        let info: SystemInfo = SystemInfo::from(SystemInfo{
            c_cnt: cpu_cnt,
            cpu: cpu,
            mem: mem
        });

        info
    }

    fn plus_one(x: i32) -> i32 {
        x + 1
    }

    fn get_module_path() -> &'static str {
        "src/service/feature_module"
    }
}
