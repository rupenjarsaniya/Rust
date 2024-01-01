#[derive(Debug)]
enum MenuChoice {
    MainMenu,
    Start,
    Quit,
}

fn get_choice(input: &str) -> Result<MenuChoice, String> {
    match input {
        "mainmenu" => Ok(MenuChoice::MainMenu),
        "start" => Ok(MenuChoice::Start),
        "quit" => Ok(MenuChoice::Quit),
        _ => Err("Unknown choice".to_owned()),
    }
}

fn print_choice(choice: &MenuChoice) {
    println!("choice: {:?}", choice);
}

fn pick_choice(input: &str) -> Result<(), String> {
    let choice: MenuChoice = get_choice(input)?;
    print_choice(&choice);
    Ok(())
}

pub fn main() {
    let choice: Result<MenuChoice, _> = get_choice("mainmenu");
    match choice {
        Ok(inner_choice) => print_choice(&inner_choice),
        Err(e) => println!("error = {:?}", e),
    }

    let choice = pick_choice("mainmenu");
    println!("choice value: {:?}", choice);
}
