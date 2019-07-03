//! A hexagonal grid representation based upon Red Blob Games' guide.

#![crate_name = "chickenwire"]
#![crate_type = "lib"]

mod hexgrid;
mod coordinate;

//////////////////////////////////////////////////////////////////////////////
// a
//////////////////////////////////////////////////////////////////////////////

//////////////////////////////////////////////////////////////////////////////
// Unit Tests
//////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
