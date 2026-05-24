# Milestones — VipaVault

Current: V1

---

## Milestone Activation Protocol

Milestone activation is a human-controlled project state change. Agents read the current milestone and enforce its scope; agents do not advance milestones unless the human explicitly asks.

### For Human

To activate a new milestone:

1. Set `Current:` to the new milestone id and name.
2. Mark the previous current milestone as `Status: done`.
3. Mark the new milestone as `Status: current`.
4. Add `Started: YYYY-MM-DD` to the new milestone.
5. Run `context-gen check-consistency .`.
6. Commit `.context/MILESTONES.md` separately when practical.

If a milestone transition changes an active decision, update `.context/TENSIONS_ACTIVE.md` or move the entry to `.context/TENSIONS_HISTORY.md` with `Status: ARCHIVED`.

### For Agent

At the start of every task:

1. Read `.context/MILESTONES.md`.
2. Treat `Current:` as the active scope boundary.
3. Load context only for modules relevant to the current milestone and task.
4. If the requested task is outside the current milestone, ask for confirmation or create an OPEN tension if it conflicts with a constraint.
5. Never activate, complete, or archive a milestone unless the human explicitly asks.
6. After code or context changes, run `context-gen build .` and `context-gen check-consistency .`.

---

## V1

Status: current

Scope: MVP desktop app for local encrypted hosting/email/domain credential management.

Rules:
- Implement `cpanel` and `directadmin` only.
- OAuth providers remain Phase 2.
- Sync remains manual with hard rate limit.
- Viewer mode remains read-only and must not call provider APIs when sync is disabled.
