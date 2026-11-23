mod color;
mod vec3;
mod ray;

use std::io;
use color::Color;
use ray::Ray;
use vec3::{Point3, Vec3};


fn render_ppm_image(){
        const IMAGE_WIDTH: i32  = 956;
    const IMAGE_HEIGHT: i32 = 956;

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);
    
    for i in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {}", i);
        for j in 0..IMAGE_WIDTH {
            let r =  j as f64 / (IMAGE_WIDTH - 1) as f64;
            let g = i as f64 / (IMAGE_HEIGHT - 1) as f64;
            let b = 0.25;
            
            let pixel_color = Color::new(r, g, b); 
            color::write_color(&mut io::stdout(), pixel_color);
        }
    }
    eprint!("\nDone!\n");
}


fn ray_color(r: &Ray) -> Color {
    let unit_direction = vec3::unit_vector(r.direction())
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t* Color::new (0.5, 0.7, 1.0)
}

fn render_gradient_with_camera() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const 

}

fn main() {
    render_ppm_image()
}