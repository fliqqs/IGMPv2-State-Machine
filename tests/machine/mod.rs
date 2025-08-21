use igmp_group_state_machine::Fsm;
use igmp_group_state_machine::actions::ActionContext;
use igmp_group_state_machine::enums::{IgmpV2MulticastGroupEvents, IgmpV2MulticastGroupStates};
use igmp_group_state_machine::services::{RoutingService, TimerService};
use igmp_group_state_machine::transitions::{TransitionTable, build_transition_table};
use std::cell::RefCell;
use std::net::Ipv4Addr;

pub struct MockRoutingService {
    pub calls: RefCell<Vec<String>>,
}
impl MockRoutingService {
    pub fn new() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
        }
    }
}
impl RoutingService for MockRoutingService {
    fn notify_plus(&self) {
        self.calls.borrow_mut().push("NotifyRoutingPlus".into());
    }
    fn notify_minus(&self) {
        self.calls.borrow_mut().push("NotifyRoutingMinus".into());
    }

    fn send_group_specific_query(&self, group: Ipv4Addr) {
        self.calls
            .borrow_mut()
            .push(format!("SendGroupSpecificQuery({})", group));
    }
}

pub struct MockTimerService {
    pub calls: RefCell<Vec<String>>,
}
impl MockTimerService {
    pub fn new() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
        }
    }
}
impl TimerService for MockTimerService {
    fn start_timer(&self) {
        self.calls.borrow_mut().push("StartTimer".into());
    }

    fn start_timer_star(&self) {
        self.calls.borrow_mut().push("StartTimerStar".into());
    }

    fn start_retransmit_timer(&self) {
        self.calls.borrow_mut().push("StartRetransmitTimer".into());
    }
}

#[test]
fn no_members_to_members() {
    let table: TransitionTable = build_transition_table();
    let mut fsm = Fsm::new(table);

    let r = MockRoutingService::new();
    let t = MockTimerService::new();

    let ctx = ActionContext {
        timer_service: &t,
        routing_service: &r,
        group_address: Ipv4Addr::new(239, 2, 3, 10),
    };

    // assert state
    fsm.on_event(IgmpV2MulticastGroupEvents::V2ReportReceived, &ctx);
    assert_eq!(fsm.state(), &IgmpV2MulticastGroupStates::MembersPresent);

    // assert actions
    let r_calls = r.calls.borrow();
    assert_eq!(r_calls.as_slice(), &["NotifyRoutingPlus"]);
}

#[test]
fn members_present_to_checking() {
    let table: TransitionTable = build_transition_table();
    let mut fsm = Fsm::new(table);

    let r = MockRoutingService::new();
    let t = MockTimerService::new();

    let ctx = ActionContext {
        timer_service: &t,
        routing_service: &r,
        group_address: Ipv4Addr::new(239, 2, 3, 10),
    };

    fsm.set_current_state(IgmpV2MulticastGroupStates::MembersPresent);

    fsm.on_event(IgmpV2MulticastGroupEvents::LeaveReceived, &ctx);
    assert_eq!(fsm.state(), &IgmpV2MulticastGroupStates::CheckingMembership);

    let r_calls = r.calls.borrow();
    let t_calls = t.calls.borrow();

    let mut expected_calls = vec![
        "StartRetransmitTimer",
        "StartTimerStar",
        "SendGroupSpecificQuery(239.2.3.10)",
    ];

    let mut all_calls = r_calls.iter().chain(t_calls.iter()).collect::<Vec<_>>();
    expected_calls.sort();
    all_calls.sort();

    assert_eq!(all_calls, expected_calls);
}
