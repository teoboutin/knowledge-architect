# Retrospective reports

This directory holds the analyses of the files of findings this repository receives: the workflow
files of retrospectives and of project audits, from this repository or from a project that uses
the workflow, and this repository's own retrospective project files. One file per received file,
named by its stem. Each analysis checks every finding of that file against the tree, the
registers, the goals and the design, proposes an action, and records the owner's ruling on it. The
decision is `design@knowledge-architect@committed-findings-analysis`, and the procedure is
`skill@knowledge-architect-retrospective-intake`, with what `skill@klarch-retrospective-intake`
adds.

An analysis is committed once the owner has ruled on every finding, before any outcome is carried
out. The issues it rules are opened and the findings ruled to be handled now are handled, in the
commits their routes need. The commit that carries out its last outcome, or a later commit of the
same branch, deletes it. So on the main branch this directory holds only the analyses whose
outcomes are not all carried out, and `git log -- docs/retrospective-reports/` lists every analysis
there has been.
