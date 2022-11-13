pub fn log_types() {
  let slice = "Hello!";
  println!("Slice is {} bytes and also {} characters.", slice.len(), slice.chars().count()); // slice est de 6 octets et également de 6 caractères
  let slice2 = "안녕!";
  println!("Slice2 is {} bytes but only {} characters.", slice2.len(), slice2.chars().count()); // slice2 est de 7 octets mais seulement 3 caractères
}
