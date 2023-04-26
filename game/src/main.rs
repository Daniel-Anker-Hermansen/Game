use std::{ops::Deref, net::{TcpStream, TcpListener}, io::{Write, BufWriter, BufReader}};

use io_serde::{BufReadExt, BufWriteExt};
use plugin_lib::{Version, ffi_types::{FFIVec, FFIStaticStr}};

const __API_VERSION: &[u8] = b"__API_VERSION";
const __PLUGIN_NAME: &[u8] = b"__PLUGIN_NAME";

fn main() {
    let tcp = connect();

    let plugins = std::fs::read_dir("plugins").unwrap();
    let mut plugs = vec![];
    let mut libs = vec![];
    for file in plugins {
        match file {
            Ok(file) => {
                let path = file.path();
                let lib = unsafe { libloading::Library::new(path).unwrap() };
                let version = unsafe { lib.get::<&'static Version>(__API_VERSION).unwrap() };
                let name = unsafe { lib.get::<&'static FFIStaticStr>(__PLUGIN_NAME).unwrap() };
                let name: &str = (*name.deref()).into();
                println!("Plugin name: {}", name);
                plugs.push(name);
                println!("{}.{}.{}", version.major, version.minor, version.patch);
                if let Ok(exported_items) = unsafe { lib.get::<extern "C" fn() -> FFIVec>(b"__exported_items") } {
                    let ffi_vec = exported_items();
                    let vec: Vec<FFIStaticStr> = unsafe { ffi_vec.to_vec() };
                    for item in vec {
                        let string: &str = item.into();
                        let ident = format!("__item_id_{}", string);
                        let item_id = unsafe { lib.get::<extern "C" fn() -> i64>(ident.as_bytes()).unwrap() };
                        let id = item_id();
                        println!("{} has id: {}", string, id);
                    }
                }
                libs.push(lib);
            },
            Err(e) => eprintln!("{e}"),
        }
    }
    let tcp_writer = tcp.try_clone().unwrap();
    let mut buf_writer = BufWriter::new(tcp_writer);
    let mut buf_reader = BufReader::new(tcp);
    let thread = std::thread::spawn(move || {
        let plugs: Vec<String> = buf_reader.read_serde().unwrap();
        println!("\nOther client has plugins:");
        for plug in plugs {
            println!("{plug}");
        }
    });
    buf_writer.write_serde(&plugs).unwrap();
    buf_writer.flush().unwrap();
    thread.join().unwrap();
}

fn connect() -> TcpStream {
    println!("Press 0 for hosting, press 1 for connecting");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    match input.trim() {
        "0" => {
            let listener = TcpListener::bind("0.0.0.0:0").unwrap();
            let addr = listener.local_addr().unwrap().port();
            println!("Hosting at port: {addr:?}");
            let (tcp, socket) = listener.accept().unwrap();
            println!("Connected to: {socket:?}");
            tcp
        }
        "1" => {
            println!("Please enter adress");
            let mut input = String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            let addr = input.trim();
            TcpStream::connect(addr).unwrap()
        }
        _ => connect()
    }
}
