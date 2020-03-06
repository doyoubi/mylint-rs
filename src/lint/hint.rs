pub const UNSAFE_HINT: &str = r#"
// For `pin_utils` crate,
// consider replacing it with `pin_project` or `pin_project_lite`
// since they can detect any unsafe usage.
"#;

pub const FUNCTOR_HINT: &str = r#"
// For Option or Result:

// Bad
if a.is_some() {
    let a = opt.unwrap();
    a.do_something();
}

// Good
if let Some(a) = opt {
    a.do_something();
}

// Good
opt.map(|a| a.do_something())?

// Good: tranform it to result
opt.map(|a| a.do_something()).ok_or_else(|| some_err)?

// Consider using `and_then` or `or_else` for nested Option and Result.
opt.and_then(|a| func_produce_option(a)) // result is a single layer Option
"#;

pub const INDEX_EXPR_HINT: &str = r#"
// Here variable `arr` can be array, Vec, String, and so on.

// Bad
if arr.len() >= 3 {
    let v = arr[2];
    v.do_something();
}

// Good
if let Some(v) = arr.get(2) {
    v.do_something();
}

// Bad
if "value" == &arr[..5] {
    do_something();
}

// Good
let sub_str = arr.get(..5).ok_or_else(|| some_err)?;
if "value" == sub_str {
    do_something();
}
"#;
