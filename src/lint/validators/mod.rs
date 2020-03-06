use crate::lint::core::Validator;

// #[macro_use]
// pub mod utils;

pub mod expect_call;
pub mod index_expr;
pub mod unsafe_code;
pub mod unwrap_call;
pub mod use_unsafe;

pub fn get_all_validators() -> Vec<Box<dyn Validator>> {
    vec![
        Box::new(unsafe_code::UnsafeCodeValidator),
        Box::new(use_unsafe::UseUnsafeValidator),
        Box::new(unwrap_call::UnwrapCallValidator),
        Box::new(expect_call::ExpectCallValidator),
        Box::new(index_expr::IndexExpressionValidator),
    ]
}
