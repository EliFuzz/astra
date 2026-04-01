pub(crate) fn is_action_key_meta() -> bool {
    cfg!(target_os = "macos")
}
