fn main() {
    // Class 133
    let first_name = {
        let action_hero = "Silvester Stallone";
        &action_hero[0..9]
    };
    println!("{}", first_name);
}
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
