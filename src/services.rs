pub trait TimerService {
    fn start(&self, name: &str);
}

pub trait RoutingService {
    fn notify_plus(&self);
    fn notify_minus(&self);
}
