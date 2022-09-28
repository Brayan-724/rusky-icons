use rusky_icons::{icon_dot, icon_error, icon_right_arrow, icon_sm_right_arrow, icon_success};

fn main() {
    icon_success::test_print_str("Success").unwrap();
    icon_error::test_print_str("Error").unwrap();
    icon_sm_right_arrow::test_print_str("Small right arrow").unwrap();
    icon_right_arrow::test_print_str("Right arrow").unwrap();
    icon_dot::test_print_str("Dot").unwrap();
}
