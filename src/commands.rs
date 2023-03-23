use std::process::{ Command, Stdio, Child, ChildStdout };

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

