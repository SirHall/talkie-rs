use rodio::{buffer::SamplesBuffer, source::Source, Decoder, OutputStream};
use talkie;

fn main()
{
    let sin_mult = 0.1f32;
    let audio = (0..50_000)
        .map(|i| ((i as f32 * sin_mult).sin() * 0.5 + 0.5))
        .collect::<Vec<_>>();

    let (_stream, stream_handle) = OutputStream::try_default().expect("Could not find a default sound device");

    let buf = SamplesBuffer::new(1, 8_000, audio);

    stream_handle.play_raw(buf).expect("Failed to play audio");

    std::thread::sleep(std::time::Duration::from_secs(5));
}
