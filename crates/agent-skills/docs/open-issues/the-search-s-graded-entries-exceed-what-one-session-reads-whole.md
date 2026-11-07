---
kind: design
---
# The record search's graded entries exceed what one session can read whole, at the design audit and the intake

## Summary

The design audit of a slice or a spec reads `bears.md` and `may-bear.md` whole, and the intake
reads whole every issue and tripwire graded `bears` or `may-bear`, per
`design@agent-skills@may-bear-read-on-touch`. Every slice audit of this repository since 2026-11-02
graded more entries than one session's context holds, so the sessions split the files among fresh
readers, or judged entries from their grade lines. No text says how the whole reading is performed
at that size, and every way of performing it contradicts a statement of a head.

## Details

### What

The texts, as they stand:

- the design audit, `skill@knowledge-architect-planning@working-a-slice`, in
  `path@agent-skills@content/skills/planning/SKILL.md`: "Read whole `bears.md` and `may-bear.md`,
  which the search's check writes, every entry the search grades `bears` or `may-bear` with every
  reference to it [...] and judge each against the slice; never act on an entry from the reason the
  search gave";
- the intake, in `path@agent-skills@content/skills/retrospective-intake/SKILL.md`: "Read whole, in
  `bears.md` and `may-bear.md`, which the search's check writes, every issue and tripwire the search
  grades `bears` or `may-bear`, every other entry it grades `bears`, and an entry graded `may-bear`
  where a proposal touches its subject";
- the design grounding, in `path@agent-skills@content/skills/design/SKILL.md`: "Then read whole
  `bears.md`, every entry the search grades `bears` with every reference to it".

`klarch worklist --verify` writes the two files whole and nothing else (`assemble` in
`path@query@src/worklist.rs`), and `worklist` takes no option that narrows the search: its one seed
option is `--seed`.

The head and its ground: `design@agent-skills@may-bear-read-on-touch` states "The design audit of a
slice or a spec reads both files whole", and that the session reads whole what it acts on, never a
summary of it, since the owner asked that it read the reported issues and tripwires themselves
rather than rely on a subagent's summary. `design@agent-skills@search-reads-the-whole-record`
refuses a lighter search whatever the work.

The instances, 11 sessions between 2026-10-31 and 2026-11-07, all of this repository: the intake 2,
the design grounding 2, the design audit of a slice 7. The sizes, as each retrospective workflow
file reports them, by `wc -w` where words are given:

- the intake of seventeen files: 200 entries graded `bears`, 83,788 words, and 164 `may-bear`,
  57,415 words; four checking subagents ran the checks on text, and the session read what its
  proposals rested on;
- the intake of forty files: 333 entries in 133,010 words and 249 in 78,434, over its `bears.md`
  and `may-bear.md`. It departed from `design@agent-skills@may-bear-read-on-touch`, and its record
  did not say so: nine checking subagents read the entries, each those its verdicts rested on, and
  the session read whole only what each outcome rested on. The analysis
  2026-10-31-knowledge-architect-intake-seventeen-files lists 185 `may-bear` entries as not read
  whole, 42 issues among them, which the head says the session reads whole. At the review before
  the merge, the design-conformance axis ran `cargo klarch show` on all 185 and read whole the 42
  issues' summaries and the entries whose subject the work touches: the work contradicts, widens or
  brings back none of them, so none reopens the head by its own clause;
- the design grounding of the development milestone: 136 entries, 61,473 words, read whole in
  five reads;
- the design grounding of one in-change decision before the 0.9.0 release: twelve searchers, 179
  entries graded `bears`, about 72,000 words;
- the slice audits: 172 `bears` (about 68,000 words) and 148 `may-bear` (about 60,000), the slice
  that moved `git` out; 888 KB and 810 KB for two slices of the development milestone; 292
  entries, the slice that renamed `check`; 347 entries, about 1 MB, the slice that built the
  `model` crate; 13,589 lines, 131,549 words, the slice that put `cli` on top; 425 entries, about
  1.1 MB, the slice that moved the consumers out; 166 and 199 entries, 1.07 MB, about 146,000
  words, the slice of the integrated tests.

The practice: six of the seven slice audits split the files among fresh readers, up to ten of
them, each reading a part whole and returning one line per entry, the session reading whole what
each gap rests on, the parts cut by a script in at least two of them; the seventh judged most
entries from their grade lines. Three failures of a reading were seen:

- entries judged from their grade lines: the slice that moved `git` out, where the
  design-conformance axis at the merge then read 266 of them whole and found five items among
  them, and a session of the Component rules revision of 2026-11-02, which judged most `may-bear`
  entries from the reason after a sample;
- a forked agent, holding the inherited context, could not read 3,546 lines and judged from
  headings and reasons; three fresh agents read that range again (development milestone);
- readers' lines found false or overtaken by the review (the slice that renamed `check`), a lapse
  of "Re-verify a conclusion before acting on it" of `primer@verify-before-relying`.

The rivals, as the analysis weighs them:

1. fresh readers over parts cut at entry boundaries, each reading its part whole and writing one
   line per entry, the session reading whole what each gap rests on: the only practice that read
   every entry; against it, a reader's line is the summary the head's ground refuses, and the lines
   were found false once;
2. the reading on touch at the audit and the intake, as at the design grounding: against it, the
   one audit that judged entries from their grade lines had five items found among those entries,
   so entries left unread at an audit did bear;
3. a smaller output: narrower seeds (`issue@agent-skills@a-slice-s-design-audit-is-seeded-with-every-decision-of-its-milestone`),
   one grade line per distinct reason
   (`issue@query@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`), a lighter pass
   (`issue@agent-skills@a-lighter-pass-for-untouched-components`): each reduces the size, and none
   bounds it;
4. `--verify` writing the files as packs under the bound it uses for clusters: a means to rival 1,
   not a rival, and the mechanical half `query` would build.

A fact for the discussion: the installed text already holds rival 1's shape at one occasion,
`skill@knowledge-architect-retrospective-intake@intake-subagents`: the checks on text may be run by
fresh subagents, and the session itself re-checks every verdict a proposal rests on. The fact that
would decide between rivals 1 and 2 is whether the owner's ask covers a reader's line that the
session re-checks where it acts, which is the owner's word.

The suspected mechanism, in one sentence: the whole reading was decided at a size one session
holds, and the search's output grew past it with the record and the seeds.

Re-entry point: a design discussion under `skill@knowledge-architect-design`, on the owner's word,
the order of known work being the owner's. Its outcome is also a re-entry point of
`issue@agent-skills@a-lighter-pass-for-untouched-components`, since packs change how the search
assembles the record, and it weighs
`issue@query@the-graded-entries-repeat-a-shared-entry-s-grade-per-agent`.

Source: W1 of the analysis 2026-10-31-knowledge-architect-intake-seventeen-files-klarch-workflow,
in the commit "[intake] The analyses of forty received retrospective files, three outcomes for the
owner", with the ten findings it groups as cluster A.

### Why it matters

An instruction no session can follow as written is followed in a shape each session chooses, and
two of the three shapes seen judged entries from a summary, against
`goal@knowledge-architect@agents-work-without-drift`, whose reading of the record against the work
the audit serves. It strains `design@agent-skills@may-bear-read-on-touch`, whose audit sentence and
intake paragraph state the reading. The packs serve `goal@knowledge-architect@mechanical-tasks-are-the-tool-s`
once the owner takes the task in.

### What would close it

A discussion under `skill@knowledge-architect-design` that chooses between the rivals above, or
keeps the whole reading with a way to perform it, and records the outcome in
`design@agent-skills@may-bear-read-on-touch` or reverses it under
`skill@knowledge-architect-decision-recording`; then the planning and intake texts follow, and,
where the choice needs packs, `query` builds them.
