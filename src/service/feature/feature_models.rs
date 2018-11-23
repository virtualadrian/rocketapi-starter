use serde_derive::{Deserialize, Serialize};

#[derive(FromForm, Serialize, Deserialize)]
pub struct Person {
    pub name: String,
    pub age: String,
}

#[derive(Serialize, Deserialize)]
pub struct CpuInf {
    pub one: f64,
    pub five: f64,
    pub fifteen: f64,
}

#[derive(Serialize, Deserialize)]
pub struct MemInf {
    pub total: u64,
    pub free: u64,
    pub avail: u64,
    pub buffers: u64,
    pub cached: u64,
    pub swap_total: u64,
    pub swap_free: u64,
}

#[derive(Serialize, Deserialize)]
pub struct SystemInfo {
    host: String,
    pub os: String,
    pub rel: String,
    pub c_cnt: u32,
    pub c_frq: u64,
    pub cpu: CpuInf,
    pub mem: MemInf,
}

impl SystemInfo {
    pub fn set_host(&mut self, host: String) {
        self.host = host;
    }

    pub fn get_host(&self) -> String {
        self.host.to_string()
    }

    pub fn new(
        host: String,
        os: String,
        rel: String,
        c_cnt: u32,
        c_frq: u64,
        cpu: CpuInf,
        mem: MemInf,
    ) -> SystemInfo {
        SystemInfo {
            host,
            os,
            rel,
            c_cnt,
            c_frq,
            cpu,
            mem,
        }
    }
}
