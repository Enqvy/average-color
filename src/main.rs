use image::GenericImageView;
use std::collections::BTreeMap;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let mut img_path = String::new();
    let mut show_pixels = false;
    let mut format = "text";
    let mut threshold = 5;
    let mut exclude_bw = false;
    
    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "--pixel" => show_pixels = true,
            "--json" => format = "json",
            "--csv" => format = "csv",
            "--xml" => format = "xml",
            "--toml" => format = "toml",
            "--threshold" => {
                i += 1;
                threshold = args[i].parse().unwrap();
            },
            "--no-bw" => exclude_bw = true,
            _ => img_path = args[i].clone(),
        }
        i += 1;
    }
    
    if img_path.is_empty() {
        println!("need image path");
        return;
    }

    let mut img = image::open(&img_path).unwrap();
    
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
    let mut total_pixels = 0;
    
    for (_,_, pixel) in img.pixels() {
        if pixel[3] < 95 {
            continue;
        }
        let bucket = get_bucket((pixel[0], pixel[1], pixel[2]), threshold);
        
        if exclude_bw && is_black_or_white(bucket) {
            continue;
        }
        
        *colors.entry(bucket).or_insert(0) += 1;
        total_pixels += 1;
    }

    let mut combined: Vec<_> = colors.into_iter().collect();
    combined.sort_unstable_by(|a,b| b.1.cmp(&a.1));

    let max = if combined.len() < 15 { combined.len() } else { 15 };
    
    match format {
        "json" => print_json(&combined, max, total_pixels, show_pixels),
        "csv" => print_csv(&combined, max, total_pixels, show_pixels),
        "xml" => print_xml(&combined, max, total_pixels, show_pixels),
        "toml" => print_toml(&combined, max, total_pixels, show_pixels),
        _ => print_text(&combined, max, total_pixels, show_pixels),
    }
}

fn get_bucket(c: (u8,u8,u8), threshold: i32) -> (u8,u8,u8) {
    let t = threshold as u8;
    ((c.0 / t) * t, (c.1 / t) * t, (c.2 / t) * t)
}

fn is_black_or_white(c: (u8,u8,u8)) -> bool {
    let sum = c.0 as u32 + c.1 as u32 + c.2 as u32;
    sum < 30 || sum > 735
}

fn print_text(colors: &[((u8,u8,u8), u32)], max: usize, total: u32, show_pixels: bool) {
    for i in 0..max {
        let ((r,g,b), cnt) = colors[i];
        let pct = (cnt as f32 / total as f32) * 100.0;
        if show_pixels {
            println!("#{:02x}{:02x}{:02x} - {:.2}% ({} pixels)", r,g,b, pct, cnt);
        } else {
            println!("#{:02x}{:02x}{:02x} - {:.2}%", r,g,b, pct);
        }
    }
}

fn print_json(colors: &[((u8,u8,u8), u32)], max: usize, total: u32, show_pixels: bool) {
    println!("[");
    for i in 0..max {
        let ((r,g,b), cnt) = colors[i];
        let pct = (cnt as f32 / total as f32) * 100.0;
        print!("  {{\"hex\": \"#{:02x}{:02x}{:02x}\", \"percentage\": {:.2}", r,g,b, pct);
        if show_pixels {
            print!(", \"pixels\": {}", cnt);
        }
        if i < max - 1 {
            println!("}},");
        } else {
            println!("}}");
        }
    }
    println!("]");
}

fn print_csv(colors: &[((u8,u8,u8), u32)], max: usize, total: u32, show_pixels: bool) {
    if show_pixels {
        println!("hex,percentage,pixels");
    } else {
        println!("hex,percentage");
    }
    for i in 0..max {
        let ((r,g,b), cnt) = colors[i];
        let pct = (cnt as f32 / total as f32) * 100.0;
        if show_pixels {
            println!("#{:02x}{:02x}{:02x},{:.2},{}", r,g,b, pct, cnt);
        } else {
            println!("#{:02x}{:02x}{:02x},{:.2}", r,g,b, pct);
        }
    }
}

fn print_xml(colors: &[((u8,u8,u8), u32)], max: usize, total: u32, show_pixels: bool) {
    println!("<?xml version=\"1.0\"?>");
    println!("<colors>");
    for i in 0..max {
        let ((r,g,b), cnt) = colors[i];
        let pct = (cnt as f32 / total as f32) * 100.0;
        print!("  <color hex=\"#{:02x}{:02x}{:02x}\" percentage=\"{:.2}\"", r,g,b, pct);
        if show_pixels {
            print!(" pixels=\"{}\"", cnt);
        }
        println!("/>");
    }
    println!("</colors>");
}

fn print_toml(colors: &[((u8,u8,u8), u32)], max: usize, total: u32, show_pixels: bool) {
    for i in 0..max {
        let ((r,g,b), cnt) = colors[i];
        let pct = (cnt as f32 / total as f32) * 100.0;
        println!("[[color]]");
        println!("hex = \"#{:02x}{:02x}{:02x}\"", r,g,b);
        println!("percentage = {:.2}", pct);
        if show_pixels {
            println!("pixels = {}", cnt);
        }
        if i < max - 1 {
            println!();
        }
    }
}