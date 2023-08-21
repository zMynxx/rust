fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn loop_fn() {
    let mut i = 1;
    loop {
        println!("Looping {}", i);
        i += 1;
        if i == 4 {
            break;
        }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn what_direction(dir: Direction) {
    match dir {
        Direction::Up => println!("Going Up"),
        Direction::Down => println!("Going Down"),
        Direction::Left => println!("Going Left"),
        Direction::Right => println!("Going Right"),
    }
}

enum Flavor {
    Sweet,
    Sour,
    Bitter,
    Salty,
}

enum Capacity {
    Milliliter,
    Liter,
    Gallon,
}

struct Drink {
    flavor: Flavor,
    capacity: Capacity,
}

fn what_flavor(drink: Drink) {
    match drink.flavor {
        Flavor::Sweet => println!("Sweet"),
        Flavor::Sour => println!("Sour"),
        Flavor::Bitter => println!("Bitter"),
        Flavor::Salty => println!("Salty"),
    }
}

fn main() {
    println!("Fuck you {:?}", "Dude");
    println!("Result {:?}", add(5, 4));
    loop_fn();

    // let try = Direction::Down;
    // println!("{:?}", str(try));

    let coffy = Drink {
        flavor: Flavor::Bitter,
        capacity: Capacity::Milliliter,
    };

    what_flavor(coffy);
    let (x, y) = (69, 73);
    println!("{:?}, {:?}", x, y);
    println!("{:?}", if 5 > 4 { true } else { false })
}
