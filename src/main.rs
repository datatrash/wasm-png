use std::path::Path;
use fs_err as fs;
use std::io::{BufWriter, Write};
use libwebp_sys::WebPEncodeLosslessRGBA;
use lodepng::{ChunkPosition, ColorType, CompressSettings, FilterStrategy};

#[derive(argh::FromArgs)]
/// Compress wasm+js files into an executable png
struct Args {
    /// the name of the wasm file
    #[argh(option, default = "String::from(\"index.wasm\")")]
    wasm_file: String,

    /// the name of the js file
    #[argh(option, default = "String::from(\"index.js\")")]
    js_file: String,

    /// the maximum width of the png file
    #[argh(option, default = "4096")]
    max_width: u32
}

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    let mut data = vec![];
    let mut wasm_data = fs::read(args.wasm_file)?;
    let wasm_len = wasm_data.len();
    let mut js_data = fs::read(args.js_file)?;
    data.append(&mut wasm_data);
    data.append(&mut js_data);

    let (width, height, data) = split_data(data, args.max_width);

    unsafe {
        let mut out_buf = std::ptr::null_mut();
        let stride = width as i32;
        let len = WebPEncodeLosslessRGBA(data.as_ptr(), width as i32 >> 2, height as i32, stride, &mut out_buf);
        //let _ = std::slice::from_raw_parts(out_buf, len as usize).into();
        println!("WEBP: {:?}", len);
    }

    let depacker = include_str!("depacker.html");
    let depacker = depacker.replace("#{width}", &width.to_string());
    let depacker = depacker.replace("#{height}", &height.to_string());
    let depacker = depacker.replace("#{split}", &wasm_len.to_string());

    let path = Path::new(r"image.png.html");
    let file = fs::File::create(path)?;
    let mut w = BufWriter::new(file);

    let mut encoder = lodepng::Encoder::new();
    encoder.set_filter_strategy(FilterStrategy::BRUTE_FORCE, false);
    encoder.set_custom_zlib(Some(compress), 0 as *const _);
    let raw_info = encoder.info_raw_mut();
    raw_info.set_colortype(ColorType::GREY);
    raw_info.set_bitdepth(8);
    let info = encoder.info_png_mut();
    info.create_chunk(ChunkPosition::IHDR, &[b't', b'r', b's', b'h'], depacker.as_bytes())?;
    let bytes = encoder.encode(&data, width as usize, height as usize)?;
    w.write_all(&bytes)?;

    Ok(())
}

fn compress(input: &[u8], output: &mut dyn Write, _context: &CompressSettings) -> Result<(), lodepng::Error> {
    zopfli::compress(&zopfli::Options::default(), &zopfli::Format::Zlib, input, output)?;
    Ok(())
}

fn split_data(mut data: Vec<u8>, max_width: u32) -> (u32, u32, Vec<u8>) {
    let width = data.len() as u32;
    if width < max_width {
        (width, 1, data)
    } else {
        let mut height = 1;
        let mut cw = width;
        while cw > max_width {
            height += 1;
            cw -= max_width;
        }
        data.resize((max_width * height) as usize, 0);
        (max_width, height, data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split_data() {
        let data = vec![0, 1, 2, 3, 4, 5];
        let (width, height, split) = split_data(data, 4);
        assert_eq!(width, 4);
        assert_eq!(height, 2);
        assert_eq!(split, vec![0, 1, 2, 3, 4, 5, 0, 0]);
    }
}
