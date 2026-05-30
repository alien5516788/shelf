fn main() {
    println!("Building icons..");

    iced_lucide::build("fonts/icons.toml")
        .expect("failed to generate icon module");
}
