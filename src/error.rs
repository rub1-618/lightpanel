#[derive(Debug, Clone)]
pub enum LpnlError {
    DirInitError { message: String, kind: DirInitErrorKind },
    InitError    { message: String, kind: InitErrorKind },
    RemoveError  { message: String, kind: RemoveErrorKind },
}

pub fn report_error(error: LpnlError) {
    match error {
        LpnlError::DirInitError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(1);
        },
        LpnlError::InitError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(2);
        },
        LpnlError::RemoveError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(3);
        },
    }    
}

#[derive(Debug, Clone)]
pub enum DirInitErrorKind {
    LpnlCreationFailure,
    BackupCreationFailure,
    RootCreationFailure,
}

#[derive(Debug, Clone)]
pub enum InitErrorKind {
    AlreadyExists,
    FsFailure,
    IoFailure,
    InvalidDomain,
    InvalidPort,
    InvalidRoot,
    ConvertionFailure,
    InvalidCmdResult,
}

#[derive(Debug, Clone)]
pub enum RemoveErrorKind {
    FsFailure,
    IoFailure,
    InvalidDomain,
    InvalidCmdResult,
}