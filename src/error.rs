#[derive(Debug, Clone)]
pub struct  InitializationError {
    pub message: String,
}

pub fn report_init(error: InitializationError) {
    let text = error.message;
    eprintln!("{text}");
    std::process::exit(1);
}

// todo: enum for all errors handling