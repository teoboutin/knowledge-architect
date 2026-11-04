# Model design: Generated files

## Generated files

### A file register's index is a banner, a count and one row per entry, and its bytes are the contract `##a-file-register-index-is-rows`

The generated `index.md` of a file-register instance, in order:

| line | content |
| --- | --- |
| 1 | `**Generated — do not edit.** \`<command> index\``, with the project's declared command, per `design@check@declared-command` |
| 2 | blank |
| 3 | `<n> entries`, over every entry of the instance, grouped or not, and `1 entry` where the instance holds one |
| then | the ungrouped entries' table, under no heading |
| then | per group, a blank line, `## <group>`, and that group's table |

A table's columns are the register's declared metadata keys, **ordered by name**, then `title`
holding the entry's level-one heading as a link to the file, relative to the index's own
directory. Rows sort by the first metadata key and then by id, or by id alone where the register
declares none. Groups sort by name, and the ungrouped entries come first.

**A `|` inside a cell is escaped as `\|`.** It is the one character of a title or of a metadata
value that would otherwise end the cell and shift every column right of it, turning a listing into
a table that says something else. Nothing else in a cell is rewritten.

**The bytes are a contract because `generated` compares bytes.** Any change here fails the gate on
every committed index at once, which is the cost of the comparison being exact; the gain is that a
listing cannot drift by a character without being named.

**Rows only: no summary and no date.** A row changes on create, delete, retitle, regroup and a
metadata change, and on nothing else, so an index is regenerated rarely. A summary column would
restale it on every wording edit, and a date would break
`design@check@generated-files-are-pure` outright. Both are `rejected@model@index-summary-column` and
`rejected@model@index-last-change-date`.

**A Directory register's index has the same bytes with one column before the title, `id`**: one
row per entry, its id, then its README's level-one title linking `<id>/README.md`, sorted by id. Its rows come from the
entries, which are anchors, so no file inside an entry is a row; the entry's own index lists those.

**The rows come from the walked entries, not from the instance's declarations.** A group is the
subdirectory an entry sits in, so an undeclared group still lists its entries and a declared group
holding none prints no heading; an entry whose id, title or metadata the shape check refuses is
still listed, under its basename and an empty cell. A listing that hid what the directory holds
because a declaration disagreed would be a listing that stops listing, and the declaration is
`check::registers`' to report.

The design this decision did not keep: `rejected@model@lines-flag-on-interpretation-index`.

### The writer and the `generated` check take the generated files from one list `##one-list-of-generated-files`

**One list names the file-register indexes, and the writer and the gate both read it.**
`cargo klarch index` and the `generated` check take the same pairs of destination and expected
bytes from one function, so neither can generate one the other does not know about, nor disagree
about what is in it. Each is derived from the file-register instances, one per instance whose
directory is there. **An extension's generated files are read the same way**: the extension
names their paths before the walk and renders their bytes when prepared, and the writer and the
gate both take them from those two calls, per
`design@check@an-extension-plugs-in-through-phased-hooks`.

**An instance with no directory contributes no index**: generating into it would create a register home as a side effect of a listing, and the
missing home is what `tree` reports.
