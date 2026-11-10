# Check design: Registers and their homes

## Registers and their homes

### The designs a project did not keep are entries of a built-in heading register, `rejected`, at level three `##rejected-register`

Every component carries the register `rejected`, its home `register@*@rejected` a file
`rejected.md` or a directory `rejected/` with a README under the component's docs/, per
`design@check@heading-register-two-shapes`. An entry is a level-three
heading that names the design not kept, its slug last, and its body opens with a marker, `live` or
`void`, then the reason it was not kept. It is cited `rejected@<anchor>@<id>`, so `show` prints it
whole with every text that cites it, and a plan item, a review finding, a commit message or a head
can name one design.

It serves `goal@knowledge-architect@design-is-recorded-with-its-arguments`, which asks for the
alternatives that lost and why, and `goal@knowledge-architect@records-reach-their-reader`, which asks that a
recorded entry reach whoever needs it through checked references: an entry with no slug can be
cited by nothing, and `show` prints an entry whole only when it is one. The home stays apart from
the design home, on the owner's argument: an agent reading a decision file partly, by grep, could
take a rejected design for what is built, and separating the two at the level of the files keeps a
grep hit on one from reading as the other. The nearest rival, the paragraphs of a compiled
document with `show` printing each one that cites a head, delivers only to a session that runs
`show` on the right head, and leaves nothing for an item, a finding or a message to cite. The name
is `rejected`, and the home is named after it, both on the owner's word: the owner proposed the
name for a shorter citation, finding a register of alternatives alone a bit odd, and judged the
new home's name to read better and to avoid a confusion point. Which designs earn an entry is
`design@agent-skills@losing-alternatives-filter`.

The designs this decision did not keep: `rejected@check@paragraphs-with-show-printing-them`,
`rejected@check@kind-named-rejected-alternative`, `rejected@check@home-keeping-the-old-file-name` and
`rejected@check@validity-marker-checked`.

### The checker does not check a tripwire's horizon line, for now `##tripwire-horizon-unchecked`

A tripwire's `**Watches until:**` line, which `design@agent-skills@tripwire-horizon` gives every
entry, is written and read by the sessions and the reviewers that the installed workflow sends, and
no check of `check` reads it. Whether a horizon has passed is an event over the history the
checker does not judge, and a presence check would read a field of a heading entry, the syntax
`rejected@check@validity-marker-checked` refused for the `rejected` register. The owner chose to
keep the line out of the checker for now, judging a check not yet needed. The tripwire axis of the
project audit brings every entry to the line, per `design@agent-skills@tripwire-audit-axis`, and the
standing-state reviewer reads each entry written after it, per
`design@agent-skills@standing-state-review-before-every-merge`.

### A `rejected` entry that no design entry cites is a finding `##orphan-rejected-entry`

The last phase reports, at its definition site, every `rejected` entry that no entry of a `design`
register cites inside its own section, in any anchor. A citation from any other text, a goal, an
issue, a tripwire, another `rejected` entry, or a design home outside its entries, does not count.
The finding names the repair: cite it from the design head whose argument covers it, or from a head
of absence that records the decision not to build it; or, where what it lost to is an instruction no
head states, delete it and name the design and its reason beside that instruction, which
`skill@knowledge-architect-decision-recording@losing-alternatives` admits since no head cites it. It is raised over a tree in
`path@check@src/check/references.rs`, not over a commit message, which holds no design entry.

It serves `goal@knowledge-architect@documentation-stays-consistent`: a head cites the designs its argument
covers, per `design@agent-skills@rejected-citation-direction`, and this check is what makes every
`rejected` entry reachable from the text a design session reads, the head, rather than only from
`show`. Only a citation from a design entry counts, on the owner's judgement; a citation counts by
the reference rule of `design@agent-skills@a-reference-claims-a-revisit`.

The designs this decision did not keep: `rejected@check@goal-citing-a-rejected-entry`,
`rejected@check@entry-reachable-by-citing-a-goal` and `rejected@check@no-orphan-check`.

### A register instance's own options sit beside it, in `register.toml` `##instance-options-beside-the-instance`

Per-instance options go in `docs/<dir>/register.toml`. They cannot sit in
the manifest without a table keyed by anchor, and the shape that avoids the table — a single list
at the root — would put one component's groups in another's directory. Today the only such option
is `groups`. A key other than `groups` is a finding, a file with no `groups` is a finding, and a
`register.toml` beside a heading register is one too.

The design this decision did not keep: `rejected@check@instance-options-at-root`.

### A heading register's home is `<dir>.md` or a `<dir>/` directory, never both `##heading-register-two-shapes`

The single file serves an anchor whose register fits one document. The directory serves one whose
register has outgrown it. In the directory shape, `README.md` is the head: an introduction, and a
bullet list of markdown links, one per subdocument, each target written relative to the README.
The entries and their slug anchors live in the subdocuments.

**Both shapes serve every heading register**, so `design`, `goal`, `tripwire` and `rejected` accept the same
two, and so does any heading register a project declares.

Five assertions carry the shape, split over three places. phase 2 asserts exactly one
home per anchor and register and that a directory home carries its `README.md`;
`check::registers` asserts that the README links every markdown subdocument the walk covers — a gitignored scratch file owes nothing. That
each relative link resolves is the references family's assertion, made for every navigation file
alike per `design@check@links-are-navigation-rows`. The entity table of `design@model@one-entity-table`
accepts a slug definition only in the file home or in a subdocument, matched against the owning
anchor's own paths rather than against a filename suffix — a suffix match would accept a slug
in any file whose name ends in `design.md`, a plan document included.

**Exactly one home, because two give a decision two candidate places to land.** That is the
one-home failure, and the reader who finds one half acts on half the design.

**The README must link every subdocument, because a subdocument nobody links is a home nobody
finds.** A markdown link is the index's row because a reader can follow it where a renderer
shows the page, and a bullet list is its conventional shape — a table grows painful to edit as
soon as a row outgrows a short sentence. Both directions are asserted over the same links: an
existing subdocument no link names is reported, and a link that resolves to nothing is reported.
A fenced link is an illustration and discharges nothing, the stance a fenced definition takes
per `design@model@a-slug-elsewhere-defines-nothing`, and a link inside a code span is typography showing the
shape. **A row is any link shape `design@check@links-are-navigation-rows` reads**, a link
definition included, so a README written with reference-style links indexes its subdocuments
like one written with inline links.
A backticked path in the README stays what it is everywhere: a checked pointer in prose, not a
row of the index.

**The README defines no slugs.** It is the head and the index. A decision recorded there competes
with the subdocuments as a home, which is what the split exists to end.

### A file register is a directory of one file per entry, with a hand-written README and a generated index `##a-file-register-is-a-directory-of-entries`

Its home is `<dir>/`. Every `.md` file under it is an entry named by its basename, except
`README.md` and `index.md` at the instance's own top level; a subdirectory is a declared group
and nothing else, groups nest one level, and a file of another suffix is a finding. An entry
carries frontmatter with each declared metadata key, a level-one title, the declared level-two
sections in order, and — for the issue register — the level-three subsections its kind owes.

**One file per entry, because a tool that edits a tracker edits the whole file.** Sessions editing
a shared tracker with `sed` and a string replacement have replaced every occurrence in the file
more than once. One file per entry makes that failure unreachable across entries, gives each entry
its own history, and turns closing an entry into a dangling-reference work list: the file is
deleted, and every reference to it becomes a finding.

**The README is hand-written and the index is generated, and neither stands in for the other.**
The introduction is prose about the project and belongs in markdown, and the README is also what
keeps the directory in git, which an empty directory cannot be. The index is mandatory: with the
README already holding the directory open, the churn argument against generating one has no weight
left, and a listing nobody generates is one that silently stops listing. This head owns that the
index exists; what it holds is `design@model@a-file-register-index-is-rows`, and that it is generated
rather than written by hand is `design@check@generated-files-are-pure`.

The designs this decision did not keep: `rejected@check@index-introduction-in-manifest` and
`rejected@check@optional-index`.

### A file register's groups are subdirectories, declared beside the instance `##groups-are-subdirectories`

A subdirectory is visible to `ls` and to a listing without parsing anything, and the group is not
part of an entry's id, so regrouping is a `git mv` that breaks no reference. The declared list is what keeps the naming from
drifting, and it is checked in both directions. An entry may sit ungrouped at the top level, so a
two-entry instance owes no directory.

The design this decision did not keep: `rejected@check@theme-metadata-key`.

### The issue register's kind list is closed `##issue-kind-list-closed`

An unknown kind is a finding naming the list, and adding one is a reviewed manifest diff. The kind
decides which subsections the entry owes, so an unknown one would owe the wrong three in silence, which is what makes refusing it load-bearing
rather than tidy. The open set is `rejected@check@open-issue-kind-set`.

