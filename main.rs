You need to install grc!
// Fill the blanks
enum Direction {
    East,
    West,
    North,
    South,
}

fn main() {
    let dire = Direction::South;
    match dire {
        Direction::East => println!("East"),
        Direction::South | Direction::North => {
            // Matching South or North here
            println!("South or North");
        }
        _ => println!("West is match"),
    };
}
