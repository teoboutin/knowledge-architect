# crates.io: what limits a publish, and what a crate's name may be used for

Findings established outside this project that `design@knowledge-architect@new-crate-placeholder`
and `issue@knowledge-architect@every-crate-takes-a-version-at-each-release` lean on. Each is read
again at the source it names. The numbers are those of the source at the revision named, and are the
claim: a reader re-reads them there before relying on them.

## The rate limits, read in crates.io's source

Read in `path@elsewhere@src/rate_limiter.rs` of the repository rust-lang/crates.io, at the commit
07a026ab6cb3 of its main branch, on 2026-11-12.

- **A new crate, one whose name crates.io does not hold yet: a burst of 5, then 1 every 10
  minutes.** `default_burst` holds `LimitedAction::PublishNew => 5,`, `default_rate_seconds` holds
  `LimitedAction::PublishNew => 10 * 60, // 10 minutes`, and the test `default_rate_limits` says
  "Publishing new crates has a burst of 5 and refill time of 1 every 10 minutes".
- **A new version of an existing crate: a burst of 30, then 1 every minute.** `default_burst` holds
  `LimitedAction::PublishUpdate => 30,`, `default_rate_seconds` holds
  `LimitedAction::PublishUpdate => 60,   // 1 minute`, and the same test says "Publishing new
  versions has a burst of 30 and refill time of every minute".
- **Both limits count per account, across all its crates.** The bucket a publish takes a token from
  is keyed on `publish_limit_buckets::user_id` and the action, with no crate in the key. Which
  account a publish by trusted publishing counts against is **not established**.
- **An account may carry a raised burst.** The burst is read from the table
  `publish_rate_overrides` first, which holds a `burst` and an optional `expires_at`. How an owner
  obtains one is not established.
- **A refused publish answers HTTP 429, with the time to retry.**
  `path@elsewhere@src/util/errors/json.rs`, the type `TooManyRequests`: the status is `StatusCode::TOO_MANY_REQUESTS`, the header
  `RETRY_AFTER` holds the date, and the message is "{}. Please try again after {retry_after} and see
  https://crates.io/docs/rate-limits for more details.", where `{}` is "You have published too many
  new crates in a short period of time" or "You have published too many updates to existing crates
  in a short period of time".

## The usage policy on reserving a name

Read in `svelte/src/routes/policies/+page.svelte` of the same repository, at the same commit, the
source of https://crates.io/policies, on 2026-11-12. Among the uses the policy forbids, a package
that "exists only to reserve a name for a prolonged period of time (often called "name squatting")
without having any genuine functionality, purpose, or significant development activity on the
corresponding repository". The same page: "The crates.io team may delete crates from the registry
that do not comply with the policies on this document. In larger cases of squatting attacks this may
happen without prior notification to the author, but in most cases the team will first give the
author the chance to justify the purpose of the crate."
