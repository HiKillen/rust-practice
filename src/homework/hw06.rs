const TRIANGLES: usize = 5; 

fn main() {
    let mut width = 1;

    for triangle in 0..TRIANGLES {
        for i in 0..(triangle + 2) { 
            let stars = 1 + i * 2;
            let spaces = width + (TRIANGLES - triangle - 1) - i;
            println!(
                "{}{}",
                " ".repeat(spaces),
                "*".repeat(stars)
            );
        }
        width += 1; 
    }
}
