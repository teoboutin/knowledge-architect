%% Delivered as a fragment: `design@agent-skills@shared-text-fragment`; its moments are
%% `design@agent-skills@component-skill-delivery`.
**A Component may need creating or splitting** when work you are placing belongs to no existing
Component, or when the code you add gives a Component a second reason to change: two concrete
changes someone could plausibly ask for, each touching it independently. Before placing the work,
load `skill@knowledge-architect-component-creation`, which holds the default criteria, the shapes and
the procedure. A split is probably cut in the wrong place where it needs many new dependencies,
would create a dependency cycle, or would have every expected change cross the new boundary. A
creation or a split is a design decision, never a step taken while writing code.
