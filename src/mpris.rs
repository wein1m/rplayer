use std::sync::mpsc::{channel, Receiver};
use std::thread;

use async_channel::{unbounded, Receiver as AsyncReceiver, Sender as AsyncSender};
use mpris_server::zbus::block_on;
use mpris_server::{Metadata, PlaybackStatus, Player, Time};

use crate::scanner::scanner::TrackAudio;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MprisCommand {
    Next,
    Previous,
    PlayPause,
    Play,
    Pause,
}

enum MprisMsg {
    UpdateSong {
        title: String,
        artist: String,
        duration: u64,
    },
    PlaybackStatus(PlaybackStatus),
}

pub struct Mpris {
    rx: Receiver<MprisCommand>,
    tx: AsyncSender<MprisMsg>,
}

impl Mpris {
    pub fn new() -> Result<Self, Box<dyn std::error::Error>> {
        let (cmd_tx, rx) = channel::<MprisCommand>();
        let (tx, msg_rx): (AsyncSender<MprisMsg>, AsyncReceiver<MprisMsg>) = unbounded();

        thread::spawn(move || {
            block_on(async move {
                let player = match Player::builder("rplayer")
                    .identity("rplayer")
                    .can_play(true)
                    .can_pause(true)
                    .can_go_next(true)
                    .can_go_previous(true)
                    .build()
                    .await
                {
                    Ok(p) => p,
                    Err(e) => {
                        eprintln!("Failed to initialize MPRIS player: {e}");
                        return;
                    }
                };

                let cmd_tx_next = cmd_tx.clone();
                player.connect_next(move |_| {
                    let _ = cmd_tx_next.send(MprisCommand::Next);
                });

                let cmd_tx_prev = cmd_tx.clone();
                player.connect_previous(move |_| {
                    let _ = cmd_tx_prev.send(MprisCommand::Previous);
                });

                let cmd_tx_pp = cmd_tx.clone();
                player.connect_play_pause(move |_| {
                    let _ = cmd_tx_pp.send(MprisCommand::PlayPause);
                });

                let cmd_tx_play = cmd_tx.clone();
                player.connect_play(move |_| {
                    let _ = cmd_tx_play.send(MprisCommand::Play);
                });

                let cmd_tx_pause = cmd_tx.clone();
                player.connect_pause(move |_| {
                    let _ = cmd_tx_pause.send(MprisCommand::Pause);
                });

                let run_task = player.run();
                let msg_loop = async {
                    while let Ok(msg) = msg_rx.recv().await {
                        match msg {
                            MprisMsg::UpdateSong {
                                title,
                                artist,
                                duration,
                            } => {
                                let builder = Metadata::builder()
                                    .title(&title)
                                    .artist([&artist])
                                    .length(Time::from_secs(duration as i64));

                                let metadata = builder.build();
                                let _ = player.set_metadata(metadata).await;
                            }
                            MprisMsg::PlaybackStatus(status) => {
                                let _ = player.set_playback_status(status).await;
                            }
                        }
                    }
                };

                futures_util::pin_mut!(run_task);
                futures_util::pin_mut!(msg_loop);
                futures_util::future::select(run_task, msg_loop).await;
            });
        });

        Ok(Self {rx, tx})
    }

    pub fn update_song(&self, song: &TrackAudio) {
        let _ = self.tx.send_blocking(MprisMsg::UpdateSong {
            title: song.track_title.clone(),
            artist: song.track_artist.clone(),
            duration: song.track_duration,
        });
    }

    pub fn set_playback_status(&self, is_paused: bool) {
        let status: PlaybackStatus;

        if is_paused {
            status = PlaybackStatus::Paused
        } else {
            status = PlaybackStatus::Playing
        }

        self.tx.send_blocking(MprisMsg::PlaybackStatus(status));
   }

    pub fn poll_command(&self) -> Option<MprisCommand> {
        self.rx.try_recv().ok()
    }
}
