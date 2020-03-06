use crate::lint::core::Filter;

pub mod filter_nothing;
pub mod filter_test;

pub fn get_all_filters() -> Vec<Box<dyn Filter>> {
    vec![Box::new(filter_test::TestModuleFilter)]
}
