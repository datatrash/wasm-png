use std::path::Path;
use fs_err as fs;
use std::io::BufWriter;

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

pub const TRSH: png::chunk::ChunkType = png::chunk::ChunkType([b't', b'r', b's', b'h']);

fn main() -> anyhow::Result<()> {
    let args: Args = argh::from_env();

    let mut data = vec![];
    let mut wasm_data = fs::read(args.wasm_file)?;
    let wasm_len = wasm_data.len();
    let mut js_data = fs::read(args.js_file)?;
    data.append(&mut wasm_data);
    data.append(&mut js_data);

    let (width, height, data) = split_data(data, args.max_width);

    let depacker = include_str!("depacker.html");
    let depacker = depacker.replace("#{width}", &width.to_string());
    let depacker = depacker.replace("#{height}", &height.to_string());
    let depacker = depacker.replace("#{split}", &wasm_len.to_string());

    let path = Path::new(r"image.png.html");
    let file = fs::File::create(path)?;
    let w = BufWriter::new(file);
    let mut encoder = png::Encoder::new(w, width, height);
    encoder.set_compression(png::Compression::Best);
    encoder.set_color(png::ColorType::Grayscale);
    encoder.set_depth(png::BitDepth::Eight);
    encoder.set_filter(png::FilterType::NoFilter);

    let mut writer = encoder.write_header()?;
    writer.write_chunk(TRSH, depacker.as_bytes())?;
    writer.write_image_data(&data)?;

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