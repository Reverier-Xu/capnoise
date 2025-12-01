pub mod converter;
pub mod generator;
pub mod resources;

/// generate_single will generate a single image with 0 offset
pub fn generate_single(content: impl AsRef<str>) -> Vec<u8> {
  let len = content.as_ref().len();
  let cover = generator::get_cover(&content);
  let rect = generator::random_rect(len);
  let merged_bits = generator::merge_cover_and_rect(&cover, &rect, &rect, len, 0);
  converter::convert_to_bmp_1bit(&merged_bits, 32 * (len + 1), 68)
}

/// generate_with_offset will generate a single image with specified offset
pub fn generate_with_offset(content: impl AsRef<str>, offset: usize) -> Vec<u8> {
  let len = content.as_ref().len();
  let cover = generator::get_cover(&content);
  let rect = generator::random_rect(len);
  let merged_bits = generator::merge_cover_and_rect(&cover, &rect, &rect, len, offset);
  converter::convert_to_bmp_1bit(&merged_bits, 32 * (len + 1), 68)
}

/// generate_full will generate a image sequence with 0..32 offsets
pub fn generate_full(content: impl AsRef<str>) -> Vec<Vec<u8>> {
  let len = content.as_ref().len();
  let cover = generator::get_cover(&content);
  let rect = generator::random_rect(len);
  let mut result = Vec::new();
  for offset in 0..32 {
    let merged_bits = generator::merge_cover_and_rect(&cover, &rect, &rect, len, offset);
    let bmp_data = converter::convert_to_bmp_1bit(&merged_bits, 32 * (len + 1), 68);
    result.push(bmp_data);
  }
  result
}
