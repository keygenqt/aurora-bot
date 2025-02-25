use std::path::{Path, PathBuf};

extern crate ffmpeg_next as ffmpeg;

use ffmpeg::format::Pixel;
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::frame::video::Video;
use ffmpeg_next::{format, log};
use gifski::collector::{ImgVec, RGBA8};
use gifski::{Repeat, Settings};
use image::{DynamicImage, GenericImageView, ImageFormat, ImageReader};
use tokio::runtime::Handle;
use tokio::task::JoinHandle;
use std::io::Cursor;

// Convert webm to gif
pub fn ffmpeg_webm_to_gif(path: &PathBuf, state: fn (usize)) -> Result<PathBuf, Box<dyn std::error::Error>> {

    ffmpeg::init().unwrap();
    log::set_level(log::Level::Warning);

    let gif_path = path.to_string_lossy().replace("webm", "gif");
    let mut ictx = format::input(&path).unwrap();

    let input = ictx
        .streams()
        .best(Type::Video)
        .ok_or(ffmpeg::Error::StreamNotFound)?;

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

    let mut frame_index = 0;
    let mut images_raw: Vec<DynamicImage> = vec![];

    state(0);

    // Get images
    let mut receive_and_process_decoded_frames =
        |decoder: &mut ffmpeg::decoder::Video| -> Result<(), Box<dyn std::error::Error>> {
            let mut decoded = Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                // Get frame
                let mut rgb_frame = Video::empty();
                scaler.run(&decoded, &mut rgb_frame)?;
                // Get image
                let image: DynamicImage = get_image(&rgb_frame)?;
                // Save image
                images_raw.push(image);
                // Up index
                frame_index += 1;
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

    // Create Gif
    tokio::task::block_in_place(|| {
        Handle::current().block_on(create_gif(&mut images_raw, &gif_path, frame_index, state))
    })?;

    state(100);

    Ok(Path::new(&gif_path).to_path_buf())
}

// Convert fame to image
fn get_image(frame: &Video) -> std::result::Result<DynamicImage, Box<dyn std::error::Error>> {
    // Create raw
    let header = format!("P6\n{} {}\n255\n", frame.width(), frame.height());
    let raw: Vec<u8> = [header.as_bytes(), frame.data(0)].concat();
    // Load image
    let image = ImageReader::with_format(
        Cursor::new(raw),
        ImageFormat::Pnm,
    ).with_guessed_format()?.decode()?;
    // Result
    Ok(image)
}

// Get black border width
fn get_black_space_width(image: &DynamicImage) -> u32 {
    let black = [0_u8, 0_u8, 0_u8, 255];
    for x in 0..(image.width() / 2) {
        if image.get_pixel(x, 0).0 != black {
            return x;
        }
    }
    return 0;
}

// Create gif
async fn create_gif(
    images_raw: &mut Vec<DynamicImage>,
    gif_path: &String,
    size: usize,
    state: fn (usize)
) -> std::result::Result<(), Box<dyn std::error::Error>> {
    // Get last frame
    let last = match images_raw.last() {
        Some(value) => value,
        None => Err("Images empty")?,
    };
    // Get black space width
    let space = get_black_space_width(&last);
    let width = last.width() - (space * 2);
    let height = last.height();
    // Create gif
    let (collector, writer) = gifski::new(Settings {
        // @todo
        // telegram sucks - crop gif
        width: None,
        height: None,
        quality: 80,
        fast: true,
        repeat: Repeat::Infinite,
    })?;
    // Move to tokio
    let mut images_raw = images_raw.clone();
    // Run create
    let join_handler:JoinHandle<Result<(), gifski::Error>> = tokio::task::spawn_blocking(move || {
        let mut percent = 0;
        for (index, image) in images_raw.iter_mut().enumerate()  {
            // Crop space and to rgba8
            let frame_image = image.crop(space, 0, width, height).to_rgba8();
            // Get pixels
            let mut image_pixels = Vec::new();
            for pixel in frame_image.pixels() {
                image_pixels.push(RGBA8::from(pixel.0));
            }
            // Send state
            let pos = index * 100 / size;
            if percent != pos {
                state(index * 100 / size);
                percent = pos;
            }
            // Add to collector
            collector.add_frame_rgba(
                index,
                ImgVec::new(image_pixels, width as usize, height as usize),
                    index as f64 * 0.1)?;
        }
        Ok(())
    });
    writer.write(std::fs::File::create(&gif_path)?, &mut gifski::progress::NoProgress {})?;
    join_handler.await??;
    Ok(())
}
