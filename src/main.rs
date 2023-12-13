pub mod trrne;

extern crate rand;

use rand::Rng;
use trrne::V2;

fn bst(weights: &[f32]) -> i32 {
    let length: i32 = weights.len() as i32;
    if length <= 0 {
        return -1;
    }

    let mut totals: Vec<f32> = Vec::new();
    let mut total: f32 = 0.0;
    for i in 0..length {
        total += weights[i as usize];
        totals.push(total);
    }

    let mut rng = rand::thread_rng();
    let r: f32 = rng.gen_range(0.0..total);
    let mut bottom: i32 = 0;
    let mut top: i32 = length - 1;
    while bottom < top {
        let middle: i32 = (bottom + top) / 2;
        if r > totals[middle as usize] {
            bottom = middle + 1;
        } else {
            let p = if middle > 0 {
                totals[middle as usize - 1]
            } else {
                0.0
            };
            if r >= p {
                return middle;
            }
            top = middle;
        }
    }
    return top;
}

fn main() {
    // let weights: &[f32] = &[100.0, 50.0, 1.0];
    // let a: &[&str] = &["R", "SR", "SSR"];
    // const A: Vec<&str> = Vec::new();
    // let rarity = &mut [A; 3];
    // for _ in 0..1024 {
    //     let choice: &str = a[bst(weights) as usize];
    //     for i in 0..rarity.len() {
    //         if choice == a[i] {
    //             rarity[i].push(choice);
    //         }
    //     }
    // }
    // for i in 0..rarity.len() {
    //     println!("{}: {}", a[i], rarity[i].len());
    // }

    let a = &mut V2::new(0., 23.2);
    let b = &mut V2::new(1.3, 23.4);
    let aa = *a;
    let bb = *b;
    println!("{}", (aa - bb).to_str());
}
