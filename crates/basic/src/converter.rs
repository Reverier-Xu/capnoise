/// convert a 1-bit per pixel bitmap (black and white) to BMP format
pub fn convert_to_bmp_1bit(bits: &[u8], width: usize, height: usize) -> Vec<u8> {
  let total_bits = width * height;
  let src_bytes_needed = total_bits.div_ceil(8);
  assert!(
    bits.len() >= src_bytes_needed,
    "bits buffer too short for {}x{} image",
    width,
    height
  );

  // 1-bit: ceil(width / 8) with fixed to 4-byte alignment
  let row_bits = width;
  let row_bytes_raw = row_bits.div_ceil(8);
  let row_bytes_padded = (row_bytes_raw + 3) & !3;

  // header sizes:
  // - BITMAPFILEHEADER: 14 bytes
  // - BITMAPINFOHEADER: 40 bytes
  // - Color table: 2 * 4 bytes
  const FILE_HEADER_SIZE: usize = 14;
  const INFO_HEADER_SIZE: usize = 40;
  const COLOR_TABLE_SIZE: usize = 2 * 4;

  let pixel_array_size = row_bytes_padded * height;
  let bf_off_bits = (FILE_HEADER_SIZE + INFO_HEADER_SIZE + COLOR_TABLE_SIZE) as u32;
  let bf_size = bf_off_bits + pixel_array_size as u32;

  let mut bmp = Vec::with_capacity(bf_size as usize);

  // typedef struct {
  //   WORD  bfType;      // 'BM' = 0x4D42
  //   DWORD bfSize;      // file size
  //   WORD  bfReserved1; // 0
  //   WORD  bfReserved2; // 0
  //   DWORD bfOffBits;   // offset to pixel data
  // } BITMAPFILEHEADER;
  bmp.extend_from_slice(&0x4D42u16.to_le_bytes()); // bfType = 'BM'
  bmp.extend_from_slice(&bf_size.to_le_bytes()); // bfSize
  bmp.extend_from_slice(&0u16.to_le_bytes()); // bfReserved1
  bmp.extend_from_slice(&0u16.to_le_bytes()); // bfReserved2
  bmp.extend_from_slice(&bf_off_bits.to_le_bytes()); // bfOffBits

  assert_eq!(bmp.len(), FILE_HEADER_SIZE);

  // typedef struct {
  //   DWORD biSize;
  //   LONG  biWidth;
  //   LONG  biHeight;
  //   WORD  biPlanes;
  //   WORD  biBitCount;
  //   DWORD biCompression;
  //   DWORD biSizeImage;
  //   LONG  biXPelsPerMeter;
  //   LONG  biYPelsPerMeter;
  //   DWORD biClrUsed;
  //   DWORD biClrImportant;
  // } BITMAPINFOHEADER;
  bmp.extend_from_slice(&(INFO_HEADER_SIZE as u32).to_le_bytes()); // biSize
  bmp.extend_from_slice(&(width as i32).to_le_bytes()); // biWidth
  bmp.extend_from_slice(&(height as i32).to_le_bytes()); // biHeight
  bmp.extend_from_slice(&1u16.to_le_bytes()); // biPlanes = 1
  bmp.extend_from_slice(&1u16.to_le_bytes()); // biBitCount = 1 (1-bit)
  bmp.extend_from_slice(&0u32.to_le_bytes()); // biCompression = BI_RGB
  bmp.extend_from_slice(&(pixel_array_size as u32).to_le_bytes()); // biSizeImage
  bmp.extend_from_slice(&0i32.to_le_bytes()); // biXPelsPerMeter
  bmp.extend_from_slice(&0i32.to_le_bytes()); // biYPelsPerMeter
  bmp.extend_from_slice(&0u32.to_le_bytes()); // biClrUsed
  bmp.extend_from_slice(&0u32.to_le_bytes()); // biClrImportant

  assert_eq!(bmp.len(), FILE_HEADER_SIZE + INFO_HEADER_SIZE);

  bmp.extend_from_slice(&[255u8, 255u8, 255u8, 0u8]);
  bmp.extend_from_slice(&[0u8, 0u8, 0u8, 0u8]);

  assert_eq!(
    bmp.len(),
    FILE_HEADER_SIZE + INFO_HEADER_SIZE + COLOR_TABLE_SIZE
  );

  bmp.resize(bf_off_bits as usize + pixel_array_size, 0u8);

  for y in 0..height {
    let src_line = y;
    let src_line_base_bit = src_line * width;

    let dst_line = height - 1 - y;
    let dst_row_start = bf_off_bits as usize + dst_line * row_bytes_padded;

    let mut row_buf = vec![0u8; row_bytes_raw];

    for x in 0..width {
      let src_bit_idx = src_line_base_bit + x;
      let src_byte_idx = src_bit_idx / 8;
      let src_bit_in_byte = 7 - (src_bit_idx % 8);
      let bit_val = (bits[src_byte_idx] >> src_bit_in_byte) & 1;

      let dst_bit_idx = x;
      let dst_byte_idx = dst_bit_idx / 8;
      let dst_bit_in_byte = 7 - (dst_bit_idx % 8);

      if bit_val != 0 {
        row_buf[dst_byte_idx] |= 1 << dst_bit_in_byte;
      }
    }

    bmp[dst_row_start..dst_row_start + row_bytes_raw].copy_from_slice(&row_buf);
  }

  bmp
}

