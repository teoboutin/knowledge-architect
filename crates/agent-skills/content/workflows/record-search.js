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
const maxRounds = args.maxRounds === undefined ? 3 : args.maxRounds
if (!(Number.isInteger(maxRounds) && maxRounds >= 1)) throw new Error('maxRounds: an integer, at least 1')

const SEARCHER = 'knowledge-architect-record-searcher'
const at = (...parts) => parts.join('/')
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

// The whole command line of `worklist --out`: after the directory, each file of the work, each
// subject, each seed, then since, seedsFrom and bound, each value quoted.
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
const names = (listed.agents || []).map((a) => a.split('/').pop())
if (!names.length || names.some((n) => !/^agent-[0-9]+$/.test(n))) {
  throw new Error('the worklist step returned no agent directory agent-<n>: ' + JSON.stringify(listed.agents))
}
const notices = listed.notices || ''
log(names.length + ' agent directories')

// The searchers: one per agent directory, which is its scratch directory.
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

// The check: one agent runs `worklist --verify`; each searcher whose report fails is sent again
// with its findings, and the check runs again, for at most maxRounds runs. No step repairs a
// report: choosing a verdict or an action is the searcher's grading.
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
    'Return passed, true where it exits 0 and false otherwise; output, every line it printed, ' +
      'verbatim; and failing, one item per agent whose report a finding names, a finding being a line ' +
      'that opens with agent-<n>/report.md: agent, its name agent-<n>, and findings, each such line ' +
      'of it, verbatim, one per line.',
    'Repair no report, and edit nothing.',
  ]), { effort: 'low', schema: VERIFIED, phase: 'verify', label: 'verify ' + round })
  if (!checked) throw new Error('the check of the reports returned nothing, at its run ' + round)
  if (checked.passed) break
  const failing = checked.failing
    .map((f) => ({ agent: f.agent.split('/').pop(), findings: f.findings }))
    .filter((f) => names.includes(f.agent))
  // A failing check with no agent to send back, such as a file it could not read, ends the loop.
  if (round >= maxRounds || !failing.length) {
    log('the check still fails after ' + round + ' runs')
    return { failing: checked.failing, verifyOutput: checked.output, notices }
  }
  log('run ' + round + ': ' + failing.length + ' reports fail, sent back')
  await pipeline(failing,
    (f) => search(f.agent, f.findings, f.agent + ' again ' + round),
    (r, f) => writeText(f.agent, r))
}

const subjects = args.subjects && args.subjects.length ? args.subjects : ['whole-work']
const outputs = { notices, verdictFiles: subjects.map((s) => at(args.scratch, 'verdicts-' + s + '.md')) }
if (args.seedsFrom !== undefined) {
  outputs.gaps = at(args.scratch, 'gaps.md')
  outputs.fitLines = at(args.scratch, 'fit-lines.md')
} else {
  outputs.generatedLines = at(args.scratch, 'generated-lines.md')
}
outputs.closingSections = at(args.scratch, 'closing-sections.md')
outputs.statement = 'The searchers read the tree as it stood while this workflow ran: the session ' +
  'edits nothing in the tree until the workflow returns, so an edit made meanwhile is read by no verdict.'
return outputs
