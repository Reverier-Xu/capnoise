import os


def generate_char(c: str) -> str:
    os.system(
        f"convert -resize 32x64\\! -font 'Iosevka-Bold' -pointsize 16 label:{c} result.xbm"
    )
    with open("result.xbm", "r") as f:
        data = f.read()
    os.remove("result.xbm")
    # get the data between {}
    data = data.split("{")[1].split("}")[0].strip()
    return data


header = """
use std::collections::HashMap;

use once_cell::sync::Lazy;

pub static PIXEL_CHAR_TABLE: Lazy<HashMap<char, [u8; 256]>> = Lazy::new(|| {
  let mut m = HashMap::new();
"""

footer = """
  m
});
"""


def main():
    # only [A-Za-z0-9]
    chars = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789"
    result = {}
    for c in chars:
        data = generate_char(c)
        result[c] = data
    with open("src/resources.rs", "w") as f:
        f.write(header)
        for c in chars:
            f.write(
                f"""
  m.insert(
    '{c}',
    [
      {result[c]}
    ],
  );
"""
            )
        f.write(footer)


if __name__ == "__main__":
    main()
