/* 
Enums are types wich have few definite values
 */
enum Movement 

{  //Variants
    Up, 
    Down,
    Left,
    Right
}

fn move_avatar(m: Movement)
{
    // Perform action depending on the info

    match m {
        Movement::Up => println!("Avatar Moving Up"),
        Movement::Down => println!("Avatar Moving down"),
        Movement::Left => println!("Avatar Moving left"),
        Movement::Right => println!("Avatar Moving right")
    }
    }
    
    pub fn run()
    {
        let avatar1 = Movement::Left;
        let avatar2 = Movement::Up;
        let avatar3 = Movement::Right;
        let avatar4 = Movement::Down;

    move_avatar(avatar4);
    move_avatar(avatar2);
    move_avatar(avatar3);
    move_avatar(avatar1);
}