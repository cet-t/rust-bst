pub mod dict;
pub mod traits;
pub mod trrne;

extern crate rand;
// use crate::trrne::trrne::print;

// use std::collections::HashMap;

use rand::Rng;
use std::string::String;
use trrne::{Vec2, V2};

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

fn header(title: &str) {
    let line = String::from("----------");
    let mut dst = String::new();
    dst.push_str(&line);
    dst.push_str(&title);
    dst.push_str(&line);
    println!("{}", dst);
}

fn footer() {
    println!("{}", String::from("--------------------------"));
}

fn main() {
    {
        header(&"lottery");
        let weights: &[f32] = &[100.0, 50.0, 1.0];
        let a: &[&str] = &["R", "SR", "SSR"];
        const A: Vec<&str> = Vec::new();
        let rarity: &mut [Vec<&str>] = &mut [A; 3];
        for _ in 0..1024 {
            let choice: &str = a[bst(weights) as usize];
            for i in 0..rarity.len() {
                if choice == a[i] {
                    rarity[i].push(choice);
                }
            }
        }
        for i in 0..rarity.len() {
            println!("{}: {}", a[i], rarity[i].len());
        }
        footer();
    }

    {
        header(&"V2");
        let (a, b, c): (&mut V2, &mut V2, f32) = (&mut V2::new(1., 4.), &mut V2::new(-2., 8.), 2.);
        println!("a: {}", a.to_str());
        println!("b: {}", b.to_str());
        println!("c: {}", c);
        println!("a+b: {}", (*a + *b).to_str());
        println!("a+c: {}", (*a + c).to_str());
        println!("a-b: {}", (*a - *b).to_str());
        println!("a-c: {}", (*a - c).to_str());
        println!("a*b: {}", (*a * *b).to_str());
        println!("a*c: {}", (*a * c).to_str());
        println!("a/b: {}", (*a / *b).to_str());
        println!("a/c: {}", (*a / c).to_str());
        *a += *b;
        println!("a+=b: {}", a.to_str());
        *a += c;
        println!("a+=c: {}", a.to_str());
        *a -= *b;
        println!("a-=b: {}", a.to_str());
        *a -= c;
        println!("a-=c: {}", a.to_str());
        *a *= *b;
        println!("a*=b: {}", a.to_str());
        *a *= c;
        println!("a*=c: {}", a.to_str());
        *a /= *b;
        println!("a/=b: {}", a.to_str());
        *a /= c;
        println!("a/=c: {}", a.to_str());
        println!("magnitude a: {}", a.magnitude());
        println!("normalize a: {}", a.normalize().to_str());
        println!("distance ab: {}", V2::distance(a, b));
        println!("dot ab: {}", V2::dot(a, b));
        println!("cross ab: {}", V2::cross(a, b));
        println!("a: {},b: {}", a.to_str(), b.to_str());
        println!("angle ab: {}", V2::angle(a, b));
        footer();
    }

    // println!("{:?}", trrne::trrne::print());
}
