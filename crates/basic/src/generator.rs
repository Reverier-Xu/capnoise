use rand::prelude::*;

use crate::resources::PIXEL_CHAR_TABLE;

pub fn random_rect(len: usize) -> Vec<u8> {
  let height = 68;
  let mut rng = rand::rng();
  // directly fill the vector with random data
  let mut result = vec![];
  let mut random_bytes = vec![0u8; 4];
  for _ in 0..height {
    rng.fill_bytes(&mut random_bytes);
    for _ in 0..=len {
      result.extend_from_slice(&random_bytes);
    }
  }
  result
}

pub fn get_cover(content: impl AsRef<str>) -> Vec<u8> {
  let len = content.as_ref().len();
  let width = 32 * (len + 1);
  let _height = 68;
  let mut pixels = Vec::new();
  for c in content.as_ref().chars() {
    if PIXEL_CHAR_TABLE.contains_key(&c) {
      // endianess conversion: from little endian to big endian for every byte
      let pixel = PIXEL_CHAR_TABLE
        .get(&c)
        .unwrap()
        .to_vec()
        .iter()
        .map(|b| b.reverse_bits())
        .collect::<Vec<u8>>();
      pixels.push(pixel);
    }
  }
  let mut result = Vec::new();
  result.extend_from_slice(&vec![0u8; width / 2]);
  for line in 0..64 {
    result.extend_from_slice(&[0u8; 2]);
    for pixel in &pixels {
      result.extend_from_slice(&pixel[line * 4..line * 4 + 4]);
    }
    result.extend_from_slice(&[0u8; 2]);
  }
  result
}

pub fn merge_cover_and_rect(
  cover: &[u8], front: &[u8], back: &[u8], len: usize, offset: usize,
) -> Vec<u8> {
  let width = 32 * (len + 1);
  let height = 68;
  let total_bits = width * height;
  let bytes_needed = total_bits.div_ceil(8);

  let offset_norm = if width == 0 { 0 } else { offset % width };

  let mut result = vec![0u8; bytes_needed];

  for line in 0..height {
    let line_base = line * width;

    for x in 0..width {
      let dst_bit_idx = line_base + x;

      let cover_byte_idx = dst_bit_idx / 8;
      let cover_bit_in_byte = 7 - (dst_bit_idx % 8);
      let cover_bit = (cover[cover_byte_idx] >> cover_bit_in_byte) & 1;

      let front_src_x = (x + offset_norm) % width;
      let front_src_bit = line_base + front_src_x;
      let front_byte_idx = front_src_bit / 8;
      let front_bit_in_byte = 7 - (front_src_bit % 8);
      let front_bit_val = (front[front_byte_idx] >> front_bit_in_byte) & 1;

      let back_src_x = (x + width - offset_norm) % width;
      let back_src_bit = line_base + back_src_x;
      let back_byte_idx = back_src_bit / 8;
      let back_bit_in_byte = 7 - (back_src_bit % 8);
      let back_bit_val = (back[back_byte_idx] >> back_bit_in_byte) & 1;

      let out_bit_val = if cover_bit == 1 {
        front_bit_val
      } else {
        back_bit_val
      };

      if out_bit_val == 1 {
        let out_byte_idx = dst_bit_idx / 8;
        let out_bit_in_byte = 7 - (dst_bit_idx % 8);
        result[out_byte_idx] |= 1 << out_bit_in_byte;
      }
    }
  }

  result
}
