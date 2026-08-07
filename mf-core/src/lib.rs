pub mod convert;
pub mod format;
pub mod job;
pub mod message;
pub mod plugin;
pub mod runner;
pub mod validation;
pub mod settings;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
