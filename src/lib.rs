//! Many function to generate test data for bioinformatics data

#![warn(missing_docs)]

/* std use */

/* crate use */

/* project use */
#[cfg(feature = "derive")]
pub use biommap_derive as derive;


/* mod declaration */
pub mod error;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
