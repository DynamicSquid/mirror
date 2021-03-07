mod mirror {
    pub struct Error {
        pub err_msg: String
    }

    impl Error {
        pub fn what(&self) -> String {
            return format!("[ error ] - {}\n", self.err_msg);
        }
    }
}

fn front_end(args: &Vec<String>) -> Result<(), mirror::Error> {
    if args.len() != 2 {
        return Err(mirror::Error{
            err_msg: String::from("expected two command line argument only()")
        });
    }

    let file = std::fs::File::open(args[1].clone());
    let _file = match file {
        Ok(file) => file,
        Err(_error) => return Err(mirror::Error{
            err_msg: format!("source file '{}' could not be opened", args[1])
        })
    };

    return Ok(());
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    
    let run = front_end(&args);
    let _run = match run {
        Ok(run) => run,
        Err(error) => {
            print!("{}", error.what());
            return;
        }
    };
}