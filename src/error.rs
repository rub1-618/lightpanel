#[derive(Debug, Clone)]
pub struct  DirInitializationError {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct  InitializationError {
    pub message: String,
}

#[derive(Debug, Clone)]
pub struct  RemoveError {
    pub message: String,
}

pub fn report_dir_init(error: DirInitializationError) {
    let text = error.message;
    eprintln!("{text}");
    std::process::exit(1);
}

pub fn report_init(error: InitializationError) {
    let text = error.message;
    eprintln!("{text}");
    std::process::exit(2);
}

pub fn report_remove(error: RemoveError) {
    let text = error.message;
    eprintln!("{text}");
    std::process::exit(3);
}

// todo: enum for all errors handling