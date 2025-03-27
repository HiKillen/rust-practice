const WIDTH: usize = 11; 
const HEIGHT: usize = 6;  

fn main() {
    let mut output = String::new();

    for i in 0..HEIGHT {
        let spaces = (WIDTH - (2 * i + 1)) / 2;
        output.push_str(&" ".repeat(spaces));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }

    for i in (0..HEIGHT - 1).rev() {
        let spaces = (WIDTH - (2 * i + 1)) / 2;
        output.push_str(&" ".repeat(spaces));
        output.push_str(&"*".repeat(2 * i + 1));
        output.push('\n');
    }

    print!("{}", output);
}
