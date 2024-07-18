/*
*
*/

pub type Result = core::result::Result<(),XError>;     // just an Ok | Err(X)

enum XError {
    // tbd. an entry for each kind of error that can be produced in the outer API
    //A(u8)
}
