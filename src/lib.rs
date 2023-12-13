#[macro_export]
macro_rules! apply_if {
    ($cond:expr, $action:expr, $val:expr) => {
        if $cond {
            $action
        } else {
            $val
        }
    };
}
