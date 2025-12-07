use rodio::Source;
use rodio::source::SawtoothWave;
use rodio::{OutputStreamBuilder, Sink};
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub struct Chip8Sound {
    sink: Arc<Mutex<Sink>>,
    _stream_handle: Arc<rodio::OutputStream>,
    is_playing: Arc<Mutex<bool>>,
}

impl Chip8Sound {
    pub fn new() -> Self {
        let _stream_handle = OutputStreamBuilder::open_default_stream()
            .expect("Sound error");

        let sink = Sink::connect_new(&_stream_handle.mixer());

        let _stream_handle = Arc::new(_stream_handle);
        let sink = Arc::new(Mutex::new(sink));
        let is_playing = Arc::new(Mutex::new(false));

        Self {
            sink,
            _stream_handle,
            is_playing,
        }
    }

    pub fn update(&mut self, sound_timer: u8) {
        let should_play = sound_timer > 0;
        let mut playing = self.is_playing.lock().unwrap();

        if should_play && !*playing {
            self.start_beep();
            *playing = true;
        } else if !should_play && *playing {
            self.stop_beep();
            *playing = false;
        }
    }

    pub fn start_beep(&self) {
        let sink = self.sink.lock().unwrap();

        let source = SawtoothWave::new(520.0)
            .take_duration(Duration::from_secs_f32(0.1))
            .amplify(0.20);

        sink.append(source);
        sink.play();
    }

    fn stop_beep(&self) {
        let sink = self.sink.lock().unwrap();
        sink.stop();
        sink.clear();
    }
}

impl Drop for Chip8Sound {
    fn drop(&mut self) {
        self.stop_beep();
    }
}