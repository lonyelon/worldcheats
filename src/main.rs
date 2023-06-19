use nix::sys::ptrace;
use nix::sys::wait::waitpid;
use nix::unistd::Pid;
use std::env;
use std::process;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug)]
struct Item {
    id: u32,
    kind: String,
    name: String,
}

fn read_items<P: AsRef<Path>>(path: P) -> io::Result<Vec<Item>> {
    let file = File::open(path)?;
    let reader = io::BufReader::new(file);

    let mut items = Vec::new();
    for line in reader.lines() {
        let line = line?;
        let parts: Vec<&str> = line.split('\t').collect();
        if parts.len() != 3 {
            return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid line format"));
        }

        let id = parts[0].parse().map_err(|_| io::Error::new(io::ErrorKind::InvalidData, "Invalid id"))?;
        let kind = parts[1].to_string();
        let name = parts[2].to_string();

        items.push(Item { id, kind, name });
    }

    Ok(items)
}


fn main() {
    let first_addr = 0x2854a394 as *mut i32;
    let points_addr = 0x28510118 as *mut i32;
    let money_addr = 0x28510114 as *mut i32;
    
    let items = read_items("items.tsv").unwrap();

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Please provide a PID.");
        process::exit(1);
    }
    let pid: Pid = Pid::from_raw(args[1].parse::<i32>().unwrap());

    ptrace::attach(pid).unwrap();
    waitpid(pid, None).unwrap();   

    // Add all materials to item box.
    let mut i = 0;
    for item in items {
        let item_addr = unsafe{first_addr.offset(4*i)};
        if item.kind == "material" {
            unsafe {
                ptrace::write(pid, item_addr as ptrace::AddressType, item.id as *mut libc::c_void);
                ptrace::write(pid, item_addr.offset(1) as ptrace::AddressType, 400 as *mut libc::c_void);
                ptrace::write(pid, item_addr.offset(3) as ptrace::AddressType, 1 as *mut libc::c_void);
            }
        }
        i += 1;
    }
    
    // Add money and points.
    unsafe {
        ptrace::write(pid, points_addr as ptrace::AddressType, 50000 as *mut libc::c_void);
        ptrace::write(pid, money_addr as ptrace::AddressType, 1000000 as *mut libc::c_void);
    }

    ptrace::detach(pid, None).unwrap();
}
