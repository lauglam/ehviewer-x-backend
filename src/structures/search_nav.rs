#[derive(Debug, PartialEq)]
pub struct SearchNav {
    /// First page, value is `None`
    /// 1. Gallery: ?prev=2453492
    /// 2. Favorites: ?prev=1496103-1669783692
    pub prev_opt: Option<String>,
    /// Last page, value is `None`
    /// 1. Gallery: ?next=2453493,
    /// 2. Favorites: ?next=1670171-1669783692
    pub next_opt: Option<String>,
    /// ?next=2453493&jump=1d
    /// ?next=2453493&jump=3d
    /// ?next=2453493&jump=1w
    /// ?next=2453493&jump=2w
    /// ?next=2453493&jump=1m
    /// ?next=2453493&jump=6m
    /// ?next=2453493&jump=1y
    /// ?next=2453493&jump=2y
    pub jump_opt: Option<String>,
    /// ?next=2453493&seek=2023-02-01
    pub seek_opt: Option<String>,
}
