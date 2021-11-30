use std::process::Command;
use serde::{Serialize};
use serde_json::Result;

fn serialize_cmd_output(infos: &Vec<Cmd>) -> Result<()> {
    println!("{}", serde_json::to_string(infos)?);
    Ok(())
}

#[derive(Debug)]
#[derive(Serialize)]
struct Cmd {
    cmd: String,
    stdout: String,
    stderr: String,
    status: i32,
}

fn main() {

    let cmds = vec![
        "python -V", 
        "docker version --format '{{ .Client.Version }}'", 
        "python2 -V"
    ];
    
    let mut infos: Vec<Cmd> = Vec::new();

    for cmd in cmds {
        let output = Command::new("sh")
                             .arg("-c")
                             .arg(cmd)
                             .output()
                             .expect("Could not executer process");  // this one only panics if `sh` does not spawn
        let cmd_parts: Vec<&str> = cmd.split(' ').collect();
        let info = Cmd { 
            cmd: String::from(cmd_parts[0]),
            stdout: String::from(String::from_utf8(output.stdout).unwrap().trim()),
            stderr: String::from(String::from_utf8(output.stderr).unwrap().trim()),
            status: output.status.code().unwrap()
        };
        infos.push(info);
    }
    
    let _result = serialize_cmd_output(&infos);
}

