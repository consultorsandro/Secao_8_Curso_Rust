fn main() {
    // Class 135
    let action_hero = String::from("Silvester Stallone");

    let first_name: &str = &action_hero[..9]; // Considera do início até posição 9
    println!("His first name is: {}", first_name);

    let last_name: &str = &action_hero[10..]; // considera da posição 10 até a última
    println!("His last name is: {}", last_name);

    let full_name: &str = &action_hero[..]; // considera do início até o final
    println!("His full name is: {}", full_name);
}
/*
    // Class 134
    let food = "🍕";
    println!("{}", food.len());
*/
/*
    // Class 133
    let first_name = {
        let action_hero = "Silvester Stallone";
        &action_hero[0..9]
    };
    println!("{}", first_name);
*/
/*
    //Class 132
    let action_hero = String::from("Silvester Stallone");
    let first_name: &str = &action_hero[0..9];
    println!("First name: {}", first_name);

    let last_name  = &action_hero[10..18];
    println!("Last name: {}", last_name);
*/
