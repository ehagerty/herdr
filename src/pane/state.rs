use crate::terminal::TerminalId;

/// Viewport state for a pane.
///
/// Terminal identity, cwd, labels, and agent metadata live in TerminalState.
pub struct PaneState {
    pub attached_terminal_id: TerminalId,
    /// Whether the user has seen this pane since its last state change to Idle.
    /// False = "Done" (agent finished while user was in another workspace).
    pub seen: bool,
    /// User-set "unread" flag (mark-unread). Shows the waiting tint while the
    /// pane is unfocused; cleared the instant the pane is focused. Focus-based,
    /// unlike `seen` (which herdr sets for any visible pane), so it works in
    /// all-visible split layouts — true read/unread semantics.
    pub marked_unread: bool,
}

impl PaneState {
    pub fn new(attached_terminal_id: TerminalId) -> Self {
        Self {
            attached_terminal_id,
            seen: true,
            marked_unread: false,
        }
    }
}
