export const meta = {
  name: 'knowledge-architect-tripwire-audit',
  description: 'One pass of the tripwire axis of a project audit, from its preflight to a report the session verifies',
  phases: [
    { title: 'preflight' }, { title: 'setup' }, { title: 'drafts' }, { title: 'calibration' },
    { title: 'verify' }, { title: 'cross-check' }, { title: 'report' }, { title: 'judge' },
  ],
}
// args: {
//   commit      the commit audited
//   scratch     the run's scratch directory; each step and each agent writes under it
//   worktrees   the directory under which the run creates the worktrees
//   history     a git range whose commit messages and added report files record the earlier passes
//   previous    optional: for a re-check, the commit the previous pass audited
//   kept        optional: the path of the kept list, for a re-check
//   sample      optional: the calibration sample, an array of entries, each { ref } for a
//               tripwire; the preflight chooses one if absent
//   counts      optional: { groupSize }, a missing key taking its default
//   from        optional: 'calibration' or 'verify', to restart from the files a run wrote; or
//               'application', to judge the session's application of a pass instead of auditing
//   applied     with from 'application': the commit that holds the application
// }
const counts = args.counts || {}
// Lower than the design-record axis's 60: each tripwire is read with the entry it guards, whole,
// and many with the history of a ruling of the owner.
const groupSize = counts.groupSize === undefined ? 30 : counts.groupSize
if (!(groupSize >= 1)) throw new Error('groupSize: at least 1')
if (args.from && args.from !== 'calibration' && args.from !== 'verify' && args.from !== 'application') {
  throw new Error('from: calibration, verify or application')
}
if (args.from === 'application' && !(args.commit && args.applied)) {
  throw new Error('commit and applied: the commit audited and the commit of the application')
}
const at = (...parts) => parts.join('/')
const lines = (xs) => xs.filter(Boolean).join('\n')
const AUDITOR = 'knowledge-architect-tripwire-auditor'
const SECTION = 'the section tripwire-axis of the skill knowledge-architect-project-audit'
const reading = at(args.worktrees, 'reading')
const preflightDir = at(args.scratch, 'preflight')
const groupsFile = at(args.scratch, 'setup', 'groups.json')
const calibrationFile = at(args.scratch, 'calibration', 'calibration.md')
// The home of this line is the bullet of review-invariants, in the skill knowledge-architect-review,
// that names the files a subagent's copy may predate: the definition, the root CLAUDE.md and the
// primer it imports.
const FROM_DISK = 'Your copies of your definition, of the root CLAUDE.md and of the primer it ' +
  'imports may predate this session: read each in the worktree this brief names, and follow the ' +
  'worktree where your copy differs.'
// The outcomes of the axis, in the order a draft proposes the first that applies.
const OUTCOMES = ['fired', 'decision-gone', 'horizon-passed', 'removed', 'converted', 'absorbed', 'reworded', 'kept']

const ENTRY = {
  type: 'object',
  properties: {
    ref: { type: 'string' }, file: { type: 'string' }, line: { type: 'number' },
    removed: { type: 'boolean' },
  },
}
const ENTRIES = {
  type: 'object',
  properties: {
    components: {
      type: 'array',
      items: { type: 'object', properties: { name: { type: 'string' }, entries: { type: 'array', items: ENTRY } }, required: ['name', 'entries'] },
    },
    sample: { type: 'array', items: ENTRY },
    listed: { type: 'number' },
  },
  required: ['components', 'sample', 'listed'],
}
const GROUPS = {
  type: 'object',
  properties: {
    groups: {
      type: 'array',
      items: {
        type: 'object',
        properties: { label: { type: 'string' }, components: { type: 'array', items: { type: 'string' } }, entries: { type: 'array', items: ENTRY } },
        required: ['label', 'components', 'entries'],
      },
    },
    sample: { type: 'array', items: ENTRY },
  },
  required: ['groups', 'sample'],
}
const DRAFTED = {
  type: 'object',
  properties: {
    judged: {
      type: 'array',
      items: {
        type: 'object',
        properties: { entry: { type: 'string' }, outcome: { type: 'string', enum: OUTCOMES }, ground: { type: 'string' } },
        required: ['entry', 'outcome', 'ground'],
      },
    },
  },
  required: ['judged'],
}
const JUDGED = {
  type: 'object',
  properties: {
    judged: {
      type: 'array',
      items: { type: 'object', properties: { entry: { type: 'string' }, verdict: { type: 'string', enum: ['conforms', 'fails'] } }, required: ['entry', 'verdict'] },
    },
  },
  required: ['judged'],
}
const CALIBRATED = {
  type: 'object',
  properties: { path: { type: 'string' }, proposed: { type: 'number' }, bothReadings: { type: 'number' } },
  required: ['path', 'proposed', 'bothReadings'],
}
const VERDICTS = {
  type: 'object',
  properties: {
    path: { type: 'string' }, verified: { type: 'number' }, refuted: { type: 'number' },
    plausible: { type: 'number' }, bearsOnRuling: { type: 'number' }, fires: { type: 'number' },
    known: { type: 'number' }, keptList: { type: 'number' }, deletions: { type: 'number' },
  },
  required: ['path', 'verified', 'refuted', 'plausible', 'bearsOnRuling', 'fires', 'known', 'keptList', 'deletions'],
}
const PATH = { type: 'object', properties: { path: { type: 'string' } }, required: ['path'] }
const DONE = {
  type: 'object',
  properties: { done: { type: 'boolean' }, notes: { type: 'string' } },
  required: ['done', 'notes'],
}

// An entry is named by its reference, tripwire@<anchor>@<id>.
const id = (e) => e.ref
const show = (e) => e.ref + (e.removed ? ' (deleted by the application)' : '')

// The cut: an anchor above groupSize is cut alone into groups of consecutive entries whose sizes
// differ by at most one; the others are packed whole, in the listing's order, each into the first
// packed group it fits in. A packed group holding one anchor takes that anchor's name.
const cut = (components) => {
  const out = []
  for (const c of components) {
    const n = c.entries.length
    if (n === 0) continue
    if (n > groupSize) {
      const k = Math.ceil(n / groupSize)
      const base = Math.floor(n / k)
      let start = 0
      for (let g = 0; g < k; g++) {
        const size = base + (g < n % k ? 1 : 0)
        out.push({ label: c.name + '-' + (g + 1), components: [c.name], entries: c.entries.slice(start, start + size) })
        start += size
      }
    } else {
      let target = out.find((g) => g.packed && g.entries.length + n <= groupSize)
      if (!target) {
        target = { packed: true, components: [], entries: [] }
        out.push(target)
      }
      target.components.push(c.name)
      target.entries = target.entries.concat(c.entries)
    }
  }
  let p = 0
  for (const g of out) {
    if (!g.packed) continue
    g.label = g.components.length === 1 ? g.components[0] + '-1' : 'packed+' + ++p
    delete g.packed
  }
  return out
}
const total = (components) => components.reduce((t, c) => t + c.entries.length, 0)

// The judgement of an application: every tripwire the session's application of a pass changed or
// deleted, judged after the session applies the pass, as a run of its own; it ends at its report.
if (args.from === 'application') {
  const dir = at(args.scratch, 'application')
  const judging = at(args.worktrees, 'application')
  phase('preflight')
  const pre = await agent(lines([
    'You set up the judgement of the application of one pass of the tripwire axis of a project ' +
      'audit. From the project root:',
    '- run git worktree add --detach ' + judging + ' ' + args.applied + ', and work in that ' +
      'worktree for every step below;',
    '- list every tripwire whose text differs between ' + args.commit + ' and ' + args.applied +
      ', an entry the application deleted included, marked removed true, with its file and line at ' +
      args.commit + ', and an entry whose only change is the target of a reference left out, its ' +
      'words unchanged; read git diff ' + args.commit + ' ' + args.applied + ' over every ' +
      'tripwires home whole, the homes the checker command cargo klarch anchors lists. Order them as ' +
      'the section take-your-group of the agent definition ' + AUDITOR + ' states, which you read ' +
      'whole, a deleted entry where the listing at ' + args.commit + ' placed it; group them by ' +
      'anchor, each as { ref, file, line, removed }, ref tripwire@<anchor>@<id>;',
    '- write that list, as JSON, to ' + at(dir, 'entries.json') + ', creating the directory;',
    '- run cargo klarch issues and save its output to ' + at(dir, 'issues.txt') + ', and ' +
      'cargo klarch tripwires to ' + at(dir, 'tripwires.txt') + '.',
    'Edit nothing else. Return the anchors as components with their entries, the sample as an ' +
      'empty array, and listed as the count of entries you listed.',
  ]), { schema: ENTRIES, phase: 'preflight', label: 'preflight' })
  if (!pre) throw new Error('preflight returned nothing')
  if (total(pre.components) !== pre.listed) {
    throw new Error('preflight: ' + pre.listed + ' entries listed, ' + total(pre.components) + ' returned')
  }
  const jgroups = cut(pre.components)
  if (!jgroups.length) {
    log('the application changed no tripwire')
    return { report: null, groups: [], failing: 0, gaps: [] }
  }
  phase('setup')
  const setup = await agent(lines([
    'You set up the groups of the judgement of an application. From the project root:',
    '- write this JSON, exactly, to ' + at(dir, 'groups.json') + ':',
    JSON.stringify({ groups: jgroups, sample: [] }),
    '- for each of these paths, run git worktree add --detach <path> ' + args.applied + ': ' +
      jgroups.map((g) => at(args.worktrees, 'application-' + g.label)).join(', ') + '.',
    'Edit nothing else. Return done true only if every command succeeded.',
  ]), { schema: DONE, phase: 'setup', label: 'setup' })
  if (!setup || !setup.done) throw new Error('setup failed: ' + (setup ? setup.notes : 'no return'))
  phase('judge')
  const jfailures = []
  const judged = await parallel(jgroups.map((g) => () => agent(lines([
    'Application brief.',
    'Commit audited: ' + args.commit,
    'Commit of the application: ' + args.applied,
    'Your worktree, detached at the application: ' + at(args.worktrees, 'application-' + g.label),
    'Your entries, ' + g.entries.length + ', each changed or deleted by the application:',
    g.entries.map((e, i) => (i + 1) + '. ' + show(e)).join('\n'),
    'Issues listing: ' + at(dir, 'issues.txt'),
    'Tripwires listing: ' + at(dir, 'tripwires.txt'),
    'Your directory: ' + at(dir, g.label),
    'In the return, name each entry by its reference, as listed here, without the mark of a ' +
      'deleted entry.',
    FROM_DISK,
  ]), { agentType: AUDITOR, schema: JUDGED, phase: 'judge', label: 'judge ' + g.label })
    .then((r) => {
      if (!r) jfailures.push('the judge of group ' + g.label + ' returned nothing')
      return { g, r }
    })))
  // An entry a judge leaves out of its return is judged by nobody, so it is named, as the drafts'
  // gaps are, rather than read as conforming.
  const jgaps = []
  for (const x of judged) {
    if (!x || !x.r) {
      jgaps.push((x ? x.g.label : 'a group') + ': no return')
      continue
    }
    const got = new Set(x.r.judged.map((j) => j.entry))
    for (const e of x.g.entries) if (!got.has(id(e))) jgaps.push(x.g.label + ': ' + show(e))
  }
  const failing = judged.filter((x) => x && x.r).reduce((t, x) => t + x.r.judged.filter((j) => j.verdict === 'fails').length, 0)
  log('judge: ' + jgroups.length + ' groups, ' + failing + ' entries failing' +
    (jgaps.length ? '; entries missing from a return: ' + jgaps.join('; ') : ''))
  // The judges' verdicts reach the report as the script holds them: a judge writes a file for a
  // failing entry and returns every verdict, so a report read from the files alone would take each
  // conforming entry for one nobody judged.
  const verdicts = judged.filter((x) => x && x.r).map((x) => ({ group: x.g.label, judged: x.r.judged }))
  phase('report')
  const jreport = await agent(lines([
    'You write the report of the judgement of an application of one pass of the tripwire axis, ' +
      'for the session that repairs it. Quote; do not summarise evidence. You edit nothing in the ' +
      'project.',
    'Every verdict the judges returned, per group: ' + JSON.stringify(verdicts),
    'Read every file under ' + dir + ' that the judges wrote.',
    'Write ' + at(dir, 'report.md') + ': per failing entry, the rule, the failure mode of an edit ' +
      'or the ruling it fails, the words quoted, and the repair the judge proposes; then the counts ' +
      'of entries judged and failing, each entry missing from a return, which nobody judged' +
      (jgaps.length ? ': ' + jgaps.join('; ') : '') + ', and every step that returned nothing' +
      (jfailures.length ? ', among them: ' + jfailures.join('; ') : '') + '.',
    'Return its path.',
  ]), { schema: PATH, phase: 'report', label: 'report' })
  return { report: jreport ? jreport.path : null, groups: jgroups.map((g) => g.label), failing, gaps: jgaps }
}

let groups = []
let sample = []
// Every step that fails or returns nothing, for the report's part 6.
const failures = []
if (!args.from) {
  phase('preflight')
  const pre = await agent(lines([
    'You set up one pass of the tripwire axis of a project audit. From the project root:',
    '- run git worktree add --detach ' + reading + ' ' + args.commit + ', and work in that worktree ' +
      'for every step below;',
    '- run the checker command cargo klarch tripwires and save its output to ' +
      at(preflightDir, 'tripwires.txt') + ', creating the directory;',
    '- list every entry of that output, in its order, which the section take-your-group of the ' +
      'agent definition ' + AUDITOR + ' states, which you read whole in the worktree: one entry per ' +
      'row, the header row excluded, a listing that prints only (no entry) giving none. Group them ' +
      'by anchor, the anchors in the order of their first row. Give each entry as { ref, file, ' +
      'line }, ref tripwire@<anchor>@<id>, its file and line those cargo klarch show prints;',
    '- write that list, as JSON, to ' + at(preflightDir, 'entries.json') + ';',
    args.sample ? '' : '- choose the calibration sample: about five entries, from different ' +
      'anchors where there are several, in the same shape;',
    '- run cargo klarch issues and save its output to ' + at(preflightDir, 'issues.txt') + ';',
    '- save the messages of the commits of the range ' + args.history + ', with git log ' +
      '--format="=== %h %s%n%n%b", to ' + at(preflightDir, 'history.txt') + ';',
    '- save the report files of earlier passes that the range added, with git log -p ' +
      '--diff-filter=A --format="=== %h %s" ' + args.history + ' -- docs/audit-reports/, to ' +
      at(preflightDir, 'reports.txt') + '.',
    'Edit nothing else. Return the anchors as components with their entries, the sample' +
      (args.sample ? ' as an empty array' : '') + ', and listed as the count of rows of the ' +
      'tripwires listing, the header row excluded.',
  ]), { schema: ENTRIES, phase: 'preflight', label: 'preflight' })
  if (!pre) throw new Error('preflight returned nothing')
  // A listing row the preflight left out of its entries is audited by nobody.
  if (total(pre.components) !== pre.listed) {
    throw new Error('preflight: ' + pre.listed + ' rows listed, ' + total(pre.components) + ' entries returned')
  }
  sample = args.sample || pre.sample
  groups = cut(pre.components)
  if (!groups.length) throw new Error('preflight listed no entry')

  phase('setup')
  const setup = await agent(lines([
    'You set up the groups of one pass of the tripwire axis of a project audit. From the project ' +
      'root:',
    '- write this JSON, exactly, to ' + groupsFile + ', creating the directory:',
    JSON.stringify({ groups, sample }),
    '- for each of these paths, run git worktree add --detach <path> ' + args.commit + ': ' +
      groups.map((g) => at(args.worktrees, g.label)).join(', ') + ';',
    args.kept ? '- copy the kept list ' + args.kept + ' to kept.md in each of these directories, ' +
      'creating it: ' + groups.map((g) => at(args.scratch, g.label)).join(', ') + '.' : '',
    'Edit nothing else. Return done true only if every command succeeded.',
  ]), { schema: DONE, phase: 'setup', label: 'setup' })
  if (!setup || !setup.done) throw new Error('setup failed: ' + (setup ? setup.notes : 'no return'))
} else {
  phase('setup')
  const back = await agent('Read ' + groupsFile + ' and return its groups and its sample exactly as ' +
    'written.', { schema: GROUPS, phase: 'setup', label: 'groups' })
  if (!back) throw new Error('could not read ' + groupsFile)
  groups = back.groups
  sample = back.sample
}
log('groups: ' + groups.map((g) => g.label + ' (' + g.entries.length + ')').join(', ') + '; sample: ' + sample.length)

// A sample entry's draft of record is that of the group whose listed entries hold it.
const ownerOf = (e) => {
  const g = groups.find((x) => x.entries.some((y) => id(y) === id(e)))
  return g ? g.label : 'none'
}
const sampleLines = sample.map((e) => '- ' + show(e) + ' (draft of record: group ' + ownerOf(e) + ')')

// Drafts, all groups in parallel; the calibration waits for every one.
const gaps = []
const proposed = {}
if (!args.from) {
  phase('drafts')
  const draftBrief = (g) => lines([
    'Drafting brief.',
    'Commit audited: ' + args.commit,
    'Your worktree, detached at that commit: ' + at(args.worktrees, g.label),
    'Your group: ' + g.label + ', ' + g.entries.length + ' entries, in this order:',
    g.entries.map((e, i) => (i + 1) + '. ' + show(e)).join('\n'),
    'The calibration sample, which you judge exactly as your own:',
    sample.filter((e) => !g.entries.some((y) => id(y) === id(e))).map((e) => '- ' + show(e)).join('\n'),
    'Scratch directory: ' + at(args.scratch, g.label) + '. Name each draft <nn>-<id>.md: nn its ' +
      'position, two digits, the sample following your group in its order; id the entry\'s id.',
    args.kept ? 'Kept list, for this re-check: ' + at(args.scratch, g.label, 'kept.md') : '',
    'In the return, name each entry by its reference, as listed here, and give its outcome and, as ' +
      'its ground, the condition or the exit it rests on.',
    FROM_DISK,
  ])
  const drafted = await parallel(groups.map((g) => () => agent(draftBrief(g),
    { agentType: AUDITOR, schema: DRAFTED, phase: 'drafts', label: 'drafts ' + g.label })
    .then((r) => ({ g, r }))))
  for (const x of drafted) {
    if (!x || !x.r) {
      failures.push('the drafts of group ' + (x ? x.g.label : '(unknown)') + ' returned nothing')
      gaps.push((x ? x.g.label : 'a group') + ': no return')
      continue
    }
    const got = new Set(x.r.judged.map((j) => j.entry))
    for (const e of x.g.entries) if (!got.has(id(e))) gaps.push(x.g.label + ': ' + show(e))
    for (const j of x.r.judged) proposed[j.outcome] = (proposed[j.outcome] || 0) + 1
  }
  if (gaps.length) log('entries missing from a return: ' + gaps.join('; '))
  log('drafts: ' + OUTCOMES.map((o) => o + ' ' + (proposed[o] || 0)).join(', ') + ', the sample counted once per group')
}

// Calibration: one auditor compares the sample's outcomes across the groups.
if (!args.from || args.from === 'calibration') {
  phase('calibration')
  const cal = await agent(lines([
    'Calibration brief.',
    'Commit audited: ' + args.commit,
    'Your worktree, detached at that commit: ' + reading,
    'The drafters\' scratch directories: ' + groups.map((g) => at(args.scratch, g.label)).join(', '),
    'The sample:',
    sampleLines.join('\n'),
    'Your directory: ' + at(args.scratch, 'calibration') + '; write calibration.md there.',
    FROM_DISK,
  ]), { agentType: AUDITOR, schema: CALIBRATED, phase: 'calibration', label: 'calibration' })
  if (!cal) {
    failures.push('the calibration returned nothing, so the verifiers had no calibration file')
    log('calibration returned nothing')
  }
}

// Verify: one auditor per group, each given the calibration.
phase('verify')
const verifyBrief = (g) => lines([
  'Verify brief.',
  'Commit audited: ' + args.commit,
  'Your worktree, detached at that commit: ' + at(args.worktrees, g.label),
  'The drafts to verify: the group ' + g.label + ', in ' + at(args.scratch, g.label) + ', its ' +
    'entries ' + show(g.entries[0]) + ' to ' + show(g.entries[g.entries.length - 1]) + '.',
  'The sample, each entry with the group whose draft is of record:',
  sampleLines.join('\n'),
  'The calibration file: ' + calibrationFile,
  'Your directory: ' + at(args.scratch, 'verify', g.label),
  args.kept ? 'Kept list: ' + at(args.scratch, g.label, 'kept.md') : '',
  'Issues listing: ' + at(preflightDir, 'issues.txt'),
  'Tripwires listing: ' + at(preflightDir, 'tripwires.txt'),
  'History of the audit earlier passes: ' + at(preflightDir, 'history.txt') + ', and their ' +
    'report files, with the owner\'s rulings: ' + at(preflightDir, 'reports.txt'),
  args.previous ? 'For this re-check, the commit the previous pass audited: ' + args.previous : '',
  FROM_DISK,
])
const verdicts = (await parallel(groups.map((g) => () => agent(verifyBrief(g),
  { agentType: AUDITOR, schema: VERDICTS, phase: 'verify', label: 'verify ' + g.label })
  .then((v) => {
    if (!v) failures.push('the verifier of group ' + g.label + ' returned nothing')
    return v ? Object.assign({ label: g.label }, v) : null
  })))).filter(Boolean)
const sum = (k) => verdicts.reduce((t, v) => t + v[k], 0)
log('verify: ' + verdicts.length + ' of ' + groups.length + ' groups verified; ' + sum('verified') +
  ' verified, ' + sum('refuted') + ' refuted, ' + sum('plausible') + ' plausible, ' +
  sum('bearsOnRuling') + ' bearing on a ruling, ' + sum('fires') + ' firing, ' +
  sum('known') + ' known, ' + sum('keptList') + ' on the kept list, ' + sum('deletions') +
  ' deleting their entry')

// Cross-check.
phase('cross-check')
const cross = at(args.scratch, 'cross-check', 'cross-check.md')
await agent(lines([
  'You cross-check the verified findings of one pass of the tripwire axis of a project audit, at ' +
    'commit ' + args.commit + ', in the worktree ' + reading + '. You edit nothing in the project, ' +
    'and write only ' + cross + '.',
  'Read every verdicts.md under ' + at(args.scratch, 'verify') + ' and the drafts they name, the ' +
    'calibration file ' + calibrationFile + ', and the outcome table of ' + SECTION + '. A sample ' +
    'entry\'s finding is its draft of record only:',
  sampleLines.join('\n'),
  'Write ' + cross + ' in three parts:',
  '1. the overlap map: every set of verified or plausible findings that touch one entry, an ' +
    'absorption into an entry and a rewording or a deletion of that same entry included; every ' +
    'pair of entries, in one group or in two, proposed absorbed into each other or into a third; ' +
    'and every set of entries guarding one head, from the guarding column of the tripwires listing ' +
    at(preflightDir, 'tripwires.txt') + ', whose drafts judge the same premise differently;',
  '2. the stale texts: for each entry a verified finding deletes or rewords, every text that cites ' +
    'it, from the referenced-at part of the checker command cargo klarch show on it, and whether the ' +
    'edit leaves that text false or dangling; a rejected entry kept on a doubt and naming a deleted ' +
    'entry is named, since the lifecycle judges it again by the recording tests;',
  '3. for each verified or plausible finding, the outcome you propose under the outcome table, ' +
    'with its reason, for the session to decide; for an entry the outcome deletes, the group of ' +
    'the report it is listed in: its outcome, and for a removal the condition it fails, 1, 2 or 3, ' +
    'or the doubt\'s weight, which alone puts it on the owner list.',
  'Return its path.',
]), { schema: PATH, phase: 'cross-check', label: 'cross-check' })
  .then((c) => {
    if (!c) failures.push('the cross-check returned nothing')
    return c
  })

// Report.
phase('report')
const report = await agent(lines([
  'You write the report of one pass of the tripwire axis of a project audit, for the session that ' +
    'verifies it and discusses it with the owner. Quote; do not summarise evidence. You edit ' +
    'nothing in the project.',
  'Read: every return.md under ' + args.scratch + ', the calibration file ' + calibrationFile +
    ', every verdicts.md under ' + at(args.scratch, 'verify') + ' and the drafts they name, and ' +
    'the cross-check ' + cross + '.',
  'The groups: ' + groups.map((g) => g.label + ', ' + show(g.entries[0]) + ' to ' +
    show(g.entries[g.entries.length - 1]) + ', ' + g.entries.length + ' entries').join('; ') + '.',
  'The sample, each entry with the group whose draft is of record:',
  sampleLines.join('\n'),
  gaps.length ? 'Entries missing from a return: ' + gaps.join('; ') + '.' : '',
  'Write ' + at(args.scratch, 'report', 'report.md') + ' in six parts:',
  '1. the header: the commit audited, ' + args.commit + '; the agents of each step; each group ' +
    'with its first and last entry; the count of entries, of drafts per outcome, and of those ' +
    'verified, refuted, plausible, bearing on a ruling, firing, known and on the kept list; for a ' +
    're-check, the counts the section record-of-an-audit of the skill ' +
    'knowledge-architect-project-audit asks of a pass, the owner-list one as the cross-check ' +
    'proposes it; each entry missing from a return; each refuted draft with its reason;',
  '2. the calibration: per sample entry, each group\'s outcome, the proposed reading and the ' +
    'sentence of the standard or the lifecycle it rests on, quoted, or the two readings the text ' +
    'admits; and the drafts the verifiers judged again under a proposed reading;',
  '3. the findings, grouped by the outcome the cross-check proposes, those applied in the branch by ' +
    'anchor and file: per finding, the entry, its outcome and the condition or exit it rests on, ' +
    'the quoted evidence, the edit before and after, for a kept entry its horizon line and the ' +
    'command that counts it, the verifier verdict and note, and old or new text for a re-check;',
  '4. the overlap map and the stale texts;',
  '5. the deletions applied in the branch, in one group per outcome, converted, absorbed, removed, ' +
    'its decision is gone, horizon passed and fired, and the removals in one group per condition ' +
    'failed, 1, 2 and 3; each group with its entries, each entry with the words it turns on and ' +
    'the texts its deletion leaves stale; then the draft of the owner list, its labels left for ' +
    'the session to number: the removals on the doubt\'s weight, as one group with its entries and ' +
    'one default, with its reason, and the entries the default excepts, if any, and every other ' +
    'item the outcome table puts to the owner, grouped by kind, each with a default;',
  '6. what the run did not decide: the plausible verdicts, the firings for the session to judge, ' +
    'the notes the drafts give of a head false of the code or failing a rule on heads, the "Met ' +
    'outside the task" notes of the returns, and every step that failed or returned nothing' +
    (failures.length ? ', among them: ' + failures.join('; ') : '') + '.',
  'Return its path.',
]), { schema: PATH, phase: 'report', label: 'report' })

return {
  report: report ? report.path : null,
  groups: groups.map((g) => g.label),
  verified: verdicts.length,
  gaps,
}
