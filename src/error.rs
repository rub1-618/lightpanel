#[derive(Debug, Clone)]
pub enum LpnlError {
    SetupError      { message: String, kind: SetupErrorKind },
    DirInitError    { message: String, kind: DirInitErrorKind },
    InitError       { message: String, kind: InitErrorKind },
    RemoveError     { message: String, kind: RemoveErrorKind },
    ListError       { message: String, kind: ListErrorKind },
    // No error for stats (yet).
}

pub fn report_error(error: LpnlError) {
    match error {
        LpnlError::SetupError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(1);
        }
        LpnlError::DirInitError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(2);
        },
        LpnlError::InitError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(3);
        },
        LpnlError::RemoveError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(4);
        },
        LpnlError::ListError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(5);
        },
    }    
}

#[derive(Debug, Clone)]
pub enum SetupErrorKind {
    FsFailure,
    InvalidCmdResult,
    ReadError,
    NotFound,
    ConvertionFailure,
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
    ConvertionFailure,
    InvalidCmdResult,

    #[allow(dead_code)]
    InvalidDomain,
    #[allow(dead_code)]
    InvalidPort,
    #[allow(dead_code)]
    InvalidRoot,
}

#[derive(Debug, Clone)]
pub enum RemoveErrorKind {
    FsFailure,
    IoFailure,
    InvalidCmdResult,

    #[allow(dead_code)]
    InvalidDomain,
}

#[derive(Debug, Clone)]
pub enum ListErrorKind {
    FsFailure,
    IoFailure,
    InvalidCmdResult,

    #[allow(dead_code)]
    InvalidDomain,
}

