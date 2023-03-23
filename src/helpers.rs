use std::{process::{ Command, Stdio, Child, ChildStdout }, io::Read};

pub fn exec(comando: String, args: Vec<&str>, entrada: Option<ChildStdout>) -> Child {
    let c: Child = match entrada {
        Some(entrada) =>{
            Command::new(comando)
                .args(args)
                .stdin(entrada)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("fallo al ejecutar el comando 'sudo'")
        },
        _ => {
            Command::new(comando)
                .args(args)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .spawn()
                .expect("fallo al ejecutar el comando 'sudo'")

        }
    };

    c
}

pub fn get_password(password: &str) -> Child {
    let args: Vec<&str> = vec![password];
    exec("echo".to_owned(), args, None)
}
pub fn check_password(password: &str) -> bool {
    let child = get_password(&password);
    let args = vec!["-k", "-S", "-p", "''", "whoami"];
    let result = exec("sudo".to_owned(), args, child.stdout);
    let l = read_output(result);
    l.trim().eq("root")
}

pub fn read_output(res: Child) -> String {
    let mut output = String::new();
    res.stdout.unwrap().read_to_string(&mut output).unwrap();
    output
}
