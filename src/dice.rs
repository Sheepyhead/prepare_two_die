use bevy::prelude::*;
use regex::{Error, Regex};
use std::fmt;

/// The types of dice supported by this crate
#[derive(Clone, Copy, Component)]
pub enum DieType {
    /// Standard cubic six-sided die
    D6,
}

impl TryFrom<u32> for DieType {
    type Error = String;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            6 => Ok(Self::D6),
            _ => Err(format!("No such die type: d{value}")),
        }
    }
}

impl fmt::Display for DieType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            DieType::D6 => "d6",
        })
    }
}

impl DieType {
    pub(crate) fn get_resting_value(&self, (x, _, z): (f32, f32, f32)) -> u32 {
        match self {
            DieType::D6 => {
                // Represents how many different sides you can turn this die across one axis
                enum SideTurns {
                    None,
                    One,
                    Two,
                    Three,
                }
                impl fmt::Display for SideTurns {
                    fn fmt(
                        &self,
                        f: &mut std::fmt::Formatter<'_>,
                    ) -> std::result::Result<(), std::fmt::Error> {
                        f.write_str(match self {
                            SideTurns::None => "None",
                            SideTurns::One => "One",
                            SideTurns::Two => "Two",
                            SideTurns::Three => "Three",
                        })
                    }
                }
                let x_turns = match () {
                    () if x < ((-3.1 + -1.6) / 2.0) => SideTurns::Two,
                    () if x < ((-1.6 + 0.0) / 2.0) => SideTurns::Three,
                    () if x < ((0.0 + 1.6) / 2.0) => SideTurns::None,
                    () if x < ((1.6 + 3.1) / 2.0) => SideTurns::One,
                    () => SideTurns::Two,
                };
                let z_turns = match () {
                    () if z < ((-3.1 + -1.6) / 2.0) => SideTurns::Two,
                    () if z < ((-1.6 + 0.0) / 2.0) => SideTurns::Three,
                    () if z < ((0.0 + 1.6) / 2.0) => SideTurns::None,
                    () if z < ((1.6 + 3.1) / 2.0) => SideTurns::One,
                    () => SideTurns::Two,
                };

                match (x_turns, z_turns) {
                    (SideTurns::None, SideTurns::One)
                    | (SideTurns::One, SideTurns::One)
                    | (SideTurns::Two, SideTurns::One)
                    | (SideTurns::Three, SideTurns::One) => 1,
                    (SideTurns::None, SideTurns::None) | (SideTurns::Two, SideTurns::Two) => 2,
                    (SideTurns::One, SideTurns::None) | (SideTurns::Three, SideTurns::Two) => 3,
                    (SideTurns::One, SideTurns::Two) | (SideTurns::Three, SideTurns::None) => 4,
                    (SideTurns::None, SideTurns::Two) | (SideTurns::Two, SideTurns::None) => 5,
                    (SideTurns::None, SideTurns::Three)
                    | (SideTurns::One, SideTurns::Three)
                    | (SideTurns::Two, SideTurns::Three)
                    | (SideTurns::Three, SideTurns::Three) => 6,
                }
            }
        }
    }
}

/// Command event for rolling one or more dice
#[allow(dead_code)]
pub enum RollDice {
    /// Rolls dice based on a string. A dice string is in the format xdy where x is a positive
    /// non-zero number representing the number of dice to roll, and y is the number of faces on
    /// the dice, see supported types in DieType. All other text than xdy is ignored, so text like
    /// "Arin rolls 3d6 for his strength check" still evaluates to a 3d6 dice roll
    DiceString(String),
    /// Rolls the dice in the given vector
    Dice(Vec<DieType>),
}

impl RollDice {
    pub(crate) fn to_dice(&self) -> Result<Vec<DieType>, String> {
        match self {
            RollDice::DiceString(dice) => {
                let reg = match Regex::new(r"(\d*)d(\d*)") {
                    Ok(reg) => reg,
                    Err(Error::CompiledTooBig(size)) => {
                        return Err(format!("Regex compiled too big, size limit {size}"))
                    }
                    Err(Error::Syntax(message)) => return Err(message),
                    Err(_) => return Err("Unknown regex error occurred".into()),
                };
                let caps = match reg.captures(dice) {
                    Some(caps) => caps,
                    None => return Err(format!("No dice expression detected in '{dice}")),
                };
                if caps.len() != 3 {
                    return Err(format!("No dice expression detected in {dice}"));
                }
                let amount = caps.get(1).unwrap().as_str().parse().unwrap();
                let face =
                    DieType::try_from(caps.get(2).unwrap().as_str().parse::<u32>().unwrap())?;
                Ok(vec![face; amount])
            }
            RollDice::Dice(dice) => Ok(dice.iter().copied().collect()),
        }
    }
}

/// The result of a dice roll
pub struct DiceResult {
    /// The indidivual dice results
    pub results: Vec<u32>,
}

impl DiceResult {
    /// Returns the sum total of all dice rolls in this result
    pub fn total(&self) -> u32 {
        self.results.iter().sum()
    }

    /// Returns the number of dice rolled for this result
    pub fn number_of_dice(&self) -> usize {
        self.results.len()
    }
}
