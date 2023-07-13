use std::io;

const CHARACTER_MOVE_SPEED: i32 = 1000;
const CHARACTER_KILL_RANGE: i32 = 2000;
const ZOMBIE_MOVE_SPEED: i32 = 400;
const ZOMBIE_KILL_RANGE: i32 = 400;

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

#[derive(Clone, Copy, Debug)]
struct Position(i32, i32);

#[derive(Clone, Copy, Debug)]
struct Entity {
    id: i32,
    position: Position
}

struct Human {
    entity: Entity,
    zombies_facing_human: Option<i32>
}

#[derive(Clone, Debug)]
struct Zombie {
    entity: Entity,
    position_next: Position,
    distance_to_character: Option<i32>,
    distances_to_humans: Option<Vec<i32>>
}

impl Zombie {
    fn calculate_danger_level(
        mut self, 
        humans: &Vec<Human>,
        character_position: Position
    ) -> Zombie {
        let distance_x_character: i32 = character_position.0 - self.entity.position.0;
        let distance_y_character: i32 = character_position.0 - self.entity.position.0;
        let distance_character: i32 = pythagorean_theorem(distance_x_character, distance_y_character);
        let mut distances: Vec<i32> = vec![];
        for human in humans.iter() {
            let distance_x_human: i32 = 
                human.entity.position.0 - self.position_next.0;
            let distance_y_human: i32 = 
                human.entity.position.1 - self.position_next.1;
            let distance_human: i32 = pythagorean_theorem(distance_x_human, distance_y_human);
            
            if distance_human / (ZOMBIE_KILL_RANGE+ZOMBIE_MOVE_SPEED) > distance_character / (CHARACTER_KILL_RANGE + CHARACTER_MOVE_SPEED) {
                distances.extend([distance_human]);
            }
        }
        self.distance_to_character = Option::from(distance_character);
        if distances.len() == 0 {
            return self;
        }
        distances.sort();
        self.distances_to_humans = Option::from(distances);
        self
    }

    fn is_zombie_targetting_character(&self, character_position: Position) -> bool {
        distance(character_position, self.position_next) 
            + distance(self.entity.position, self.position_next) 
        == distance(character_position, self.entity.position)
    }
}

/**
 * Save humans, destroy zombies!
 **/
fn main() {

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        // let x = parse_input!(inputs[0], i32);
        // let y = parse_input!(inputs[1], i32);
        let character_position: Position = Position(
            parse_input!(inputs[0], i32),
            parse_input!(inputs[1], i32)
        );
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let human_count = parse_input!(input_line, i32);
        let mut humans: Vec<Human> = vec![];
        for i in 0..human_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let human_id = parse_input!(inputs[0], i32);
            let human_x = parse_input!(inputs[1], i32);
            let human_y = parse_input!(inputs[2], i32);
            let new_entity = Human{
                entity: Entity {
                    id: human_id,
                    position: Position(human_x, human_y)
                },
                zombies_facing_human: Option::None
            };
            humans.extend([new_entity]);
        }
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let zombie_count = parse_input!(input_line, i32);
        let mut zombies: Vec<Zombie> = vec![];
        for i in 0..zombie_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let zombie_id = parse_input!(inputs[0], i32);
            let zombie_x = parse_input!(inputs[1], i32);
            let zombie_y = parse_input!(inputs[2], i32);
            let zombie_xnext = parse_input!(inputs[3], i32);
            let zombie_ynext = parse_input!(inputs[4], i32);
            let new_entity = Zombie {
                entity: Entity { 
                    id: zombie_id, 
                    position: Position(zombie_x, zombie_y)
                },
                position_next: Position(zombie_xnext, zombie_ynext),
                distance_to_character: Option::None,
                distances_to_humans: Option::None
            };
            let new_zombie = new_entity.calculate_danger_level(
                &humans,
                Position(
                    character_position.0, 
                    character_position.1
                )
            );
            zombies.extend([new_zombie])
        }
        zombies.sort_by(|a,b| {
            let a_human_dist = a.distances_to_humans.as_ref().unwrap_or(&vec![i32::MAX])[0];
            let b_human_dist = b.distances_to_humans.as_ref().unwrap_or(&vec![i32::MAX])[0];
            if (a_human_dist == b_human_dist) {
                a_human_dist.cmp(&b_human_dist)
            }
            else
            {
                a.distance_to_character.cmp(&b.distance_to_character)
            }
        });

        let mut target: Option<Position> = None;
        for zombie in zombies.iter() {
            if zombie.is_zombie_targetting_character(character_position) {
                target = Option::from(zombie.entity.position);
                break;
            }
        }

        let mut new_humans: Vec<Human> = vec![];
        for human in humans.iter() {
            let mut new_human: Human = Human {
                entity: human.entity,
                zombies_facing_human: Option::None
            };
            for zombie in zombies.iter() {
                if zombie.is_zombie_targetting_character(human.entity.position) {
                    match human.zombies_facing_human {
                        Some (amount) => new_human = {
                            Human {
                                entity: human.entity,
                                zombies_facing_human: Option::from(amount + 1)
                            }
                        },
                        None => new_human = Human {
                                entity: human.entity,
                                zombies_facing_human: Option::from(1)
                        }                        
                    }
                }
            }
            new_humans.extend([new_human]);
        }

        eprintln!("Humans {}", new_humans.len());
        for human in new_humans.iter() {
            eprintln!("Facing: {}", human.zombies_facing_human.unwrap_or(-1));
            if human.zombies_facing_human.unwrap_or(-1) > 0 {
                target = Option::from(human.entity.position);
            }
        }

        match target {
            Some(position) => {
                eprintln!("Target {:?}", position);
                println!(
                    "{} {}", 
                    position.0, 
                    position.1
                );
            },
            None => {
                let zombie: &Zombie = &zombies[0];
                println!(
                    "{} {}", 
                    zombie.entity.position.0, 
                    zombie.entity.position.1
                );
            }
        }

        // Write an action using println!("message...");
        // To debug: eprintln!("Debug message...");
        // println!(
        //     "{} {}", 
        //     target.entity.position.0, 
        //     target.entity.position.1
        // ); 
        // Your destination coordinates
    }
}

fn pythagorean_theorem(a: i32, b:i32) -> i32 {
    ((a.pow(2) + b.pow(2)) as f32).sqrt() as i32
}

fn distance(a: Position, b: Position) -> i32 {
    (((a.0 - b.0) + (a.1 - b.1))as f32).sqrt() as i32
}
