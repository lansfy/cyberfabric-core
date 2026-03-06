Created:  2026-03-06 by Constructor Tech
Updated:  2026-03-06 by Constructor Tech
# ADR-0002: Capability Model

**Date**: 2026-02-04

**Status**: accepted

**ID**: `fdd-chat-engine-adr-capability-model`

## Context and Problem Statement

Chat Engine needs to support different session types with varying capabilities (file attachments, web search, AI model selection, summarization). The system uses a two-tier model: `SessionType.available_capabilities` is the developer-configured catalog declaring the max set of capabilities a session type supports; `Session.enabled_capabilities` is the user-selected set of specific capabilities and their values, chosen from that catalog. Who should define which capabilities are available for each session type, and how should users control which capabilities are active?

## Decision Drivers

* Session types need a static catalog for UI discovery before creating a session
* Capability definitions need human-readable metadata (`name`, `description`) for UI rendering
* Capabilities need typed defaults (`default_value`) so clients can omit per-message overrides
* Enum capabilities need allowed value validation (`enum_values`)
* Users should control which capabilities are active and at what values per session
* Capability semantics should be opaque to Chat Engine (no hardcoded validation)
* New capabilities should be addable without infrastructure changes
* Session types should be independently evolvable

## Considered Options

* **Option 1: Developer configures catalog, user selects values** - Developer declares `SessionType.available_capabilities` (static max set); user selects `Session.enabled_capabilities` (specific values) from that catalog
* **Option 2: Capabilities configured in Chat Engine only** - Admin configures capabilities via Chat Engine UI/API per session type with no user-level selection

## Decision Outcome

Chosen option: "Developer configures catalog, user selects values", because it separates the concerns of capability definition (developer/admin responsibility) from capability activation (user responsibility), enables pre-session UI discoverability, gives users control over cost optimization and feature selection, and keeps Chat Engine agnostic to capability semantics.

### Consequences

* Good, because clients discover available capabilities before session creation via `SessionType.available_capabilities`
* Good, because capability `name`, `description`, and `type` enable rich UI for capability selection
* Good, because capability `default_value` allows per-message `CapabilityValue[]` to be optional (see ADR-0022)
* Good, because `enum_values` enables client-side validation for enum capabilities
* Good, because users control capability activation and values (cost optimization, privacy)
* Good, because Chat Engine doesn't need to understand capability semantics (stores and forwards)
* Good, because adding new capabilities requires only developer configuration, not infrastructure changes
* Bad, because developer must keep `SessionType.available_capabilities` consistent with backend plugin expectations
* Bad, because Chat Engine cannot validate capability correctness (trusts developer configuration)
* Bad, because capability schema is not enforced beyond basic type checking

### Confirmation

Confirmed when `SessionType` schema requires `available_capabilities` as a non-optional field, and `Session` schema stores `enabled_capabilities` populated from user selection at session creation. Verify via schema validation and API contract tests.

## Pros and Cons of the Options

### Option 1: Developer configures catalog, user selects values

Developer declares `SessionType.available_capabilities`; user selects `Session.enabled_capabilities` from that catalog.

* Good, because pre-session discoverability via static catalog
* Good, because separation of concerns — definition (developer) vs. activation (user)
* Good, because user-controlled cost optimization and feature selection
* Bad, because developer must keep catalog in sync with backend plugin expectations

### Option 2: Capabilities configured in Chat Engine only

Admin configures capabilities via Chat Engine UI/API per session type with no user-level selection.

* Good, because centralized capability management in one place
* Bad, because no user control over capability activation per session
* Bad, because requires Chat Engine changes to add or modify capabilities

## Related Design Elements

**Actors**:
* `fdd-chat-engine-actor-developer` - Declares `SessionType.available_capabilities` catalog (required field)
* `fdd-chat-engine-actor-client` - Selects `Session.enabled_capabilities` from the catalog; enables/disables features in UI
* `fdd-chat-engine-actor-webhook-backend` - Receives user-selected `CapabilityValue[]` per message (see ADR-0022); does not define capabilities

**Requirements**:
* `fdd-chat-engine-fr-create-session` - Session stores user-selected `enabled_capabilities`
* `fdd-chat-engine-fr-switch-session-type` - New session type catalog replaces available capabilities

**Design Elements**:
* `fdd-chat-engine-entity-session-type` - Includes `available_capabilities: Capability[]` (developer-configured catalog, required field)
* `fdd-chat-engine-entity-session` - Stores `enabled_capabilities: Capability[]` (user-selected subset of the catalog)
* `fdd-chat-engine-entity-capability` - `Capability` schema: `{id, name, description?, type, default_value, enum_values?}` — used in both tiers
* `fdd-chat-engine-entity-capability-value` - `CapabilityValue` schema: `{id, value}` — per-message capability override (see ADR-0022)
* `fdd-chat-engine-principle-webhook-authority` - Backend plugin receives selected capabilities but does not define them

**Related ADRs**:
* ADR-0006 (Webhook Protocol) - Defines events using `enabled_capabilities`
* ADR-0018 (Session Type Switching with Capability Updates) - Capability catalog changes when switching session type
* ADR-0022 (Per-Request Capability Filtering) - Client sends `CapabilityValue[]` per message; `Capability.default_value` makes per-message values optional