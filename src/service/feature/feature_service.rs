use sys_info::{cpu_num, cpu_speed, hostname, loadavg, mem_info, os_release, os_type};
use sys_info::{LoadAvg, MemInfo};

use super::feature_models::{CpuInf, MemInf, SystemInfo};

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

    let mem: MemInf = MemInf {
        total: mem_inf.total,
        free: mem_inf.free,
        avail: mem_inf.avail,
        buffers: mem_inf.buffers,
        cached: mem_inf.cached,
        swap_total: mem_inf.swap_total,
        swap_free: mem_inf.swap_free,
    };

    SystemInfo::new(host, os, rel, cpu_cnt, cpu_frq, cpu, mem)
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn get_module_path() -> &'static str {
    "src/service/feature_module"
}
