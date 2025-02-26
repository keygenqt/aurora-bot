use ffmpeg::format::Pixel;
use ffmpeg::media::Type;
use ffmpeg::software::scaling::context::Context;
use ffmpeg::software::scaling::flag::Flags;
use ffmpeg::util::frame::video::Video;
use ffmpeg_next::format;
use ffmpeg_next::log;
use gifski::collector::ImgVec;
use gifski::collector::RGBA8;
use gifski::Repeat;
use gifski::Settings;
use image::DynamicImage;
use image::GenericImageView;
use image::ImageFormat;
use image::ImageReader;
use minimp4::Mp4Muxer;
use openh264::encoder::Encoder;
use openh264::formats::RgbSliceU8;
use openh264::formats::YUVBuffer;
use std::io::Cursor;
use std::io::Read;
use std::io::Seek;
use std::io::SeekFrom;
use std::path::Path;
use std::path::PathBuf;
use tokio::runtime::Handle;
use tokio::task::JoinHandle;

extern crate ffmpeg_next as ffmpeg;

#[allow(dead_code)]
/// Crop black space and convert video Webm to Gif
pub fn webm_to_gif(path: &PathBuf, state: fn(usize)) -> Result<PathBuf, Box<dyn std::error::Error>> {
    state(0);
    let mut images = webm_to_images(path)?;
    state(1);
    let images = images_crop_space(&mut images)?;
    state(2);
    // Params
    let image = match images.last() {
        Some(value) => value,
        None => Err("Empty frames")?,
    };
    let width = image.width();
    let height = image.height();
    // Create Gif
    let path =
        tokio::task::block_in_place(|| Handle::current().block_on(create_gif(images, path, width, height, state)))?;
    Ok(path)
}

/// Crop black space and convert video Webm to Mp4
pub fn webm_to_mp4(path: &PathBuf, state: fn(usize)) -> Result<PathBuf, Box<dyn std::error::Error>> {
    state(0);
    let mut images = webm_to_images(path)?;
    state(1);
    let images = images_crop_space(&mut images)?;
    state(2);
    // Params
    let image = match images.last() {
        Some(value) => value,
        None => Err("Empty frames")?,
    };
    let width = image.width();
    let height = image.height();
    // Create mp4
    let path = create_mp4(images, path, width, height, state)?;
    Ok(path)
}

/// Get images from webm
/// C bindings: ffmpeg
fn webm_to_images(path: &PathBuf) -> Result<Vec<DynamicImage>, Box<dyn std::error::Error>> {
    /// Convert fame to image
    fn get_image(frame: &Video) -> std::result::Result<DynamicImage, Box<dyn std::error::Error>> {
        // Create raw
        let header = format!("P6\n{} {}\n255\n", frame.width(), frame.height());
        let raw: Vec<u8> = [header.as_bytes(), frame.data(0)].concat();
        // Load image
        let image = ImageReader::with_format(Cursor::new(raw), ImageFormat::Pnm)
            .with_guessed_format()?
            .decode()?;
        // Result
        Ok(image)
    }
    // Start load data
    ffmpeg::init().unwrap();
    log::set_level(log::Level::Warning);
    // Params
    let mut result: Vec<DynamicImage> = vec![];
    let mut ictx = format::input(&path).unwrap();
    let input = ictx.streams().best(Type::Video).ok_or(ffmpeg::Error::StreamNotFound)?;
    let video_stream_index = input.index();
    let context_decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())?;
    let mut decoder = context_decoder.decoder().video()?;
    let mut scaler = Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::RGB24,
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    )?;
    // Get images
    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), Box<dyn std::error::Error>> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                // Get frame
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                // Get image
                result.push(get_image(&rgb_frame)?);
            }
            Ok(())
        };
    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder.send_packet(&packet)?;
            receive_and_process_decoded_frames(&mut decoder)?;
        }
    }
    decoder.send_eof()?;
    receive_and_process_decoded_frames(&mut decoder)?;
    // Result
    Ok(result)
}

fn images_crop_space(images: &mut Vec<DynamicImage>) -> Result<Vec<DynamicImage>, Box<dyn std::error::Error>> {
    /// Get black border width
    fn get_black_space_width(image: &DynamicImage) -> u32 {
        let black = [0_u8, 0_u8, 0_u8, 255];
        for x in 0..(image.width() / 2) {
            if image.get_pixel(x, 0).0 != black {
                return x;
            }
        }
        return 0;
    }
    // Params
    let mut result: Vec<DynamicImage> = vec![];
    let mut width = 0;
    let mut height = 0;
    let mut space = 0;
    // Crop
    for image in images.iter_mut() {
        // Set space and size
        if space == 0 {
            space = get_black_space_width(&image);
            width = image.width() - (space * 2);
            height = image.height();
        }
        // Crop space
        result.push(image.crop(space, 0, width, height));
    }
    Ok(result)
}

/// Create gif from image
/// gifski
async fn create_gif(
    images: Vec<DynamicImage>,
    path: &PathBuf,
    width: u32,
    height: u32,
    state: fn(usize),
) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
    let gif_path = path.to_string_lossy().replace("webm", "gif");
    let (collector, writer) = gifski::new(Settings {
        width: None,
        height: None,
        quality: 60,
        fast: false,
        repeat: Repeat::Infinite,
    })?;
    // Move to tokio
    let mut images = images.clone();
    // Run create
    let join_handler: JoinHandle<Result<(), gifski::Error>> = tokio::task::spawn_blocking(move || {
        let size = images.iter().count();
        let mut percent = 0;
        for (index, image) in images.iter_mut().enumerate() {
            // Send state
            let pos = index * 100 / size;
            if percent != pos && pos > 2 && pos != 100 {
                state(index * 100 / size);
                percent = pos;
            }
            // Get pixels
            let mut image_pixels = Vec::new();
            for pixel in image.to_rgba8().pixels() {
                image_pixels.push(RGBA8::from(pixel.0));
            }
            // Add to collector
            collector.add_frame_rgba(
                index,
                ImgVec::new(image_pixels, width as usize, height as usize),
                index as f64 * 0.1,
            )?;
        }
        Ok(())
    });
    // Write file
    writer.write(std::fs::File::create(&gif_path)?, &mut gifski::progress::NoProgress {})?;
    join_handler.await??;
    // Send done
    state(100);
    // Result
    Ok(Path::new(&gif_path).to_path_buf())
}

/// Create mp4 from image
/// Rust way
fn create_mp4(
    images: Vec<DynamicImage>,
    path: &PathBuf,
    width: u32,
    height: u32,
    state: fn(usize),
) -> std::result::Result<PathBuf, Box<dyn std::error::Error>> {
    let mp4_path = path.to_string_lossy().replace("webm", "mp4");
    let mut encoder = Encoder::new().unwrap();
    let mut buf = Vec::new();
    // Run create
    let size = images.iter().count();
    let mut percent = 0;
    for (index, image) in images.iter().enumerate() {
        // Send state
        let pos = index * 100 / size;
        if percent != pos && pos > 2 && pos != 100 {
            state(index * 100 / size);
            percent = pos;
        }
        let frame = image.as_bytes().to_vec();
        // Convert RGB into YUV.
        let rgb_source = RgbSliceU8::new(&frame, (width as usize, height as usize));
        let yuv = YUVBuffer::from_rgb8_source(rgb_source);
        // Encode YUV into H.264.
        let bit_stream = encoder.encode(&yuv).unwrap();
        bit_stream.write_vec(&mut buf);
    }
    let mut video_buffer = Cursor::new(Vec::new());
    let mut mp4muxer = Mp4Muxer::new(&mut video_buffer);
    mp4muxer.init_video(width as i32, height as i32, false, "Record Emulator");
    mp4muxer.write_video(&buf);
    mp4muxer.close();
    // Some shenanigans to get the raw bytes for the video.
    video_buffer.seek(SeekFrom::Start(0)).unwrap();
    let mut video_bytes = Vec::new();
    video_buffer.read_to_end(&mut video_bytes).unwrap();
    // Write file
    std::fs::write(&mp4_path, &video_bytes).unwrap();
    // Send done
    state(100);
    // Result
    Ok(Path::new(&mp4_path).to_path_buf())
}
