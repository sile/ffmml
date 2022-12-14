use crate::{
    channel::{Channel, ChannelName},
    macros::Macros,
    types::Credits,
};
use std::collections::BTreeMap;

#[derive(Debug, Default, Clone)]
pub struct Music {
    pub credits: Credits,
    pub macros: Macros,
    pub channels: BTreeMap<ChannelName, Channel>,
}

impl Music {}
