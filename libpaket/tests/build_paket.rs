#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use std::path::Path;

    use libpaket::build::*;

    macro_rules! folder_test {
        ($name:tt) => {
            #[test]
            fn $name() {
                let _ = create_paket_from_toml(Path::new(concat!(
                    "./example_pakets/",
                    stringify!($name)
                )))
                .unwrap();
            }
        };
    }

    folder_test!(configuration_paket);
    folder_test!(configuration_paket_with_user_home);
    folder_test!(application_paket);
    folder_test!(application_paket_with_assets);
    folder_test!(script_paket);
    folder_test!(script_paket_with_assets);
}
