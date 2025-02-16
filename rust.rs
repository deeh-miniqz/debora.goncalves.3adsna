fn reverse_string(s: String) -> String {
    s.chars().rev().collect()
}

fn main() {
    let input = String::from("Olá, mundo!");
    let invertida = reverse_string(input);

    println!("String invertida: {}", invertida);
}
