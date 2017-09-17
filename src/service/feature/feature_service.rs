
pub mod feature_module {

    use rocket_contrib::{Json};

    use sys_info::{LoadAvg, MemInfo};
    use sys_info::{loadavg, mem_info, cpu_num, os_release, os_type, hostname, cpu_speed};

    #[get("/")]
    pub fn index() -> String {
         format!("Hello world!")
    }

    #[get("/howdy")]
    pub fn howdy_index() -> String {
         format!("Hello 2.0 from the {} !", get_module_path())
    }

    #[get("/howdy/sysload")]
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
        let host: String = hostname().unwrap();
        let os: String = os_type().unwrap();
        let rel: String = os_release().unwrap();
        let load: LoadAvg = loadavg().unwrap();
        let mem_inf: MemInfo = mem_info().unwrap();
        let cpu_cnt: u32 = cpu_num().unwrap();
        let cpu_frq: u64 = cpu_speed().unwrap();

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
            host: host,
            os: os,
            rel: rel,
            c_cnt: cpu_cnt,
            c_frq: cpu_frq,
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
