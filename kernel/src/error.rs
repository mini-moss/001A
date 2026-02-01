#[derive(Debug, Clone, Copy)]
pub enum Errno {

}

#[derive(Debug, Clone, Copy)]
pub struct Error {
    error: Errno,
    msg: Option<&'static str>,
}