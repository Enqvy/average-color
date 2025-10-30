use image::GenericImageView;
use std::collections::BTreeMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("need image path");
        return;
    }

    let mut img = image::open(&args[1]).unwrap();
    
    let max_dim = 3840;
    if img.width() > max_dim || img.height() > max_dim {
        let ratio = if img.width() > img.height() {
            max_dim as f32 / img.width() as f32
        } else {
            max_dim as f32 / img.height() as f32
        };
        let new_w = (img.width() as f32 * ratio) as u32;
        let new_h = (img.height() as f32 * ratio) as u32;
        img = img.resize(new_w, new_h, image::imageops::FilterType::Nearest);
    }
    
    let mut colors: BTreeMap<(u8,u8,u8), u32> = BTreeMap::new();
    
    for (_,_, pixel) in img.pixels() {
        if pixel[3] < 95 {
            continue;
        }
        let bucket = get_bucket((pixel[0], pixel[1], pixel[2]));
        *colors.entry(bucket).or_insert(0) += 1;
    }

    let mut combined: Vec<_> = colors.into_iter().collect();

    combined.sort_unstable_by(|a,b| b.1.cmp(&a.1));

    let max = if combined.len() < 15 { combined.len() } else { 15 };
    
    for i in 0..max {
        let ((r,g,b), cnt) = combined[i];
        println!("#{:02x}{:02x}{:02x} - {} pixels", r,g,b, cnt);
    }
}

fn get_bucket(c: (u8,u8,u8)) -> (u8,u8,u8) {
    ((c.0 / 5) * 5, (c.1 / 5) * 5, (c.2 / 5) * 5)
}