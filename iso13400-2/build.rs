fn main() {
    let features = [
        std::env::var("CARGO_FEATURE_STD2010").is_ok(),
        std::env::var("CARGO_FEATURE_STD2012").is_ok(),
        std::env::var("CARGO_FEATURE_STD2019").is_ok(),
    ];

    let crate_name = std::env::var("CARGO_PKG_NAME").unwrap_or("iso13400-2".into());

    match features.iter()
        .filter(|&&en| en)
        .count() {
        1 => {},
        _ => panic!(
            "***`{}`*** at most one of the features `std2010` `std2012` or `std2019` can be enabled at a time.",
            crate_name
        )
    }
}
