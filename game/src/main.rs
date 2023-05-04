use std::{ops::Deref, net::{TcpStream, TcpListener}};

use plugin_lib::{Version, ffi_types::{FFIStaticStr, load_slice_from_lib, FFISlice}, ItemGame, ItemVTable};

const __API_VERSION: &[u8] = b"__API_VERSION";
const __PLUGIN_NAME: &[u8] = b"__PLUGIN_NAME";

fn main() {
    //let tcp = connect();
    
    std::panic::set_hook(Box::new(|_info| {
        println!("Do i get called in foreign code?");
    }));

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
                let items: &[ItemVTable] = unsafe { load_slice_from_lib(&lib, b"__EXPORTED_ITEMS") };
                for item in items {
                    let mut instance = item.new();
                    println!("Missing name...");
                    for _ in 0..50 {
                        println!("{}", instance.id());
                    }
                }
                libs.push(lib);
            },
            Err(e) => eprintln!("{e}"),
        }
    }
    /*let tcp_writer = tcp.try_clone().unwrap();
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
    thread.join().unwrap();*/
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
