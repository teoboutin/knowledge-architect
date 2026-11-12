# crates.io: what limits a publish, and what a crate's name may be used for

Findings established outside this project on how crates.io limits a publish, how a crate leaves it,
and what its policy allows a name to be used for. Each is read again at the source it names. The
numbers are those of the source at the revision named, and are the claim: a reader re-reads them
there before relying on them.

## The rate limits, read in crates.io's source

Read in the repository rust-lang/crates.io, at the commit 07a026ab6cb3 of its main branch, on
2026-11-12.

- **A new crate, one whose name crates.io does not hold yet: a burst of 5, then 1 every 10
  minutes.** In `path@elsewhere@src/rate_limiter.rs`, `default_burst` holds
  `LimitedAction::PublishNew => 5,`, `default_rate_seconds` holds
  `LimitedAction::PublishNew => 10 * 60, // 10 minutes`, and the test `default_rate_limits` says
  "Publishing new crates has a burst of 5 and refill time of 1 every 10 minutes".
- **A new version of an existing crate: a burst of 30, then 1 every minute.** The same file:
  `default_burst` holds `LimitedAction::PublishUpdate => 30,`, `default_rate_seconds` holds
  `LimitedAction::PublishUpdate => 60,   // 1 minute`, and the same test says "Publishing new
  versions has a burst of 30 and refill time of every minute".
- **Both limits count per account, across all its crates.** The bucket a publish takes a token from
  is keyed on `publish_limit_buckets::user_id` and the action, with no crate in the key. The burst is
  read from the table `publish_rate_overrides` first, which holds a `burst` and an optional
  `expires_at`; how an owner obtains a raised burst is not established.
- **A publish by trusted publishing counts against no account, so neither limit applies to it.** In
  `path@elsewhere@src/controllers/krate/publish.rs`, the limit is checked only inside
  `if let Some(user_id) = auth.user_id()`, and `AuthType::user_id` answers `None` for
  `AuthType::TrustPub(_)`. A publish with an API token counts against the token's account.
- **A per-crate daily cap applies to every publish.** The same file: where the configuration holds
  `new_versions_daily`, a crate that has published that many versions in the last 24 hours is
  refused with HTTP 429, "You have published too many versions of this crate in the last 24
  hours". `path@elsewhere@src/config/rate_limits.rs` reads it from the variable
  `MAX_NEW_VERSIONS_DAILY`; the deployed value is not in the source.
- **A refused publish answers HTTP 429, with the time to retry.**
  `path@elsewhere@src/util/errors/json.rs`, the type `TooManyRequests`: the status is
  `StatusCode::TOO_MANY_REQUESTS`, the header `RETRY_AFTER` holds the date, and the message is "{}.
  Please try again after {retry_after} and see https://crates.io/docs/rate-limits for more
  details.", where `{}` is "You have published too many new crates in a short period of time" or
  "You have published too many updates to existing crates in a short period of time".
- **The numbers are defaults.** The deployment may replace each through the configuration that
  `path@elsewhere@src/config/rate_limits.rs` reads.

## How a version and a crate leave crates.io

The same repository and commit. No handler of `path@elsewhere@src/controllers/version/` deletes a
version: `yank.rs` yanks one, and the others read or update it. A whole crate is deleted by its owner, per the doc
comment of `path@elsewhere@src/controllers/krate/delete.rs`: "only if the crate has been published
for less than 72 hours, or if the crate has a single owner, has been downloaded less than 1000 times
for each month it has been published, and is not depended upon by any other crate on crates.io."

## The usage policy on a crate's name

Read in `path@elsewhere@svelte/src/routes/policies/+page.svelte` of the same repository, at the same
commit, the source of https://crates.io/policies.

- **Names are first come, first served.** "crates.io has a first-come, first-serve policy on crate
  names."
- **A name may not be held without a crate.** Among the uses the policy forbids, a package that
  "exists only to reserve a name for a prolonged period of time (often called "name squatting")
  without having any genuine functionality, purpose, or significant development activity on the
  corresponding repository". The same page: "The crates.io team may delete crates from the registry
  that do not comply with the policies on this document. In larger cases of squatting attacks this
  may happen without prior notification to the author, but in most cases the team will first give
  the author the chance to justify the purpose of the crate."
