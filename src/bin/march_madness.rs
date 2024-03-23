fn main() {
    let mut max_strength: f64 = f64::MAX;
    let mut max_seeds = vec![];

    for seed in 1..=16 {
        let mut strength = 1.0;

        for round in 1..=4 {
            let opp_seed = get_opponent_seed(seed, round);
            strength *= opp_seed as f64;
        }

        // Higher number is easier schedule
        if strength < max_strength {
            max_strength = strength;
            max_seeds = vec![seed];
        } else if strength == max_strength {
            max_seeds.push(seed);
        }
    }

    println!("Toughest schedule for seeds {:?}", max_seeds);
}

fn get_opponent_seed(seed: i32, round: i32) -> i32 {
    match round {
        1 => 17 - seed,
        2 => {
            // Shouldn't have won round 1
            let true_seed = if seed > 8 {
                get_opponent_seed(seed, 1)
            } else {
                seed
            };
            9 - true_seed
        }
        3 => {
            // Shouldn't have won round 1
            let true_one = if seed > 8 {
                get_opponent_seed(seed, 1)
            } else {
                seed
            };
            // Shouldn't have won round 2
            let true_seed = if true_one > 4 {
                get_opponent_seed(true_one, 2)
            } else {
                true_one
            };
            5 - true_seed
        }
        4 => {
            // Shouldn't have won round 1
            let true_one = if seed > 8 {
                get_opponent_seed(seed, 1)
            } else {
                seed
            };
            // Shouldn't have won round 2
            let true_two = if true_one > 4 {
                get_opponent_seed(true_one, 2)
            } else {
                true_one
            };
            // Shouldn't have won round 3
            let true_seed = if true_two > 2 {
                get_opponent_seed(true_two, 3)
            } else {
                true_two
            };
            3 - true_seed
        }
        _ => panic!("Invalid round {}", round),
    }
}
