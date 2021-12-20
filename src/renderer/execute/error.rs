use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ComputeError {
    // Represents an issue, defect or system issue, within the render threads.
    #[error("A compute thread panicked.")]
    ThreadPanickedError,

    // Represents any other io error
    #[error(transparent)]
    CommunicationError(#[from] std::sync::mpsc::RecvError),
}
