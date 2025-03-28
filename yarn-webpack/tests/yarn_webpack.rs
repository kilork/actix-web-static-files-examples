use std::{
    error::Error,
    io::Read,
    process::{Child, Command, Stdio},
};

struct Wait(Child);

impl Drop for Wait {
    fn drop(&mut self) {
        self.0.kill().ok();
        self.0.wait().ok();
    }
}

#[test]
fn main() -> Result<(), Box<dyn Error>> {
    use assert_cmd::cargo::CommandCargoExt as _;

    const CARGO_PKG_NAME: &str = env!("CARGO_PKG_NAME");

    let mut cmd = Command::cargo_bin(CARGO_PKG_NAME)?;

    let mut child = cmd.stdout(Stdio::piped()).stdin(Stdio::inherit()).spawn()?;

    let port = server_port(&mut child)?;

    let _child = Wait(child);

    let text = reqwest::blocking::get(format!("http://127.0.0.1:{port}/"))?.text()?;
    let js = reqwest::blocking::get(format!("http://127.0.0.1:{port}/main.js"))?.text()?;

    assert!(text.contains("actix-web-static-files WebPack"));
    assert!(js.contains("main.js.LICENSE.txt"));

    Ok(())
}

fn server_port(child: &mut Child) -> Result<u16, Box<dyn Error>> {
    Ok(if let Some(output) = child.stdout.as_mut() {
        let mut buf = [0u8; 5];
        output.read_exact(&mut buf)?;
        let port = String::from_utf8_lossy(&buf);
        port.parse()?
    } else {
        0
    })
}
