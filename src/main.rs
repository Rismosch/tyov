mod pcg;
pub mod rng;

use crate::rng::Rng;
use crate::rng::Seed;

fn main() {
    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let mut rng = Rng::new(seed);

    println!("press enter for a new random number...");

    loop {
        let mut input = String::new();
        if let Err(e) = std::io::stdin().read_line(&mut input) {
            println!("error: {}", e);
            return;
        }

        let (w10, w6) = roll_dice(&mut rng);
        let n = w10 - w6;
        let w10 = w10 % 10;
        println!("{} - {} = {}", w10, w6, n);
    }
}

fn roll_dice(rng: &mut Rng) -> (i32, i32) {
    let w10 = rng.next_i32_between(1, 10);
    let w6 = rng.next_i32_between(1, 6);
    (w10, w6)
}

#[test]
fn should_return_all_expected_numbers() {
    const ATTEMPTS: usize = 1000;

    let mut expected = vec![-5, -4, -3, -2, -1, 0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let seed = Seed::new();
    println!("seed: {:?}", seed);
    let mut rng = Rng::new(seed);

    for _ in 0..ATTEMPTS {
        let (w10, w6) = roll_dice(&mut rng);
        let n = w10 - w6;

        let Some(position) = expected.iter().position(|x| *x == n) else {
            continue;
        };

        expected.swap_remove(position);

        if expected.is_empty() {
            return; // success, all numbers have been hit!
        }
    }

    expected.sort();
    panic!("the following numbers have not been hit: {:?}", expected);
}
