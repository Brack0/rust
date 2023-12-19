mod io {
    pub fn read_line() -> String {
        let str = std::io::stdin()
            .lines()
            .next()
            .expect("end of input")
            .expect("error reading from stdin");

        str.split('\n').next().expect("!").to_string()
    }
}

mod weapon {
    use std::fmt;

    #[derive(PartialEq, Clone, Copy, Debug)]
    pub enum Weapon {
        Rock = 1,
        Paper = 2,
        Scissors = 3,
        Lizard = 4,
        Spock = 5,
    }

    impl Weapon {
        pub fn beats(&self, weapon: &Weapon) -> bool {
            match *self {
                Weapon::Rock => *weapon == Weapon::Scissors || *weapon == Weapon::Lizard,
                Weapon::Paper => *weapon == Weapon::Rock || *weapon == Weapon::Spock,
                Weapon::Scissors => *weapon == Weapon::Paper || *weapon == Weapon::Lizard,
                Weapon::Lizard => *weapon == Weapon::Paper || *weapon == Weapon::Spock,
                Weapon::Spock => *weapon == Weapon::Rock || *weapon == Weapon::Scissors,
            }
        }
    }

    impl fmt::Display for Weapon {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "[{}] {:?}", *self as u8, self)
        }
    }

    impl TryFrom<i32> for Weapon {
        type Error = &'static str;

        fn try_from(value: i32) -> Result<Self, Self::Error> {
            match value {
                1 => Ok(Weapon::Rock),
                2 => Ok(Weapon::Paper),
                3 => Ok(Weapon::Scissors),
                4 => Ok(Weapon::Lizard),
                5 => Ok(Weapon::Spock),
                _ => Err("Unable to determine weapon"),
            }
        }
    }

    #[cfg(test)]
    mod test {
        use super::*;

        #[test]
        fn array_starts_at_0() {
            assert_eq!(Weapon::try_from(0), Err("Unable to determine weapon"))
        }

        #[test]
        fn valid_weapon() {
            assert_eq!(Weapon::try_from(3), Ok(Weapon::Scissors))
        }

        #[test]
        fn off_by_one_weapon() {
            assert_eq!(Weapon::try_from(6), Err("Unable to determine weapon"))
        }

        #[test]
        fn the_answer() {
            assert_eq!(Weapon::try_from(42), Err("Unable to determine weapon"))
        }

        #[test]
        fn format() {
            assert_eq!(Weapon::Lizard.to_string(), "[4] Lizard")
        }
    }
}

mod game {
    use crate::{io::read_line, weapon::Weapon};

    fn pick_weapon(player_name: &str) -> Result<Weapon, &'static str> {
        println!("{}, choose your weapon :", player_name);
        println!("{}", Weapon::Rock);
        println!("{}", Weapon::Paper);
        println!("{}", Weapon::Scissors);
        println!("{}", Weapon::Lizard);
        println!("{}", Weapon::Spock);

        Weapon::try_from(read_line().parse::<i32>().unwrap())
    }

    pub fn run() {
        let weapon_p1 = &pick_weapon("P1").unwrap();
        let weapon_p2 = &pick_weapon("P2").unwrap();

        if weapon_p1.beats(weapon_p2) {
            println!("P1 wins !");
        } else if weapon_p2.beats(weapon_p1) {
            println!("P2 wins !");
        } else {
            println!("Draw !");
        }
    }
}

fn main() {
    game::run();
}
