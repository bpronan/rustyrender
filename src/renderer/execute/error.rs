use thiserror::Error;

#[derive(Error, Debug)]
#[allow(dead_code)]
pub enum ComputeError {
    // Represents an issue, defect or system issue, within the render threads.
    #[error("A compute thread panicked.")]
    ThreadPanicked,

    // Represents any other io error
    #[error(transparent)]
    Communication(#[from] std::sync::mpsc::RecvError),

    // TODO: Remove this
    // Represents any other io error
    #[error(transparent)]
    IO(#[from] std::io::Error),
}
