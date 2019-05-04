use crate::block::{Block, BlockError, BlockState, ClickEvent};

pub struct MediaPlayer {}

impl Block for MediaPlayer {
    fn current_state(&self) -> Result<BlockState, BlockError> {
        let player = mpris::PlayerFinder::new()
            .or_else(|_| Err(BlockError::new("Failed to create player finder".into())))
            .and_then(|finder| {
                finder
                    .find_all()
                    .map_err(|_| BlockError::new("Failed to find all players".into()))
            })
            .and_then(|players| {
                let player = players
                    .first()
                    .ok_or(BlockError::new("No players are running".into()))?;
                let metadata = player
                    .get_metadata()
                    .or_else(|_| Err(BlockError::new("".into())))?;

                let artist = metadata
                    .artists()
                    .ok_or(BlockError::new("Failed to get artists".into()))
                    .and_then(|artists| {
                        artists
                            .first()
                            .ok_or(BlockError::new("Found zero artists".into()))
                    })
                    .or_else(|_| Err(BlockError::new("".into())))?;

                let title = metadata
                    .title()
                    .ok_or(BlockError::new("Failed to get title".into()))?;

                let formatted = format!("{} - {}", artist, title);

                Ok(BlockState::new(formatted))
            });

        return player;
    }

    fn handle_click(&self, _event: ClickEvent) {}
}

impl MediaPlayer {
    pub fn new() -> Self {
        MediaPlayer {}
    }
}
