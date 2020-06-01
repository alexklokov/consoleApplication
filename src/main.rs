
extern crate libloading as lib;

extern crate server;
use server::router::Router;

use std::env;
use std::net::TcpListener;
use std::path::Path;
use std::path::MAIN_SEPARATOR;



fn start(ip: &str, port: &str, directory: &str, name: &str) -> String {
    let site = if directory.ends_with(MAIN_SEPARATOR) {
        format!("{}{}", directory, name)
    } else {
        format!("{}{}{}", directory, MAIN_SEPARATOR, name)
    };

    let dir = Path::new(&directory);
    match env::set_current_dir(&dir) {
        Err(_) => return "Не удалось открыть директорию".into(),
        _ => {}
    }

    if port.is_empty() || ip.is_empty() || directory.is_empty() || name.is_empty() {
        return "Вы не ввели все нужные данные".into();
    }

    let file = Path::new(&site);

    if !file.exists() {
        return "Файл сайта не существует".into();
    }
    if !site.ends_with(".so") && !site.ends_with(".dll") && !site.ends_with(".dylib") {
        return "Файл должен быть в формате .so, .dll или .dylib".into();
    }

    match TcpListener::bind(format!("{}:{}", ip, port)) {
        Err(err) => {
            return "IP-адрес или порт недоступены".into();
        }
        _ => {}
    }

    let file = lib::Library::new(site.as_str()).unwrap();
    let mut server = server::Server::new(&ip, &port);
    unsafe {
        let site: lib::Symbol<unsafe extern "C" fn() -> Router> = match file.get(b"site") {
            Ok(site) => {
                println!("Сервер запущен по адресу http://{}:{}", ip, port);
                site
            }
            Err(_) => {
                return "Ошибка. не была найдена функция site".into();
            }
        };
        let router = site();
        server.start(router);
    }

    String::new()
}




// Аргументы
// Адрес:порт -- опционально
// Путь до директории с сайтом -- обязательный
// Название файла .dll/.so -- обязательный
fn main() {
    let args: Vec<String> = env::args().collect();

    let address = if args.len() == 3 {
        "127.0.0.1:8080".to_string()
    } else {
        args[1].clone()
    };
    let address = address.split(":").collect::<Vec<&str>>();
    let port = address[1];
    let ip = address[0];

    let (directory, name): (&String, &String) = if args.len() == 3 {
        (&args[1], &args[2])
    } else {
        (&args[2], &args[3])
    };
    let log = start(ip, port, directory, name);
    println!("{}", log);

}
