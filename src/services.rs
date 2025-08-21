use std::net::Ipv4Addr;

pub trait TimerService {
    fn start_timer(&self);
    fn start_timer_star(&self);
    fn start_retransmit_timer(&self);
}

pub trait RoutingService {
    fn notify_plus(&self);
    fn notify_minus(&self);
    fn send_group_specific_query(&self, group: Ipv4Addr);
}
