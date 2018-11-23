use sys_info::{cpu_num, cpu_speed, hostname, loadavg, mem_info, os_release, os_type};
use sys_info::{LoadAvg, MemInfo};

use super::feature_models::{CpuInf, MemInf, SystemInfo};

// private methods
pub fn sysinf() -> SystemInfo {
    let load: LoadAvg = loadavg().unwrap();
    let mem_inf: MemInfo = mem_info().unwrap();

    SystemInfo {
        host: hostname().unwrap(),
        os: os_type().unwrap(),
        rel: os_release().unwrap(),
        c_cnt: cpu_num().unwrap(),
        c_frq: cpu_speed().unwrap(),
        cpu: CpuInf {
            one: load.one,
            five: load.five,
            fifteen: load.fifteen,
        },
        mem: MemInf {
            total: mem_inf.total,
            free: mem_inf.free,
            avail: mem_inf.avail,
            buffers: mem_inf.buffers,
            cached: mem_inf.cached,
            swap_total: mem_inf.swap_total,
            swap_free: mem_inf.swap_free,
        },
    }
}

pub fn plus_one(x: i32) -> i32 {
    x + 1
}

pub fn get_module_path() -> &'static str {
    "src/service/feature_module"
}
