#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SchedulingAlgorithm {
    RoundRobin,
    RealTime,
    Lottery
}
