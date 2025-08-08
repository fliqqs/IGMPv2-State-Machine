use crate::enums::{IgmpV2MulticastGroupEvents, IgmpV2MulticastGroupStateActions};
use crate::services::{RoutingService, TimerService};

pub struct ActionContext<'a, T: TimerService, R: RoutingService> {
    pub timer_service: &'a T,
    pub routing_service: &'a R,
}

// -- Actions --

pub fn start_timer<T: TimerService, R: RoutingService>(
    ctx: &ActionContext<T, R>,
    _event: &IgmpV2MulticastGroupEvents,
) {
    ctx.timer_service.start("main");
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

pub fn action_map<T: TimerService, R: RoutingService>(
    action: IgmpV2MulticastGroupStateActions,
) -> fn(&ActionContext<T, R>, &IgmpV2MulticastGroupEvents) {
    match action {
        IgmpV2MulticastGroupStateActions::StartTimer => start_timer,
        IgmpV2MulticastGroupStateActions::NotifyRoutingPlus => notify_routing_plus,
        IgmpV2MulticastGroupStateActions::NotifyRoutingMinus => notify_routing_minus,
        _ => unimplemented!("Action not implemented"),
    }
}
