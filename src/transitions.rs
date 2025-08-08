use crate::enums::{
    IgmpV2MulticastGroupEvents, IgmpV2MulticastGroupStateActions, IgmpV2MulticastGroupStates,
};
use crate::enums::{
    IgmpV2MulticastGroupEvents::*, IgmpV2MulticastGroupStateActions::*,
    IgmpV2MulticastGroupStates::*,
};
use std::collections::HashMap;

pub type TransitionTable = HashMap<
    (IgmpV2MulticastGroupStates, IgmpV2MulticastGroupEvents),
    (
        IgmpV2MulticastGroupStates,
        Vec<IgmpV2MulticastGroupStateActions>,
    ),
>;

pub fn build_transition_table() -> TransitionTable {
    let mut table = HashMap::new();

    table.insert(
        (NoMembersPresent, V2ReportReceived),
        (MembersPresent, vec![StartTimer, NotifyRoutingPlus]),
    );

    table.insert(
        (MembersPresent, TimerExpired),
        (NoMembersPresent, vec![NotifyRoutingMinus]),
    );

    table
}
