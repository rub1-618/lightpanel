#[derive(Debug, Clone)]
pub enum LpnlError {
    SetupError      { message: String, kind: SetupErrorKind         },
    DirInitError    { message: String, kind: DirInitErrorKind       },
    ValidationError { message: String, kind: ValidationErrorKind    },
    CommandError    { message: String, kind: CommandErrorKind       },
    BackupError     { message: String, kind: BackupErrorKind        },
    InitError       { message: String, kind: InitErrorKind          },
    RemoveError     { message: String, kind: RemoveErrorKind        },
    ListError       { message: String, kind: ListErrorKind          },
    AddLocError     { message: String, kind: AddLocErrorKind        },
    RemoveLocError  { message: String, kind: RemoveLocErrorKind     },
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
        LpnlError::ValidationError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(3);
        },
        LpnlError::CommandError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(4);
        },
        LpnlError::BackupError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(5);
        },
        LpnlError::InitError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(6);
        },
        LpnlError::RemoveError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(7);
        },
        LpnlError::ListError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(8);
        },
        LpnlError::AddLocError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(9);
        },
        LpnlError::RemoveLocError { message, kind } => {
            eprintln!("[{kind:?}] {message}");
            std::process::exit(9);
        },
    }    
}

#[derive(Debug, Clone)]
pub enum SetupErrorKind {
    FsFailure,
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
pub enum ValidationErrorKind {
    IoFailure,
    InvalidDomain,
    InvalidRoot,
    InvalidPort,
    InvalidProxy,
    InvalidLocation,
}

#[derive(Debug, Clone)]
pub enum CommandErrorKind {
    InvalidCmdResult,
}

#[derive(Debug, Clone)]
pub enum BackupErrorKind {
    FsFailure,
    NotFound,
}

#[derive(Debug, Clone)]
pub enum InitErrorKind {
    FsFailure,
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
    #[allow(dead_code)]
    InvalidDomain,
}

#[derive(Debug, Clone)]
pub enum ListErrorKind {
    FsFailure,
}

#[derive(Debug, Clone)]
pub enum AddLocErrorKind {
    FsFailure,
    IoFailure,
    InvalidInput,
    NotFound,
    AlreadyExists,
}

#[derive(Debug, Clone)]
pub enum RemoveLocErrorKind {
    FsFailure,
    NotFound,
}