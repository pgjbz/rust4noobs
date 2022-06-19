fn main() {
    let avg = calc_avg(10_f32, 8.3, 5.4, 4.7);
    if approve(avg) {
        println!("Aprovado com {} de média", avg);
    } else {
        println!("Reprovado com {} de média", avg);
    }
}

fn calc_avg(n1: f32, n2: f32, n3: f32, n4: f32) -> f32 {
    (n1 + n2 + n3 + n4 ) / 4.0
}

fn approve(avg: f32) -> bool {
    avg >= 7.0
}