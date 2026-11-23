use std::env;
use std::io::{self, Read, Write};
use std::net::{Shutdown, SocketAddr, TcpListener, TcpStream, ToSocketAddrs};
use std::os::fd::AsRawFd;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

const SO_ORIGINAL_DST: i32 = 80;
const SOL_IP: i32 = 0;
const MAX_HEADERS: usize = 16 * 1024;
const MAX_PROXY_ATTEMPTS: usize = 5;

#[repr(C)]
struct SockAddrIn {
    family: u16,
    port: u16,
    address: u32,
    zero: [u8; 8],
}

unsafe extern "C" {
    fn getsockopt(
        fd: i32,
        level: i32,
        option: i32,
        value: *mut u8,
        length: *mut u32,
    ) -> i32;
}

struct Config {
    server: String,
    port: u16,
    username: String,
    password: String,
}

fn загрузить_настройки(path: &str) -> Result<Config, String> {
    let содержимое = std::fs::read_to_string(path)
        .map_err(|e| format!("Не удалось прочитать конфигурацию: {e}"))?;

    let json: serde_json::Value = serde_json::from_str(&содержимое)
        .map_err(|e| format!("Ошибка разбора конфигурации: {e}"))?;

    let выходы = json["outbounds"]
        .as_array()
        .ok_or("В конфигурации отсутствует список outbounds")?;

    for выход in выходы {
        if выход["tag"].as_str() == Some("youtube-proxy")
            && выход["type"].as_str() == Some("http")
        {
            let server = выход["server"]
                .as_str()
                .ok_or("Не указан сервер прокси")?
                .to_string();

            let port = выход["server_port"]
                .as_u64()
                .ok_or("Не указан порт прокси")?;

            if port == 0 || port > 65535 {
                return Err("Некорректный порт прокси".into());
            }

            return Ok(Config {
                server,
                port: port as u16,
                username: выход["username"].as_str().unwrap_or("").to_string(),
                password: выход["password"].as_str().unwrap_or("").to_string(),
            });
        }
    }

    Err("Не найден HTTP outbound youtube-proxy".into())
}

fn исходный_адрес(поток: &TcpStream) -> io::Result<SocketAddr> {
    let mut адрес = SockAddrIn {
        family: 0,
        port: 0,
        address: 0,
        zero: [0; 8],
    };

    let mut длина = std::mem::size_of::<SockAddrIn>() as u32;

    let результат = unsafe {
        getsockopt(
            поток.as_raw_fd(),
            SOL_IP,
            SO_ORIGINAL_DST,
            &mut адрес as *mut _ as *mut u8,
            &mut длина,
        )
    };

    if результат != 0 {
        return Err(io::Error::last_os_error());
    }

    if адрес.family != 2 {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            "Получен адрес назначения не IPv4",
        ));
    }

    let ip = std::net::Ipv4Addr::from(u32::from_be(адрес.address));
    let port = u16::from_be(адрес.port);

    Ok(SocketAddr::from((ip, port)))
}

fn кодировать_base64(данные: &[u8]) -> String {
    let mut результат = String::new();
    let таблица = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

    for часть in данные.chunks(3) {
        let a = часть.first().copied().unwrap_or(0) as u32;
        let b = часть.get(1).copied().unwrap_or(0) as u32;
        let c = часть.get(2).copied().unwrap_or(0) as u32;
        let число = (a << 16) | (b << 8) | c;

        результат.push(таблица[((число >> 18) & 63) as usize] as char);
        результат.push(таблица[((число >> 12) & 63) as usize] as char);
        результат.push(if часть.len() > 1 {
            таблица[((число >> 6) & 63) as usize] as char
        } else {
            '='
        });
        результат.push(if часть.len() > 2 {
            таблица[(число & 63) as usize] as char
        } else {
            '='
        });
    }
    результат
}

fn подключиться_к_прокси(config: &Config) -> io::Result<TcpStream> {
    let адрес = format!("{}:{}", config.server, config.port);
    let адреса = адрес.to_socket_addrs()?;

    for адрес in адреса {
        match TcpStream::connect_timeout(&адрес, Duration::from_secs(8)) {
            Ok(поток) => {
                let _ = поток.set_nodelay(true);
                return Ok(поток);
            }
            Err(ошибка) => {
                eprintln!("Не удалось подключиться к прокси: {ошибка}");
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::ConnectionRefused,
        "Не удалось подключиться ни к одному адресу прокси",
    ))
}

enum ОшибкаConnect {
    Повторить(String),
    Фатальная(io::Error),
}

fn попытка_connect(
    прокси: &mut TcpStream,
    назначение: &str,
    config: &Config,
) -> Result<(), ОшибкаConnect> {
    let _ = прокси.set_read_timeout(Some(Duration::from_secs(8)));
    let _ = прокси.set_write_timeout(Some(Duration::from_secs(8)));

    let mut запрос = format!(
        "CONNECT {назначение} HTTP/1.1\r\nHost: {назначение}\r\nProxy-Connection: Keep-Alive\r\n"
    );

    if !config.username.is_empty() || !config.password.is_empty() {
        let данные = format!("{}:{}", config.username, config.password);
        let кодировка = кодировать_base64(данные.as_bytes());
        запрос.push_str(&format!("Proxy-Authorization: Basic {кодировка}\r\n"));
    }

    запрос.push_str("\r\n");
    прокси
        .write_all(запрос.as_bytes())
        .map_err(|e| ОшибкаConnect::Повторить(format!("ошибка отправки CONNECT: {e}")))?;

    let mut заголовки = Vec::new();
    let mut байт = [0u8; 1];

    while заголовки.len() < MAX_HEADERS {
        match прокси.read_exact(&mut байт) {
            Ok(()) => заголовки.push(байт.first().copied().unwrap_or(0)),
            Err(e) => {
                return Err(ОшибкаConnect::Повторить(format!(
                    "обрыв при чтении ответа CONNECT: {e}"
                )));
            }
        }

        if заголовки.ends_with(b"\r\n\r\n") {
            break;
        }
    }

    let _ = прокси.set_read_timeout(None);
    let _ = прокси.set_write_timeout(None);

    if !заголовки.ends_with(b"\r\n\r\n") {
        return Err(ОшибкаConnect::Фатальная(io::Error::new(
            io::ErrorKind::InvalidData,
            "Слишком длинные заголовки ответа прокси",
        )));
    }

    let текст = String::from_utf8_lossy(&заголовки);
    let строка_статуса = текст.lines().next().unwrap_or("");
    let код = строка_статуса.split_whitespace().nth(1).unwrap_or("");

    match код {
        "200" => Ok(()),
        "502" | "503" | "504" => Err(ОшибкаConnect::Повторить(строка_статуса.to_string())),
        _ => Err(ОшибкаConnect::Фатальная(io::Error::new(
            io::ErrorKind::ConnectionRefused,
            format!("Прокси отклонил CONNECT: {строка_статуса}"),
        ))),
    }
}

fn установить_туннель_и_отправить_hello(
    назначение: &str,
    начальные_данные: &[u8],
    config: &Config,
) -> io::Result<(TcpStream, Vec<u8>)> {
    for попытка in 1..=MAX_PROXY_ATTEMPTS {
        let mut прокси = подключиться_к_прокси(config)?;
        match попытка_connect(&mut прокси, назначение, config) {
            Ok(()) => {
                if начальные_данные.is_empty() {
                    return Ok((прокси, Vec::new()));
                }

                let res_write = if начальные_данные.len() > 5 {
                    прокси.write_all(&начальные_данные[..5])
                        .and_then(|_| прокси.flush())
                        .and_then(|_| {
                            thread::sleep(Duration::from_millis(15));
                            прокси.write_all(&начальные_данные[5..])
                        })
                        .and_then(|_| прокси.flush())
                } else {
                    прокси.write_all(начальные_данные)
                };
                if let Err(e) = res_write {
                    eprintln!("[{назначение}] Попытка {попытка}: ошибка отправки ClientHello: {e}");
                    continue;
                }

                let _ = прокси.set_read_timeout(Some(Duration::from_secs(5)));
                let mut первый_ответ = [0u8; 4096];
                match прокси.read(&mut первый_ответ) {
                    Ok(0) => {
                        eprintln!("[{назначение}] Попытка {попытка}/{MAX_PROXY_ATTEMPTS}: прокси закрыл туннель с 0 байт, повтор...");
                        thread::sleep(Duration::from_millis(100 * попытка as u64));
                        continue;
                    }
                    Ok(n) => {
                        let _ = прокси.set_read_timeout(None);
                        return Ok((прокси, первый_ответ[..n].to_vec()));
                    }
                    Err(e) => {
                        eprintln!("[{назначение}] Попытка {попытка}/{MAX_PROXY_ATTEMPTS}: таймаут/ошибка TLS ответа ({e}), повтор...");
                        thread::sleep(Duration::from_millis(100 * попытка as u64));
                        continue;
                    }
                }
            }
            Err(ОшибкаConnect::Повторить(причина)) => {
                eprintln!(
                    "[{назначение}] Попытка {попытка}/{MAX_PROXY_ATTEMPTS}: {причина}, повтор..."
                );
                if попытка < MAX_PROXY_ATTEMPTS {
                    thread::sleep(Duration::from_millis(100 * попытка as u64));
                }
            }
            Err(ОшибкаConnect::Фатальная(ошибка)) => return Err(ошибка),
        }
    }

    Err(io::Error::new(
        io::ErrorKind::TimedOut,
        format!("Не удалось установить рабочий туннель к {назначение} после {MAX_PROXY_ATTEMPTS} попыток"),
    ))
}

fn передавать(
    mut источник: TcpStream,
    mut назначение: TcpStream,
    направление: &'static str,
) {
    let mut буфер = [0u8; 32 * 1024];
    let mut всего: u64 = 0;

    loop {
        match источник.read(&mut буфер) {
            Ok(0) => {
                eprintln!("[{направление}] EOF, передано байт: {всего}");
                break;
            }
            Ok(количество) => {
                if let Err(ошибка) = назначение.write_all(&буфер[..количество]) {
                    eprintln!("[{направление}] Ошибка записи после {всего} байт: {ошибка}");
                    break;
                }
                всего += количество as u64;
            }
            Err(ошибка) if ошибка.kind() == io::ErrorKind::Interrupted => continue,
            Err(ошибка) => {
                eprintln!("[{направление}] Ошибка чтения после {всего} байт: {ошибка}");
                break;
            }
        }
    }

    let _ = назначение.shutdown(Shutdown::Both);
    let _ = источник.shutdown(Shutdown::Both);
}

fn извлечь_sni(данные: &[u8]) -> Option<String> {
    let mut handshake = Vec::new();
    let mut позиция = 0usize;

    loop {
        if данные.len().saturating_sub(позиция) < 5 || *данные.get(позиция)? != 22 {
            return None;
        }

        let длина_записи =
            u16::from_be_bytes([*данные.get(позиция + 3)?, *данные.get(позиция + 4)?]) as usize;
        let конец = позиция.checked_add(5)?.checked_add(длина_записи)?;

        if конец > данные.len() {
            return None;
        }

        handshake.extend_from_slice(&данные[позиция + 5..конец]);
        позиция = конец;

        if handshake.len() >= 4 {
            if *handshake.first()? != 1 {
                return None;
            }

            let длина_сообщения = ((*handshake.get(1)? as usize) << 16)
                | ((*handshake.get(2)? as usize) << 8)
                | (*handshake.get(3)? as usize);

            if handshake.len() < 4 + длина_сообщения {
                continue;
            }

            let тело = &handshake[4..4 + длина_сообщения];
            if тело.len() < 34 {
                return None;
            }

            let mut курсор = 34usize;
            let длина_session_id = *тело.get(курсор)? as usize;
            курсор = курсор.checked_add(1 + длина_session_id)?;

            let длина_шифров =
                u16::from_be_bytes([*тело.get(курсор)?, *тело.get(курсор + 1)?]) as usize;
            курсор = курсор.checked_add(2 + длина_шифров)?;

            let длина_компрессии = *тело.get(курсор)? as usize;
            курсор = курсор.checked_add(1 + длина_компрессии)?;

            let длина_расширений =
                u16::from_be_bytes([*тело.get(курсор)?, *тело.get(курсор + 1)?]) as usize;
            курсор += 2;

            let конец_расширений = курсор.checked_add(длина_расширений)?;
            if конец_расширений > тело.len() {
                return None;
            }

            while курсор.checked_add(4)? <= конец_расширений {
                let тип_расширения =
                    u16::from_be_bytes([*тело.get(курсор)?, *тело.get(курсор + 1)?]);
                let длина_данных =
                    u16::from_be_bytes([*тело.get(курсор + 2)?, *тело.get(курсор + 3)?]) as usize;
                курсор += 4;

                let конец_данных = курсор.checked_add(длина_данных)?;
                if конец_данных > конец_расширений {
                    return None;
                }

                if тип_расширения == 0 {
                    let список = &тело[курсор..конец_данных];
                    if список.len() < 2 {
                        return None;
                    }
                    let длина_списка =
                        u16::from_be_bytes([*список.first()?, *список.get(1)?]) as usize;
                    let mut p = 2usize;
                    let конец_списка = p.checked_add(длина_списка)?;
                    if конец_списка > список.len() {
                        return None;
                    }

                    while p.checked_add(3)? <= конец_списка {
                        let тип_имени = *список.get(p)?;
                        let длина_имени =
                            u16::from_be_bytes([*список.get(p + 1)?, *список.get(p + 2)?]) as usize;
                        p += 3;

                        let конец_имени = p.checked_add(длина_имени)?;
                        if конец_имени > конец_списка {
                            return None;
                        }

                        if тип_имени == 0 {
                            let имя = std::str::from_utf8(&список[p..конец_имени]).ok()?;
                            return Some(имя.to_string());
                        }
                        p = конец_имени;
                    }
                }
                курсор = конец_данных;
            }
            return None;
        }
    }
}

fn прочитать_client_hello(клиент: &mut TcpStream) -> io::Result<(Vec<u8>, Option<String>)> {
    клиент.set_read_timeout(Some(Duration::from_secs(3)))?;

    let результат = (|| {
        let mut данные = Vec::new();
        let mut буфер = [0u8; 4096];

        loop {
            match клиент.read(&mut буфер) {
                Ok(0) => break,
                Ok(количество) => {
                    данные.extend_from_slice(&буфер[..количество]);

                    if данные.first().map(|байт| *байт != 22).unwrap_or(false) {
                        break;
                    }

                    if let Some(имя) = извлечь_sni(&данные) {
                        return Ok((данные, Some(имя)));
                    }

                    if данные.len() >= 64 * 1024 {
                        break;
                    }
                }
                Err(ошибка) if ошибка.kind() == io::ErrorKind::Interrupted => continue,
                Err(ошибка)
                    if ошибка.kind() == io::ErrorKind::TimedOut
                        || ошибка.kind() == io::ErrorKind::WouldBlock =>
                {
                    break;
                }
                Err(ошибка) => return Err(ошибка),
            }
        }

        Ok((данные, None))
    })();

    let _ = клиент.set_read_timeout(None);
    результат
}

fn замаскировать_sni(данные: &mut [u8]) -> bool {
    if данные.len() < 43 || данные[0] != 22 || данные[5] != 1 {
        return false;
    }
    let mut c = 43usize;
    if c >= данные.len() { return false; }
    c += 1 + данные[c] as usize;
    if c + 2 > данные.len() { return false; }
    let cs_len = u16::from_be_bytes([данные[c], данные[c + 1]]) as usize;
    c += 2 + cs_len;
    if c >= данные.len() { return false; }
    c += 1 + данные[c] as usize;
    if c + 2 > данные.len() { return false; }
    let ext_len = u16::from_be_bytes([данные[c], данные[c + 1]]) as usize;
    c += 2;
    let end = c + ext_len;
    if end > данные.len() { return false; }
    while c + 4 <= end {
        let etype = u16::from_be_bytes([данные[c], данные[c + 1]]);
        let elen = u16::from_be_bytes([данные[c + 2], данные[c + 3]]) as usize;
        if etype == 0 {
            данные[c] = 0x0a;
            данные[c + 1] = 0x0a;
            return true;
        }
        c += 4 + elen;
    }
    false
}

fn обработать_соединение(mut клиент: TcpStream, config: Arc<Config>) {
    let _ = клиент.set_nodelay(true);

    let назначение = match исходный_адрес(&клиент) {
        Ok(адрес) => адрес,
        Err(ошибка) => {
            eprintln!("Не удалось определить исходный адрес: {ошибка}");
            return;
        }
    };

    let (mut начальные_данные, sni) = if назначение.port() == 443 {
        match прочитать_client_hello(&mut клиент) {
            Ok(результат) => результат,
            Err(ошибка) => {
                eprintln!("Ошибка чтения TLS ClientHello: {ошибка}");
                return;
            }
        }
    } else {
        (Vec::new(), None)
    };

    let цель_connect = sni
        .as_ref()
        .map(|имя| format!("{имя}:{}", назначение.port()))
        .unwrap_or_else(|| назначение.to_string());

    println!("[DEBUG] IP: {} | SNI: {:?} | ClientHello: {} байт", назначение, sni, начальные_данные.len());
    if let Some(ref имя) = sni {
        if имя.ends_with(".googlevideo.com") {
            let masked = false;
            println!("[MASK] SNI {} замаскирован (GREASE): {}", имя, masked);
        }
    }
    let (прокси, первый_ответ) =
        match установить_туннель_и_отправить_hello(&цель_connect, &начальные_данные, &config) {
            Ok(res) => res,
            Err(ошибка) => {
                eprintln!("Ошибка установки туннеля: {ошибка}");
                return;
            }
        };

    match &sni {
        Some(имя) => println!("TLS SNI: {имя} (назначение CONNECT: {цель_connect})"),
        None if назначение.port() == 443 => {
            println!("TLS SNI не обнаружен (назначение CONNECT: {цель_connect})");
        }
        None => {}
    }

    if !первый_ответ.is_empty() {
        if let Err(ошибка) = клиент.write_all(&первый_ответ) {
            eprintln!("Не удалось передать первый ответ клиенту: {ошибка}");
            return;
        }
    }

    println!("Туннель установлен и проверен: {цель_connect}");

    let клиент_для_отправки = match клиент.try_clone() {
        Ok(поток) => поток,
        Err(ошибка) => {
            eprintln!("Не удалось клонировать соединение клиента: {ошибка}");
            return;
        }
    };

    let прокси_для_отправки = match прокси.try_clone() {
        Ok(поток) => поток,
        Err(ошибка) => {
            eprintln!("Не удалось клонировать соединение прокси: {ошибка}");
            return;
        }
    };

    let поток_клиента = thread::spawn(move || {
        передавать(клиент_для_отправки, прокси, "iPhone → прокси");
    });

    передавать(прокси_для_отправки, клиент, "прокси → iPhone");

    let _ = поток_клиента.join();
    println!("Соединение завершено: {назначение}");
}

fn main() {
    let аргументы: Vec<String> = env::args().collect();

    if аргументы.len() != 3 {
        eprintln!("Использование: youtube-relay CONFIG.json АДРЕС_ПРОСЛУШИВАНИЯ");
        std::process::exit(2);
    }

    let config_path = аргументы.get(1).expect("отсутствует путь к конфигу");
    let listen_addr = аргументы.get(2).expect("отсутствует адрес прослушивания");

    let config = match загрузить_настройки(config_path) {
        Ok(config) => Arc::new(config),
        Err(ошибка) => {
            eprintln!("{ошибка}");
            std::process::exit(1);
        }
    };

    let слушатель = match TcpListener::bind(listen_addr) {
        Ok(слушатель) => слушатель,
        Err(ошибка) => {
            eprintln!("Не удалось открыть порт: {ошибка}");
            std::process::exit(1);
        }
    };

    println!("Релей запущен на {listen_addr}");
    println!("Внешний HTTP-прокси: {}:{}", config.server, config.port);

    for соединение in слушатель.incoming() {
        match соединение {
            Ok(клиент) => {
                let настройки = Arc::clone(&config);
                thread::spawn(move || обработать_соединение(клиент, настройки));
            }
            Err(ошибка) => eprintln!("Ошибка входящего соединения: {ошибка}"),
        }
    }
}
