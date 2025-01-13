use crate::protocol::PlaySound;
use crate::sound_player::SoundPlayer;
use std::thread::{self, sleep};
use std::time::{Duration, SystemTime, UNIX_EPOCH};

pub struct SoundScheduler;

impl SoundScheduler {
    pub fn handle_scheduled_sound(play_msg: PlaySound) {
        thread::spawn(move || {
            let PlaySound {
                channels,
                sample_rate,
                sound_data,
                timestamp,
            } = play_msg;
            println!(
                "Scheduled sound: len={:?} at {:?}",
                sound_data.len(),
                timestamp
            );
            let now_micros = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .expect("Could not get time")
                .as_micros();
            let wait = timestamp as i128 - now_micros as i128;
            if wait <= 0 {
                println!("Too late to play sound");
                return;
            }
            sleep(Duration::from_micros(wait as u64));
            println!("Playing sound...");
            SoundPlayer::play_sound(channels, sample_rate, sound_data);
        });
    }
}
