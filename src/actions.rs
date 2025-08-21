use crate::enums::{IgmpV2MulticastGroupEvents, IgmpV2MulticastGroupStateActions};
use crate::services::{RoutingService, TimerService};
use std::net::Ipv4Addr;

pub struct ActionContext<'a, T: TimerService, R: RoutingService> {
    pub timer_service: &'a T,
    pub routing_service: &'a R,
    pub group_address: Ipv4Addr,
}

// -- Actions --

pub fn start_timer<T: TimerService, R: RoutingService>(
    ctx: &ActionContext<T, R>,
    _event: &IgmpV2MulticastGroupEvents,
) {
    ctx.timer_service.start_timer();
}

pub fn start_retransmit_timer<T: TimerService, R: RoutingService>(
    ctx: &ActionContext<T, R>,
    _event: &IgmpV2MulticastGroupEvents,
) {
    ctx.timer_service.start_retransmit_timer();
}

pub fn start_timer_start<T: TimerService, R: RoutingService>(
    ctx: &ActionContext<T, R>,
    _event: &IgmpV2MulticastGroupEvents,
) {
    ctx.timer_service.start_timer_star();
}

pub fn notify_routing_plus<T: TimerService, R: RoutingService>(
    ctx: &ActionContext<T, R>,
    _event: &IgmpV2MulticastGroupEvents,
) {
    ctx.routing_service.notify_plus();
}

pub fn notify_routing_minus<T: TimerService, R: RoutingService>(
    ctx: &ActionContext<T, R>,
    _event: &IgmpV2MulticastGroupEvents,
) {
    ctx.routing_service.notify_minus();
}

pub fn send_group_specific_query<T: TimerService, R: RoutingService>(
    ctx: &ActionContext<T, R>,
    _event: &IgmpV2MulticastGroupEvents,
) {
    ctx.routing_service
        .send_group_specific_query(ctx.group_address);
}

pub fn action_map<T: TimerService, R: RoutingService>(
    action: IgmpV2MulticastGroupStateActions,
) -> fn(&ActionContext<T, R>, &IgmpV2MulticastGroupEvents) {
    match action {
        IgmpV2MulticastGroupStateActions::StartTimer => start_timer,
        IgmpV2MulticastGroupStateActions::NotifyRoutingPlus => notify_routing_plus,
        IgmpV2MulticastGroupStateActions::NotifyRoutingMinus => notify_routing_minus,
        IgmpV2MulticastGroupStateActions::SendGroupSpecificQuery => send_group_specific_query,
        IgmpV2MulticastGroupStateActions::StartTimerStar => start_timer_start,
        IgmpV2MulticastGroupStateActions::StartRetransmitTimer => start_retransmit_timer,
        _ => unimplemented!("Action not implemented"),
    }
}
