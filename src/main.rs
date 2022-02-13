use std::{
    env,
    error::Error,
    fs::File,
    io::{self, StdoutLock},
};

fn main() -> Result<(), Box<dyn Error>> {
    let out_handler = io::stdout();
    let out_handler = out_handler.lock();

    let args: Vec<_> = env::args().into_iter().skip(1).collect();

    if args.is_empty() {
        pipe_stdin_out(out_handler)?;
    } else {
        print_all_files(out_handler, args)?;
    }

    Ok(())
}

fn pipe_stdin_out(mut out_handler: StdoutLock) -> Result<(), Box<dyn Error + 'static>> {
    let in_handler = io::stdin();
    let mut in_handler = in_handler.lock();
    io::copy(&mut in_handler, &mut out_handler)?;
    Ok(())
}

fn print_all_files(mut out_handler: StdoutLock, args: Vec<String>) -> Result<(), Box<dyn Error>> {
    for filename in &args {
        let mut reader = File::open(filename)?;
        io::copy(&mut reader, &mut out_handler)?;
    }
    Ok(())
}
