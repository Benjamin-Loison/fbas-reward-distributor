use crate::{Reward, Score};
use fbas_analyzer::NodeId;

pub type NodeRanking = (NodeId, PublicKey, Score);
pub type NodeReward = (NodeId, PublicKey, Score, Reward);
pub type PublicKey = String;

pub mod report;

pub use report::*;
