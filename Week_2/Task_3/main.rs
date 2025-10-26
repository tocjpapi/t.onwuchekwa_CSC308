fn main() {
    let mut name = String::from("Firstname ");
    let returned_name = add_surname_to_firstname(& mut name);
    println!("{returned_name}");
}

fn add_surname_to_firstname(nom: & mut String) -> &String {
    nom.push_str("Lastname");
    nom
}
