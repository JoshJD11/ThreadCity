#[derive(Debug, Clone, Copy)]
pub enum State {
    Ready,
    Running,
    Blocked,
    Terminated
}

#[derive(Debug, Clone, Copy)]
pub enum SchedulingAlgorithm {
    RoundRobin,
    RealTime,
    Lottery
}
