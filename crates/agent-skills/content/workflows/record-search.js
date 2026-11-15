export const meta = {
  name: 'knowledge-architect-record-search',
  description: 'The record search, from the work list to a passing check of every report; the session edits nothing in the tree until it returns',
  phases: [{ title: 'worklist' }, { title: 'search' }, { title: 'verify' }],
}
// args: {
//   scratch     the directory `worklist --out` writes into: a new directory, outside the project
//   work        the files of the work, each one --work
//   subjects    optional: the subjects of the work, each one --subject; none gives the one subject
//               whole-work
//   seeds       optional: the seeds, each one --seed
//   since       optional, at a design audit: the commit, --since
//   seedsFrom   optional, at a design audit: the plan document's README.md, --seeds-from
//   bound       optional: --bound
//   maxRounds   optional: the runs of `worklist --verify`, at least 1; 3 where absent
// }
const isList = (x) => Array.isArray(x) && x.every((v) => typeof v === 'string' && v !== '')
if (!(typeof args.scratch === 'string' && args.scratch !== '')) throw new Error('scratch: the directory `worklist --out` writes into')
if (!(isList(args.work) && args.work.length)) throw new Error('work: at least one file of the work')
if (args.subjects !== undefined && !isList(args.subjects)) throw new Error('subjects: a list of ids')
if (args.seeds !== undefined && !isList(args.seeds)) throw new Error('seeds: a list of references')
if (args.since !== undefined && typeof args.since !== 'string') throw new Error('since: a commit')
if (args.seedsFrom !== undefined && typeof args.seedsFrom !== 'string') throw new Error('seedsFrom: a path')
if (args.bound !== undefined && !Number.isInteger(args.bound)) throw new Error('bound: an integer')
const maxRounds = args.maxRounds === undefined ? 3 : args.maxRounds
if (!(Number.isInteger(maxRounds) && maxRounds >= 1)) throw new Error('maxRounds: an integer, at least 1')

const SEARCHER = 'knowledge-architect-record-searcher'
const at = (...parts) => parts.join('/')
// The last name agent-<n> in a directory or a file path an agent returns, or null where it holds
// none.
const agentName = (s) => {
  const all = [...String(s).matchAll(/(?:^|\/)(agent-[0-9]+)(?=\/|$)/g)]
  return all.length ? all[all.length - 1][1] : null
}
const lines = (xs) => xs.filter(Boolean).join('\n')

%% The two functions below are pure, and `path@agent-skills@src/workflow_parse.rs` reads each from
%% its `function <name>(` line to the next line that is a lone `}` and runs it under `node`: keep
%% each in that form, and keep `worklistCommand` calling nothing outside the two. The flag builder
%% is a function of its own because its failure is silent: a dropped or misquoted seed weakens the
%% search, and nothing reports it.
// A value single-quoted for the shell, an inner ' written '\'', so that a backticked seed or a path
// holding a space or a quote reaches the command as one word, and runs no substitution.
function quoted(value) {
  return "'" + String(value).replace(/'/g, "'\\''") + "'"
}

function worklistCommand(a) {
  const line = ['{{command}} worklist --out', quoted(a.scratch)]
  for (const w of a.work) line.push('--work', quoted(w))
  for (const s of a.subjects || []) line.push('--subject', quoted(s))
  for (const s of a.seeds || []) line.push('--seed', quoted(s))
  if (a.since !== undefined) line.push('--since', quoted(a.since))
  if (a.seedsFrom !== undefined) line.push('--seeds-from', quoted(a.seedsFrom))
  if (a.bound !== undefined) line.push('--bound', quoted(a.bound))
  return line.join(' ')
}

// The work list: one agent runs `worklist --out`, since a script runs no command, and returns the
// agent directories it wrote and the notices it printed on stdout, which it writes into no file.
phase('worklist')
const LISTED = {
  type: 'object',
  properties: {
    agents: { type: 'array', items: { type: 'string' } },
    notices: { type: 'string' },
    refusal: { type: 'string' },
  },
}
const listed = await agent(lines([
  'You run the command that writes the work list of a record search. From the project root, run ' +
    'this command once, exactly as written:',
  worklistCommand(args),
  'Where it exits 0: return agents, the name of each directory agent-<n> it wrote under ' +
    args.scratch + ', and notices, every line it printed on stdout, verbatim.',
  'Where it exits with any other code: return refusal, every line it printed on stderr, ' +
    'verbatim, then a line exit: <code>.',
  'Edit nothing, and run no other command but a listing of ' + args.scratch + '.',
]), { effort: 'low', schema: LISTED, phase: 'worklist', label: 'worklist' })
if (!listed) throw new Error('the worklist step returned nothing')
if (listed.refusal) return { refusal: listed.refusal }
const names = (listed.agents || []).map(agentName)
if (!names.length || names.some((n) => !n)) {
  throw new Error('the worklist step returned no agent directory agent-<n>: ' + JSON.stringify(listed.agents))
}
const notices = listed.notices || ''
log(names.length + ' agent directories')

const REPORT = {
  type: 'object',
  properties: { written: { type: 'boolean' }, reportText: { type: 'string' } },
  required: ['written'],
}
const DONE = { type: 'object', properties: { done: { type: 'boolean' } }, required: ['done'] }
const searcherBrief = (name, findings) => lines([
  'Your agent directory, which is your scratch directory: ' + at(args.scratch, name),
  'The work list: ' + at(args.scratch, 'work.txt'),
  findings ? 'Your report failed the check of every report, {{command}} worklist --verify, with ' +
    'these findings. Repair each, and run the check of your own report again before you return:\n' +
    findings : '',
  'Return written true once your report.md is filled and its check passes; or written false, with ' +
    'the whole filled report as reportText, where the harness refused every write of it.',
])
const search = (name, findings, label) => agent(searcherBrief(name, findings),
  { agentType: SEARCHER, schema: REPORT, phase: findings ? 'verify' : 'search', label })

// A report returned as text is written into its report.md by an agent of its own, so that no one
// prompt holds every text report, through a shell redirect, since the harness can refuse a
// subagent's file tool on a file named report.md. The delimiter is a line the text does not hold.
const delimiter = (text) => {
  let d = 'END_OF_REPORT'
  while (text.split('\n').includes(d)) d += '_'
  return d
}
const writeText = (name, r) => {
  if (!r || r.written || !r.reportText) return r
  const d = delimiter(r.reportText)
  return agent(lines([
    'You write one report file, verbatim. From the project root, run this command once, exactly as ' +
      'written, in one shell call, then edit nothing else:',
    'cat > ' + quoted(at(args.scratch, name, 'report.md')) + " <<'" + d + "'\n" + r.reportText + '\n' + d,
    'Return done true once the command succeeded.',
  ]), { effort: 'low', schema: DONE, phase: 'verify', label: 'write ' + name })
}
// A searcher that returns nothing leaves its report unfilled, which the check fails, so it is sent
// again with the others.
phase('search')
await pipeline(names, (name) => search(name, null, name), (r, name) => writeText(name, r))

// No step repairs a report: choosing a verdict or an action is the searcher's grading.
phase('verify')
const VERIFIED = {
  type: 'object',
  properties: {
    passed: { type: 'boolean' },
    output: { type: 'string' },
    failing: {
      type: 'array',
      items: {
        type: 'object',
        properties: { agent: { type: 'string' }, findings: { type: 'string' } },
        required: ['agent', 'findings'],
      },
    },
  },
  required: ['passed', 'output', 'failing'],
}
for (let round = 1; ; round++) {
  const checked = await agent(lines([
    'You run the check of every report of a record search. From the project root, run this command ' +
      'once, exactly as written:',
    '{{command}} worklist --verify ' + quoted(args.scratch),
    'Return passed, true where it exits 0 and false otherwise; output, every line it printed on ' +
      'stdout and on stderr, verbatim; and failing, one item per agent whose report a line of that ' +
      'output names, by a path that holds agent-<n>/report.md, whatever comes before it: agent, its ' +
      'name agent-<n>, and findings, each such line, verbatim, one per line.',
    'Repair no report, and edit nothing.',
  ]), { effort: 'low', schema: VERIFIED, phase: 'verify', label: 'verify ' + round })
  if (!checked) throw new Error('the check of the reports returned nothing, at its run ' + round)
  if (checked.passed) break
  // Each agent once, with all its findings: those the agent returned, and each line of the output
  // that names its report, so that a line the agent left out, such as the error of a line the check
  // cannot read, still sends the report back. An agent the worklist step did not return is kept,
  // since its directory holds a report nobody filled.
  const byAgent = new Map()
  const add = (name, text) => {
    if (!name) return
    if (!byAgent.has(name)) byAgent.set(name, [])
    for (const l of String(text).split('\n')) if (l && !byAgent.get(name).includes(l)) byAgent.get(name).push(l)
  }
  for (const f of checked.failing) add(agentName(f.agent), f.findings)
  for (const l of String(checked.output).split('\n')) {
    const m = /(agent-[0-9]+)\/report\.md/.exec(l)
    if (m) add(m[1], l)
  }
  const failing = [...byAgent].map(([agent, lines]) => ({ agent, findings: lines.join('\n') }))
  // A failing check that names no report to send back, such as one whose work list it could not
  // read, ends the loop.
  if (round >= maxRounds || !failing.length) {
    log('the check still fails after ' + round + ' runs')
    return { failing: checked.failing, verifyOutput: checked.output, notices }
  }
  log('run ' + round + ': ' + failing.length + ' reports fail, sent back')
  await pipeline(failing,
    (f) => search(f.agent, f.findings, f.agent + ' again ' + round),
    (r, f) => writeText(f.agent, r))
}

const subjects = args.subjects && args.subjects.length ? [...new Set(args.subjects)] : ['whole-work']
const outputs = { notices, verdictFiles: subjects.map((s) => at(args.scratch, 'verdicts-' + s + '.md')) }
if (args.seedsFrom !== undefined) {
  outputs.gaps = at(args.scratch, 'gaps.md')
  outputs.fitLines = at(args.scratch, 'fit-lines.md')
} else {
  outputs.generatedLines = at(args.scratch, 'generated-lines.md')
}
outputs.closingSections = at(args.scratch, 'closing-sections.md')
outputs.statement = 'The searchers read the tree as it stood when each read it: the session edits ' +
  'nothing in the tree until the workflow returns, since an edit made meanwhile is read by some ' +
  'verdicts and missed by others.'
return outputs
