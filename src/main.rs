use std::io;

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
        match self.habitat {
            Habitat::Habitable => true,
            _ => false
        }
    }
}

fn main() {

    println!("::::: Welcome to Space Colony game :::::");
    println!("Let's generate your own planet! Write your planet name.");
    let mut planet_name = String::from("");
    io::stdin().read_line(&mut planet_name).expect("Something went wrong");
    planet_name.pop();
    println!("Ok! Your planet name is {}.", planet_name);

    let mother_planet = Planet {
        habitat: Habitat::Habitable,
        name: planet_name
    };
    
    println!("result: {}", mother_planet.is_habitable());
}