fn main() {
    println!("Visiting {}", restaurant::NAME);
    println!("Visiting {}", restaurant::RESTAURANT_NAME);
    restaurant::eat_at_restaurant();
    restaurant::customer::eat_at_restaurant();
}
