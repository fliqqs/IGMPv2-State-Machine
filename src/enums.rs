#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum IgmpV2MulticastGroupStates {
    NoMembersPresent,
    MembersPresent,
    Version1MembersPresent,
    CheckingMembership,
}

// There are six significant events that can cause router state transitions:
#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum IgmpV2MulticastGroupEvents {
    V2ReportReceived,
    V1ReportReceived,
    LeaveReceived,
    TimerExpired,
    RetransmitTimerExpired,
    V1HostTimerExpired,
}

#[derive(Debug, Clone, Copy)]
pub enum IgmpV2MulticastGroupStateActions {
    StartTimer,
    StartTimerStar,
    StartRetransmitTimer,
    StartV1HostTimer,
    SendGroupSpecificQuery,
    NotifyRoutingPlus,
    NotifyRoutingMinus,
}
