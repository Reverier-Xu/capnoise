fn main() {
  let images = capnoise::generate_full("ABCD");
  for (i, img) in images.iter().enumerate() {
    let filename = format!("tests/output_{}.bmp", i);
    std::fs::write(&filename, img).expect("Failed to write image file");
    println!("Wrote {}", filename);
  }
}
