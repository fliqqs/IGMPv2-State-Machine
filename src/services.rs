pub trait TimerService {
    fn start_timer(&self);
    fn start_timer_star(&self);
}

pub trait RoutingService {
    fn notify_plus(&self);
    fn notify_minus(&self);
    fn send_group_specific_query(&self, group: &str);
}
