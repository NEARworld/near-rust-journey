use std::{thread, time};
use std::io::{self, Write};
use rand::Rng;

enum Habitat {
    Habitable,
    Uninhabitable
}

struct Planet {
    habitat: Habitat,
    name: String
}
impl Planet {
    fn is_habitable(self) -> bool {
        matches!(self.habitat, Habitat::Habitable)
    }
}
enum Spaceship {
    Mothership,
    Colonyship
}
enum Building {
    Farm,
    Factory
}

fn construct_things() {
    let half_sec = time::Duration::from_secs_f32(0.5);
    print!("Constructing");
    std::io::stdout().flush().unwrap();
    thread::sleep(half_sec);
    for n in 1..4 {
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(half_sec);
    }
}

fn main() {

    println!("::::: Welcome to Space Colony game :::::");
    thread::sleep(time::Duration::from_secs(1));
    println!("Let's generate your own planet! Write your planet name.");
    let mut planet_name = String::new();
    io::stdin().read_line(&mut planet_name).expect("Something went wrong");
    println!("Ok! Your planet name is <{}>.", planet_name.trim());

    // random Habitat variant to the gamer's planet.
    let num: u32 = rand::thread_rng().gen_range(0..=0);

    let mother_planet = Planet {
        habitat: match num {
            0 => Habitat::Habitable,
            1 => Habitat::Uninhabitable,
            2_u32..=u32::MAX => panic!("There's no variant for this random number.")
        },
        name: planet_name
    };

    if mother_planet.is_habitable() {
        println!("Your planet is habitable. People started building a farm in this planet.");
        construct_things();
    } else {
        println!("kek! The planet is uninhabitable. People are trying to build a new colonyship.");
        construct_things();
        // println!("People are leaving {}", mother_planet.name);
    }
}