extern crate libc;

use std::env;
use std::fs::File;
use std::os::unix::io::AsRawFd;

fn main() {
    let args: Vec<String> = env::args().collect();

    let pid = &args[1];
    let cmd = &args[2];

    println!("pid is {}", pid);
    println!("cmd is {}", cmd);

    let mnt_ns_path = format!("/proc/{}/ns/mnt", pid);

    unsafe { 
        let file = File::open(mnt_ns_path.as_str()).unwrap();
        let fd = file.as_raw_fd();
        let res = libc::setns(fd, 0);
        if res != 0 {
            println!("error is {}", res);
        }
    };
}
