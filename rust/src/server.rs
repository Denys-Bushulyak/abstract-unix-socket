use std::error::Error;
use std::fs::File;
use std::io::Read;
use std::os::linux::net::SocketAddrExt;
use std::os::unix::net::{SocketAddr, UnixListener};
use std::path::PathBuf;
use std::str::FromStr;
use std::sync::Arc;
use std::thread::spawn;

use quick_js::{ContextError, ExecutionError};

const UNIX_SOCKET_NAME: &str = "hidden";
const PROGRAM_JS_FILE: &str = "fsharp/Program.js";

type ProgramScript = String;

#[derive(Debug)]
enum ExecutionProblems {
    CanNotEvalProgramCode(ProgramScript, ExecutionError),
    CreationContextProblem(ContextError),
    CanNotCallFunction(ExecutionError),
    CanNotFindProgramFile,
}

fn main() -> Result<(), Box<dyn Error>> {
    let addr = SocketAddr::from_abstract_name(UNIX_SOCKET_NAME)?;
    let listener = UnixListener::bind_addr(&addr)?;

    for stream in listener.incoming() {
        spawn(move || match stream {
            Ok(mut data) => {
                let path_buf = PathBuf::from_str(PROGRAM_JS_FILE).unwrap();
                let mut file = File::open(path_buf).unwrap();
                let mut buffer = String::new();
                file.read_to_string(&mut buffer).unwrap();
                let program = regex::Regex::new(r#"export\s*\{[^}]*\};"#)
                    .unwrap()
                    .replace(&buffer, "");
                let mut buf = String::new();
                let _ = data.read_to_string(&mut buf);

                let result = quick_js::Context::new()
                    .map_err(ExecutionProblems::CreationContextProblem)
                    .and_then(|vm| {
                        vm.eval(&program).map(|_| vm).map_err(|e: ExecutionError| {
                            ExecutionProblems::CanNotEvalProgramCode(program.to_string(), e)
                        })
                    })
                    .and_then(|vm| {
                        vm.call_function("handle", vec![buf])
                            .map_err(|e| ExecutionProblems::CanNotCallFunction(e))
                    });

                dbg!(result);
            }
            Err(e) => eprintln!("{:?}", e),
        });
    }

    Ok(())
}
