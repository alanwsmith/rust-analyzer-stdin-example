use std::io::Write;
use std::process::{Command, Stdio};
use std::{thread, time};

fn main() {
    let mut child = Command::new("rust-analyzer")
        .stdin(Stdio::piped())
        .spawn()
        .unwrap();
    let mut stdin = child.stdin.take().unwrap();

    std::thread::spawn(move || {
        let initialize_content = include_str!["jsons/initialize.json"];
        let initialize_payload = format!("Content-Length: {}\r\n\r\n{}", &initialize_content.len(), initialize_content);
        stdin.write_all(initialize_payload.as_bytes()).unwrap();

        let initialized_content = include_str!["jsons/initialized.json"];
        let initialized_payload = format!("Content-Length: {}\r\n\r\n{}", &initialized_content.len(), initialized_content);
        stdin.write_all(initialized_payload.as_bytes()).unwrap();

        let did_open_content = include_str!["jsons/did_open.json"];
        let did_open_payload = format!("Content-Length: {}\r\n\r\n{}", &did_open_content.len(), did_open_content);
        stdin.write_all(did_open_payload.as_bytes()).unwrap();

        // this should be handled differently, but this got me
        // the details I needed without additional complexity
        let sleep_time = time::Duration::from_millis(4000);
        thread::sleep(sleep_time);

        let semantic_tokens_full_content = include_str!["jsons/semantic_tokens_full.json"];
        let semantic_tokens_full_payload = format!("Content-Length: {}\r\n\r\n{}", &semantic_tokens_full_content.len(), semantic_tokens_full_content);
        stdin.write_all(semantic_tokens_full_payload.as_bytes()).unwrap();

        let sleep_time = time::Duration::from_millis(2000);
        thread::sleep(sleep_time);

        let exit_content = include_str!["jsons/exit.json"];
        let exit_payload = format!("Content-Length: {}\r\n\r\n{}", &exit_content.len(), exit_content);
        stdin.write_all(exit_payload.as_bytes()).unwrap();

    });

    let _ = child.wait();
    println!("Process complete");

}
