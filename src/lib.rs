pub mod prelude {
    pub use components::prelude::*;
}

pub use components::*;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
