mod lib;

fn main() {
  // Statements here are executed when the compiled binary is called

  // Print text to the console
  let shirt: lib::items::Shirt = lib::create_large_shirt("red".to_string());

  let serilize: Vec<u8> = lib::serialize_shirt(&shirt);
  println!("{:?}", serilize);

  let result: Result<lib::items::Shirt, prost::DecodeError> = lib::deserialize_shirt(&serilize);

  match result {
    Ok(v) => println!("working with version {}", v.color),
    Err(e) => println!("error parsing header: {}", e),
  }
}
