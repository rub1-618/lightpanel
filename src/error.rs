#[derive(Debug, Clone)]
pub enum LpnlError {
    SetupError      { message: String, kind: SetupErrorKind         },
    DirInitError    { message: String, kind: DirInitErrorKind       },
    ValidationError { message: String, kind: ValidationErrorKind    },
    CommandError    { message: String, kind: CommandErrorKind       },
    BackupError     { message: String, kind: BackupErrorKind        },
    InitError       { message: String, kind: InitErrorKind          },
    StateError      { message: String, kind: StateErrorKind         },
    RemoveError     { message: String, kind: RemoveErrorKind        },
    ListError       { message: String, kind: ListErrorKind          },
    AddLocError     { message: String, kind: AddLocErrorKind        },
    RemoveLocError  { message: String, kind: RemoveLocErrorKind     },
}

pub fn report_error(error: LpnlError) { //todo: less 'eprintln()' copying
    let (message, kind, code) = match &error {
        LpnlError::SetupError       { message, kind }        => (message, format!("{:?}", kind), 1),
        LpnlError::DirInitError     { message, kind }      => (message, format!("{:?}", kind), 2),
        LpnlError::ValidationError  { message, kind }   => (message, format!("{:?}", kind), 3),
        LpnlError::CommandError     { message, kind }      => (message, format!("{:?}", kind), 4),
        LpnlError::BackupError      { message, kind }       => (message, format!("{:?}", kind), 5),
        LpnlError::InitError        { message, kind }         => (message, format!("{:?}", kind), 6),
        LpnlError::StateError       { message, kind }        => (message, format!("{:?}", kind), 7),
        LpnlError::RemoveError      { message, kind }       => (message, format!("{:?}", kind), 8),
        LpnlError::ListError        { message, kind }         => (message, format!("{:?}", kind), 9),
        LpnlError::AddLocError      { message, kind }       => (message, format!("{:?}", kind), 10),
        LpnlError::RemoveLocError   { message, kind }    => (message, format!("{:?}", kind), 11),
    };
    eprintln!("[{kind}] {message}");
    std::process::exit(code)
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
    FsFailure,
}

#[derive(Debug, Clone)]
pub enum BackupErrorKind {
    FsFailure,
    NotFound,
}

#[derive(Debug, Clone)]
pub enum InitErrorKind {
    FsFailure,
    AlreadyExists,
}

#[derive(Debug, Clone)]
pub enum StateErrorKind {
    FsFailure,
    NotFound,
}

#[derive(Debug, Clone)]
pub enum RemoveErrorKind {
    FsFailure,
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
    AlreadyExists,
}