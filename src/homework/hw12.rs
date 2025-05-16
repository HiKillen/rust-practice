use rand::Rng;

fn gen_shipments(n: usize) -> Vec<u32> {
    let mut rng = rand::thread_rng();
    let avg = rng.gen_range(10..=30); 

    let mut base: Vec<u32> = (0..n).map(|_| rng.gen_range(avg / 2..=avg * 3 / 2)).collect();

    let total: u32 = base.iter().sum();
    let desired_total = (total / n as u32) * n as u32;
    let diff = desired_total as i32 - total as i32;

    if diff != 0 {
        let idx = rng.gen_range(0..n);
        base[idx] = (base[idx] as i32 + diff).max(0) as u32;
    }

    assert!(base.iter().sum::<u32>() % n as u32 == 0);
    base
}

fn count_permutation(shipments: &Vec<u32>) -> Option<usize> {
    let total: u32 = shipments.iter().sum();
    let len = shipments.len() as u32;

    if total % len != 0 {
        return None;
    }

    let avg = total / len;
    let mut moves = 0;
    let mut balance: i32 = 0;

    for &shipment in shipments {
        balance += shipment as i32 - avg as i32;
        moves += balance.abs() as usize;
    }

    Some(moves)
}

fn print_result(shipments: &Vec<u32>) {
    let len = shipments.len();
    let total: u32 = shipments.iter().sum();
    let average = total / len as u32;

    println!("Shipments: {:?}", shipments);
    println!("Total   = {}", total);
    println!("Average = {}\n", average);

    match count_permutation(shipments) {
        Some(answer) => println!("Answer = {}\n{}\n", answer, "-".repeat(50)),
        None => println!("Cannot distribute equally\n{}\n", "-".repeat(50)),
    }
}

fn main() {
    let cases = vec![
        vec![8, 2, 2, 4, 4],
        vec![9, 3, 7, 2, 9],
        vec![30, 18, 68, 87, 99, 81, 88, 45, 34, 79, 81, 79, 93, 55, 26, 24, 32, 55, 59, 97],
        gen_shipments(5),
        gen_shipments(10),
        gen_shipments(20),
    ];

    for shipments in cases {
        print_result(&shipments);
    }
}
