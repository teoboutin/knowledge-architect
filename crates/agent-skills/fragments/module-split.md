%% The module guidelines are `design@agent-skills@module-split-guidelines`; one
%% module owning one external effect is the shape of `design@git@git-is-asked-through-one-module`;
%% guidelines and not rules, per `goal@projects-are-split-into-modular-units`, which fixes no rule of
%% splitting. One fragment, hosted by the development skill, the code-shape reviewer and the
%% code-split scout, per `design@agent-skills@shared-text-fragment`. Size's role rests on the
%% measurement its head names: a read returns a file only up to a cap, and one with `allow_large`
%% only up to what fits in the context left. A split is planned and committed as
%% `design@agent-skills@splits-planned-before-work` and
%% `design@agent-skills@move-lands-in-move-only-commits` say.
**A module holds what one review reads against one reference.** A reference is a design head, a
section of a specification the project implements, a set of invariants, or one external effect, such
as a process the project runs, a store it writes or a service it calls, which one module owns. These
are guidelines a project weighs, never a rule a module fails alone. A module is a candidate for a
split where one of these signals holds:

| signal | the failure a split prevents |
| --- | --- |
| a second reference enters the module | a review that reads code outside its subject |
| two parts of the module change apart, counting the changes the goals already predict | unrelated changes in one file's diff |
| an invariant can only be protected by a boundary of visibility | a helper that bypasses the invariant made visible to code that must not call it |

Each signal is weighed against the signals against a split:

- **two parts that every change reads together**, such as a process and the changes it applies: a
  split leaves one review reading across two files;
- **one item large by itself**, such as an enumeration of many cases: moving code does not divide
  it, and dividing it is a design change;
- **one owner per external effect**: a split that spreads one effect over two modules leaves it with
  no owner.

**Size has one role: a file a session cannot read whole in one pass is a signal**, since a review
that reads it in parts judges code it has not read. No number of lines is a signal by itself.

**A split is a move before the work**, planned and committed as
`skill@knowledge-architect-planning@cutting-steps-and-slices` says.
