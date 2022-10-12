extern crate libc;

use std::env;
use std::ffi::CString;
use std::fs::File;
use std::os::unix::io::AsRawFd;

use nix::errno::Errno;
use nix::unistd::execvp;

fn main() {
    let args: Vec<String> = env::args().collect();

    let pid = &args[1];
    let ns = &args[2];
    let cmd = &args[3];
    let mut arg = args.clone();
    arg.drain(0..=2);

    println!("pid is {}, namespace is {}", pid, ns);
    println!("cmd is {}, arg is {:?}", cmd, arg);

    let mnt_ns_path = format!("/proc/{}/ns/{}", pid, ns);
    let file = File::open(mnt_ns_path.as_str()).unwrap();
    let fd = file.as_raw_fd();
    println!("fd is {:?}", fd);

    let res = unsafe { libc::setns(fd, 0) };
    if res !=0 {
        println!("failed to setns FFDD: {}", res);
        println!("Error is: {:?}", Errno::result(res).map(drop));
    }

    let p = CString::new(cmd.to_string()).unwrap();
    let sa: Vec<CString> = arg
        .iter()
        .map(|s| CString::new(s.to_string()).unwrap_or_default())
        .collect();
    let _ = execvp(p.as_c_str(), &sa).map_err(|e| {
        println!("error is {:?}", e);
        e
    }); 

}
