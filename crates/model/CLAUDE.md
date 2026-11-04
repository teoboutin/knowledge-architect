# model

What knowledge-architect reads of a project. What it is for is `register@model@goal`; what it does for
a user is `path@model@README.md`; why it is shaped this way is `register@model@design`.

**An item is public only through the facade**, `path@model@src/lib.rs`: a re-export in one of its role
modules, each with its body in that file. Every other module is private, and its unit test refuses a
`pub mod <name>;` and a glob. An item the core or another crate calls is `pub` and named there; an
internal item is `pub(crate)`. A method of `Model` or `Manifest` made `pub` is public on the core's
re-export too, so it is made public for a caller's code and never for a test.
