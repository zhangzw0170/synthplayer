use rodio::{Decoder, OutputStream, Sink, Source};
use crate::core::analyzer::{AnalyzerSource, SampleBuffer};
use std::fs::File;
use std::io::BufReader;
use std::path::PathBuf;
use std::time::{Duration, Instant};

pub struct AudioPlayer {
    _stream: Option<OutputStream>,
    sink: Option<Sink>,
    current_path: Option<PathBuf>,
    current_duration: Duration,
    paused: bool,
    volume: f32,
    // Manual position tracking (rodio 0.20 Sink lacks get_pos)
    analyzer_buf: Option<SampleBuffer>,
    play_start: Option<Instant>,
    elapsed_before_pause: Duration,
}

impl AudioPlayer {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            _stream: None,
            sink: None,
            current_path: None,
            current_duration: Duration::ZERO,
            paused: false,
            volume: 0.8,
            analyzer_buf: None,
            play_start: None,
            elapsed_before_pause: Duration::ZERO,
        })
    }

    pub fn play(&mut self, path: PathBuf, known_dur: Option<Duration>, analyzer_buf: Option<SampleBuffer>) -> Result<(), Box<dyn std::error::Error>> {
        self.stop();
        self.analyzer_buf = analyzer_buf;

        let (stream, stream_handle) = OutputStream::try_default()?;
        let sink = Sink::try_new(&stream_handle)?;

        let file = File::open(&path)?;
        let decoder = Decoder::new(BufReader::new(file))?;
        let duration = decoder.total_duration().or(known_dur);

        // Convert to f32, optionally feed analyzer buffer, then track position
        let source = decoder.convert_samples::<f32>();
        let source = if let Some(ref buf) = self.analyzer_buf {
            Box::new(AnalyzerSource::new(source, buf.clone()))
                as Box<dyn Source<Item = f32> + Send>
        } else {
            Box::new(source) as Box<dyn Source<Item = f32> + Send>
        };
        sink.append(source.track_position());
        sink.set_volume(self.volume);

        self._stream = Some(stream);
        self.sink = Some(sink);
        self.current_path = Some(path);
        self.current_duration = duration.unwrap_or(Duration::ZERO);
        self.paused = false;
        self.play_start = Some(Instant::now());
        self.elapsed_before_pause = Duration::ZERO;
        Ok(())
    }

    pub fn pause(&mut self) {
        if let Some(sink) = &self.sink {
            sink.pause();
            self.paused = true;
            self.elapsed_before_pause = self.position();
            self.play_start = None;
        }
    }

    pub fn resume(&mut self) {
        if let Some(sink) = &self.sink {
            sink.play();
            self.paused = false;
            self.play_start = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(sink) = self.sink.take() {
            sink.stop();
        }
        self._stream = None;
        self.current_path = None;
        self.current_duration = Duration::ZERO;
        self.paused = false;
        self.play_start = None;
        self.elapsed_before_pause = Duration::ZERO;
    }

    pub fn set_volume(&mut self, volume: f32) {
        self.volume = volume.clamp(0.0, 1.0);
        if let Some(sink) = &self.sink {
            sink.set_volume(self.volume);
        }
    }

    pub fn seek(&mut self, pos: Duration) {
        if let Some(sink) = &self.sink
            && sink.try_seek(pos).is_ok()
        {
            self.elapsed_before_pause = pos;
            self.play_start = Some(Instant::now());
        }
    }

    pub fn seek_forward(&mut self, delta: Duration) {
        let total = self.current_duration;
        let target = self.position().saturating_add(delta).min(total);
        self.seek(target);
    }

    pub fn seek_backward(&mut self, delta: Duration) {
        let target = self.position().saturating_sub(delta);
        self.seek(target);
    }

    pub fn is_paused(&self) -> bool {
        self.paused
    }

    pub fn current_path(&self) -> Option<&PathBuf> {
        self.current_path.as_ref()
    }

    pub fn current_duration(&self) -> Duration {
        self.current_duration
    }

    pub fn volume(&self) -> f32 {
        self.volume
    }

    pub fn analyzer_buffer(&self) -> Option<&SampleBuffer> {
        self.analyzer_buf.as_ref()
    }

    pub fn position(&self) -> Duration {
        let elapsed = self.elapsed_before_pause;
        if let Some(start) = self.play_start {
            elapsed + start.elapsed()
        } else {
            elapsed
        }
    }

    pub fn finished(&self) -> bool {
        self.sink.as_ref().is_none_or(|s| s.empty())
    }
}