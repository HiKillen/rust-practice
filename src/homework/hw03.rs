const WIDTH: usize = 30;
const HEIGHT: usize = 15;

fn main() {
    let mut output = String::new();

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            // Малюємо рамку
            if y == 0 || y == HEIGHT - 1 || x == 0 || x == WIDTH - 1 {
                output.push('*');
            }
            // Малюємо діагоналі (лінії конверта)
            else if x == y * (WIDTH - 1) / (HEIGHT - 1) || x == (WIDTH - 1) - y * (WIDTH - 1) / (HEIGHT - 1) {
                output.push('*');
            } else {
                output.push(' ');
            }
        }
        output.push('\n');
    }

    print!("{}", output);
}
