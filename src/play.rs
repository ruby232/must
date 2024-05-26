use std::fs::File;
use std::path::Path;
use symphonia::core::codecs::{CODEC_TYPE_NULL, DecoderOptions};
use symphonia::core::errors::Error;
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;
use log::warn;

use crate::output;

pub fn play() {
    let path = Path::new("/home/ban/Projects/must/examples/perfect-beauty.mp3");
    let src = File::open(path).expect("Failed to open file");
    let source = Box::new(src);
    let mss = MediaSourceStream::new(source, Default::default());
    let mut hint = Hint::new();
    hint.with_extension("mp3");

    let meta_opts: MetadataOptions = Default::default();
    let fmt_opts: FormatOptions = Default::default();

    let probe = symphonia::default::get_probe()
        .format(&hint, mss, &fmt_opts, &meta_opts)
        .expect("Unsuported format");

    let mut format = probe.format;

    let track = format
        .tracks()
        .iter()
        .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
        .expect("No audio track found");

    let dec_opts: DecoderOptions = Default::default();

    let mut decoder = symphonia::default::get_codecs()
        .make(&track.codec_params, &dec_opts)
        .expect("Unsupported codec");

    let track_id = track.id;

    // Set audio output
    let audio_output: &mut Option<Box<dyn output::AudioOutput>> = &mut None;


    loop {
        let packet = match format.next_packet() {
            Ok(packet) => packet,
            Err(Error::ResetRequired) => {
                unimplemented!();
            }
            Err(err) => {
                panic!("Error: {}", err);
            }
        };

        if packet.track_id() != track_id {
            continue;
        }

        match decoder.decode(&packet) {
            Ok(decoded) => {
                if audio_output.is_none() {
                    let spec = *decoded.spec();
                    let duration = decoded.capacity() as u64;
                    audio_output.replace(output::try_open(spec, duration).unwrap());
                } else {
                    // TODO: Check the audio spec. and duration hasn't changed.
                }

                // @todo, analizar esta parte proque packet.ts() dice que siempre es mayor que 0
                // revisar la logica de symphonia-play
                if packet.ts() >= 0 {
                    if let Some(audio_output) = audio_output {
                        audio_output.write(decoded).unwrap()
                    }
                }
            }
            Err(Error::DecodeError(err)) => {
                warn!("decode error: {}", err);
            }
            Err(_err) => break,
        }
    }

    //@todo, ver si esto es necesario
    if let Some(audio_output) = audio_output.as_mut() {
        audio_output.flush();
    }
}