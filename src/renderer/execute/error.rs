use thiserror::Error;


#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ComputeError {

    #[error("A compute thread panicked.")]
    ThreadPanickedError,

    // Represents any other io error
    #[error(transparent)]
    CommunicationError(#[from] std::sync::mpsc::RecvError),
}