-- Anonymous per-question answer distribution (vitrine "% des autres", ADR 0006).
--
-- Integrity-first: one count per client per question, so a re-judge overwrites a
-- client's choice rather than inflating the tally. The client id is the opaque,
-- random `raip_cohort_id` (no PII, no nominative link); this table is never
-- joined to the cohort tables — the pseudonymous link stays confined here.

create table anonymous_question_answers (
    -- opaque random client token (raip_cohort_id): pseudonymous, never nominative
    client_id text not null,
    question_id text not null,
    choice_id text not null,
    -- epoch seconds; retention is enforced against this, mirroring anonymous_sessions
    created_at bigint not null,
    -- integrity: a client counts once per question (a re-judge updates its choice)
    primary key (client_id, question_id)
);

-- read path: the distribution is count(*) grouped by choice for one question
create index anonymous_question_answers_qid_idx on anonymous_question_answers (question_id);

-- Audit of distribution-API access: access events only, never user data (ADR 0006).
create table question_access_audit (
    id bigserial primary key,
    accessed_at bigint not null,
    question_id text not null,
    cohort_size integer not null,
    threshold_met boolean not null
);
