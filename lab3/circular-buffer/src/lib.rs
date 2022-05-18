pub mod sol_with_default;
pub mod sol_with_option;


#[derive(Debug, PartialEq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}
