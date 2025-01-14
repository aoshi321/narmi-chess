/*
1.2	The player with the light-coloured pieces (White) makes the first move,
then the players move alternately, with the player with the dark-coloured pieces (Black) making the next move.
*/

use crate::game::game::*;
use crate::game::moving::*;
use super::rule::Rule;

pub struct Implementation {}

impl Implementation {
    pub fn new() -> Implementation { Implementation {} }
}

impl Rule for Implementation {
    fn validate(&self, _: &Game, _: String) -> Result<(), MoveError> {
        Ok(())
    }
}
