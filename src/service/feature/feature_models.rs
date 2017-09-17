


    #[derive(FromForm)]
    pub struct Person {
        pub name: String,
        pub age: String
    }

    #[derive(Serialize)]
    pub struct CpuInf {
        pub one: f64,
        pub five: f64,
        pub fifteen: f64
    }

    #[derive(Serialize)]
    pub struct MemInf {
        pub total: u64,
        pub free: u64,
        pub avail: u64,
        pub buffers: u64,
        pub cached: u64,
        pub swap_total: u64,
        pub swap_free: u64,
    }

    #[derive(Serialize)]
    pub struct SystemInfo {
        pub host: String,
        pub os: String,
        pub rel: String,
        pub c_cnt: u32,
        pub c_frq: u64,
        pub cpu: CpuInf,
        pub mem: MemInf
    }

 
