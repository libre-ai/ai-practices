-- Anonymous cohort schema (ADR 0006). No nominative column exists anywhere:
-- a session is an opaque server-generated id and its per-axis practice levels.

create table anonymous_sessions (
    session_id text primary key,
    -- epoch seconds; the retention window is enforced against this (ADR 0006)
    completed_at bigint not null,
    created_at bigint not null
);

create table anonymous_session_axes (
    session_id text not null
        references anonymous_sessions (session_id) on delete cascade,
    axis text not null,
    level text not null,
    score double precision not null,
    primary key (session_id, axis)
);

create index anonymous_session_axes_axis_idx on anonymous_session_axes (axis);

-- Audit of cohort-API access: access events only, never user data (ADR 0006).
create table cohort_access_audit (
    id bigserial primary key,
    accessed_at bigint not null,
    axis text not null,
    cohort_size integer not null,
    threshold_met boolean not null
);
