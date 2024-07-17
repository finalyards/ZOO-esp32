/*
*
*/

pub type Result = core::result::Result<(),X>;     // just an Ok | Err(X)

enum X {
    A(u8)
}
