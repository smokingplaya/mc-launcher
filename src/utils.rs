// 200iq
#[cfg(target_os = "windows")]
pub(crate) fn choice_by_os<'a>(_: &'a str, on_windows: &'a str) -> &'a str {
  on_windows
}

#[cfg(not(target_os = "windows"))]
pub(crate) fn choice_by_os<'a>(on_linux: &'a str, _: &'a str) -> &'a str {
  on_linux
}