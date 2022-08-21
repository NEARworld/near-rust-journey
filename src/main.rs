use std::io;
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
    fn is_habitable(self: &Self) -> bool {
        matches!(self.habitat, Habitat::Habitable)
    }
}

fn main() {

    println!("::::: Welcome to Space Colony game :::::");
    println!("Let's generate your own planet! Write your planet name.");
    let mut planet_name = String::new();
    io::stdin().read_line(&mut planet_name).expect("Something went wrong");
    println!("Ok! Your planet name is <{}>.", planet_name.trim());

    // random Habitat variant to the gamer's planet.
    let num: u32 = rand::thread_rng().gen_range(0..=1);

    let mother_planet = Planet {
        habitat: match num {
            0 => Habitat::Habitable,
            1 => Habitat::Uninhabitable,
            2_u32..=u32::MAX => panic!("There's no variant for this random number.")
        },
        name: planet_name
    };
    
    println!("result: {}", mother_planet.is_habitable());
}