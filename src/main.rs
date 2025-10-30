use image::GenericImageView;
use std::collections::HashMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("need image path");
        return;
    }

    let img = image::open(&args[1]).unwrap();
    
    let mut colors: HashMap<(u8,u8,u8), u32> = HashMap::new();
    
    for (_,_, pixel) in img.pixels() {
        if pixel[3] < 95 {
            continue;
        }
        let rgb = (pixel[0], pixel[1], pixel[2]);
        *colors.entry(rgb).or_insert(0) += 1;
    }

    let mut combined: Vec<((u8,u8,u8), u32)> = vec![];
    
    for (color, count) in colors {
        let mut found = false;
        for i in 0..combined.len() {
            if close_enough(color, combined[i].0) {
                combined[i].1 += count;
                found = true;
                break;
            }
        }
        if !found {
            combined.push((color, count));
        }
    }

    combined.sort_by(|a,b| b.1.cmp(&a.1));

    let max = if combined.len() < 15 { combined.len() } else { 15 };
    
    for i in 0..max {
        let ((r,g,b), cnt) = combined[i];
        println!("#{:02x}{:02x}{:02x} - {} pixels", r,g,b, cnt);
    }
}

fn close_enough(c1: (u8,u8,u8), c2: (u8,u8,u8)) -> bool {
    let dr = (c1.0 as i32 - c2.0 as i32).abs();
    let dg = (c1.1 as i32 - c2.1 as i32).abs();
    let db = (c1.2 as i32 - c2.2 as i32).abs();
    dr < 5 && dg < 5 && db < 5
}
