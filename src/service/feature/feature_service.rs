
use sys_info::{LoadAvg, MemInfo};
use sys_info::{loadavg, mem_info, cpu_num, os_release, os_type, hostname, cpu_speed};

use service::feature::feature_models::*;

// private methods
pub fn sysinf() -> SystemInfo {
    let host: String = hostname().unwrap();
    let os: String = os_type().unwrap();
    let rel: String = os_release().unwrap();
    let load: LoadAvg = loadavg().unwrap();
    let mem_inf: MemInfo = mem_info().unwrap();
    let cpu_cnt: u32 = cpu_num().unwrap();
    let cpu_frq: u64 = cpu_speed().unwrap();

    let cpu: CpuInf = CpuInf {
        one: load.one,
        five: load.five,
        fifteen: load.fifteen,
    };

    let mem: MemInf = MemInf::from(MemInf {
        total: mem_inf.total,
        free: mem_inf.free,
        avail: mem_inf.avail,
        buffers: mem_inf.buffers,
        cached: mem_inf.cached,
        swap_total: mem_inf.swap_total,
        swap_free: mem_inf.swap_free,
    });

    let info: SystemInfo = SystemInfo {
        host: host,
        os: os,
        rel: rel,
        c_cnt: cpu_cnt,
        c_frq: cpu_frq,
        cpu: cpu,
        mem: mem,
    };

    info
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn get_module_path() -> &'static str {
    "src/service/feature_module"
}
