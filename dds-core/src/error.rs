pub struct BackendError {

}

impl std::error::Error for BackendError {}
impl std::fmt::Debug for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("BackendError").finish()
    }
}
impl std::fmt::Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "BackendError")
    }
}