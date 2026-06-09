#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[allow(dead_code)]
pub enum AppLifecycleAction {
    ShowPanel,
    HidePanel,
    Quit,
}
