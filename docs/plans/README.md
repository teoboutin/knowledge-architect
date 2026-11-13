# Plan documents

This is the plans directory: specs and milestones for work that is designed and not yet built.
The checker constructs the anchor `plans` here, and it holds this file and two homes, nothing
else:

- [specs/](specs/README.md): one directory per spec, the plan of work done in one pull
  request, cited `spec@plans@<id>`;
- [milestones/](milestones/README.md): one directory per milestone, work across several pull
  requests, cited `milestone@plans@<id>`. Its `README.md` is the milestone document, its
  `index.md` is the generated listing of its slices, and each slice is a directory in it, cited
  `spec@<milestone>@<slice>`, moved under its `harvested/` once the slice lands.

A plan document may land before its work starts, and leaves in a commit of its own, after its last
harvest and the review before the merge that reads it. The procedure is the
installed `skill@knowledge-architect-planning`.
