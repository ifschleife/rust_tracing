fn main() {
    const NX: u32 = 200;
    const NY: u32 = 100;

    println!("P3\n{} {}\n255\n", NX, NY);
    for y in (0..NY).rev() {
        for x in 0..NX {
            let r = x as f32 / NX as f32;
            let g = y as f32 / NY as f32;
            let b = 0.2;
            let r = (255.99 * r) as u32;
            let g = (255.99 * g) as u32;
            let b = (255.99 * b) as u32;
            println!("{} {} {}", r, g, b);
        }
    }
}
