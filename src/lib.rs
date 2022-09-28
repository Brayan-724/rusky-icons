macro_rules! create_icon {
    ($ICON: ident, $NAME: ident, $CHAR: expr, $STR: expr) => {
        pub mod $NAME {
            pub const CHAR: char = $CHAR;
            pub const STR: &'static str = $STR;

            /// Apply icon to the input text.
            /// `{ICON} {TEXT}` (With one space between they)
            pub fn apply<S: Into<String>>(text: S) -> String {
                format!("{} {}", STR, text.into())
            }

            /// Alias to 'apply'
            pub fn a<S: Into<String>>(text: S) -> String {
                format!("{} {}", STR, text.into())
            }

            pub fn test_print_str(name: &str) -> ::std::io::Result<()> {
                let icon_name = stringify!($NAME);
                println!("{}", $crate::format_icon!(@ $ICON; "{} (rusky_icons::{})", name, icon_name));
                Ok(())
            }
        }

        pub const $ICON: &'static str = $STR;
    };
}

create_icon!(SUCCESS, icon_success, '✔', "✔");
create_icon!(ERROR, icon_error, '✘', "✘");
create_icon!(SM_RIGHT_ARROW, icon_sm_right_arrow, '›', "›");
create_icon!(RIGHT_ARROW, icon_right_arrow, '❯', "❯");
create_icon!(DOT, icon_dot, '·', "·");

#[macro_export]
macro_rules! format_icon {
    (@ $ICON:ident; $($EX:tt)+) => ({
        let formatted = format!($($EX)+);
        format!("{} {}", $crate :: $ICON, formatted)
    })
}
