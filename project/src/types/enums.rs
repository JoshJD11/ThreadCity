#[derive(Debug, Clone, Copy)]
pub enum State {
    Ready,
    Running,
    Blocked
}

#[derive(Debug, Clone, Copy)]
pub enum SchedulingAlgorithm {
    Round_Robin,
    Real_Time,
    Lottery
}
