%% Delivered as a fragment: `design@agent-skills@shared-text-fragment`.
| what | where | why there |
| --- | --- | --- |
| the current design, and its **standing argument** | the Component's design home | it is rewritten in place, so it cannot go stale |
| a design not kept, and why | the Component's `register@*@rejected`, each entry cited by the heads whose argument covers it | a reader about to propose it must meet it without knowing to look, and a reader of the head meets its citation |
| the **deliberation**: what was weighed, in which order, what evidence was built, who ruled what | the spec or milestone document while it exists, then history; the commit message when there was no spec | failing to find it costs a re-derivation, not a wrong decision |

**The standing argument** is `primer@design-heads`, and what stays out of the head is `skill@knowledge-architect-decision-recording@head-shape`.

**The deliberation is not copied into the head**, and, where a plan document carries it, not into
the harvest's commit message either.
A plan document is deleted when its work lands, per `skill@knowledge-architect-planning`, and the
commit that deletes it cites it by its kind. A reader who needs the deliberation finds it there:

```sh
git log --diff-filter=D --name-only -- docs/plans/         # every deleted plan document, with its commit
git log -G'<slug>'                                         # every commit whose diff touches the slug
git show <commit>^:<path of the document>                  # the document as it stood before deletion
```

**`-G` matches any commit whose diff touches the slug. `-S` matches only a change in how many times
it occurs, so it misses every revision.**

When a decision was taken with no spec, on the in-change path of `skill@knowledge-architect-design`,
the deliberation is where that path puts it.
