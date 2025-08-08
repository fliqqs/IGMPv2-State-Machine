use igmp_group_state_machine::Fsm;
use igmp_group_state_machine::actions::ActionContext;
use igmp_group_state_machine::enums::{IgmpV2MulticastGroupEvents, IgmpV2MulticastGroupStates};
use igmp_group_state_machine::services::{RoutingService, TimerService};
use igmp_group_state_machine::transitions::{TransitionTable, build_transition_table};

pub struct MockRoutingService;
impl RoutingService for MockRoutingService {
    fn notify_plus(&self) {}
    fn notify_minus(&self) {}
}

pub struct MockTimerService;
impl TimerService for MockTimerService {
    fn start(&self, _name: &str) {}
}

#[test]
fn no_members_to_members() {
    let table: TransitionTable = build_transition_table();
    let mut fsm = Fsm::new(table);
    let ctx = ActionContext {
        timer_service: &MockTimerService,
        routing_service: &MockRoutingService,
    };

    fsm.on_event(IgmpV2MulticastGroupEvents::V2ReportReceived, &ctx);
    assert_eq!(fsm.state(), &IgmpV2MulticastGroupStates::MembersPresent);
}
