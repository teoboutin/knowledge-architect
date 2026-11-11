# Model design: Reading a file

## Reading a file

### A parse that cannot be trusted is reported, never silent `##a-failed-parse-is-loud`

A file the walk cannot read, a source the grammar cannot parse, and a source nested deeper than the
walk goes all produce a finding naming the file. Silence would remove every citation in it from the
walk while the run reported success, which is indistinguishable from a clean file, against
`goal@documentation-stays-consistent`: a tree that passes the checker conforms, in the texts the
checker reads, to every rule it holds.

**A file outside the walk is held to the same, for the question an extension asks of it.** An
extension may assert that no unwalked file says something, as thaum's rules extension does of a rule
number. Its bytes are handed over decoded lossily when they are not UTF-8: the question is
whether an ASCII token appears, and a lossy decoding keeps every ASCII byte, where a walked
document read lossily would be text nobody wrote. **A binary file is handed to no check**, by
git's own test, a NUL byte in its first 8000 bytes, in `check` and in `commits` alike: its bytes
are no prose anybody wrote a claim in, and a number in them, such as a PDF's page size, is data.
Every other state is a phase-2 finding naming the file, each with its own repair: a path git
lists and the working tree does not hold, a directory where git lists one entry, and a file whose
bytes cannot be had at all. A symlink and a gitlink are no file to read, and are reported as
links.

The rival, a read or a parse that fails in silence, fails in two ways. Under it, one byte of
Windows-1252, such as a pasted em dash, puts a document read strictly outside the walk and outside
the inverse assertion, so a fabricated quote in it is read by nothing. And unbounded recursion
aborts the whole run with no file named and no finding printed.
