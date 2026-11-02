%% The default shape and its force are `design@agent-skills@default-code-shape`; test economy is
%% `design@agent-skills@test-economy`. One fragment, hosted by the development skill and the
%% code-shape reviewer, per `design@agent-skills@shared-text-fragment`.
**The default shape of the code a change writes or edits** holds five properties:

- **One responsibility per unit, read as one reason to change.** A function, a type or a module has
  two reasons to change when two concrete changes someone could plausibly ask for would each touch
  it independently, and the two are entangled in it. A finding of two reasons names both changes,
  shows that each touches the unit on its own, and shows the entanglement. "Does one thing", read
  literally, leads to code where everything is a function, and is not the test.
- **No fragmentation.** A split that buys nothing departs from the shape too: a function with one
  caller, no test of its own and a name that says no more than its body.
- **The logic pure, the side effects in a thin layer** that decides nothing a test of the logic
  should cover: input and output, the file system, the clock, the network, a process.
- **The logic tested without the side effects**, where the change adds or changes logic.
%% The integrated test's property is `design@agent-skills@integrated-test-through-public-api`.
- **An integrated test drives the project through its public API, as a consumer would**, in a mock
  environment or not. Where it cannot observe what it needs in what that API returns, the API hides
  it from consumers too, and the API is what is fixed; where what it would observe has no effect a
  consumer could see, the claim is a unit test's, and an integrated test is the wrong place for it.
  Code that no integrated test can reach and no consumer can observe is suspect: read its comment
  and the commit that added it before removing any of it.

**A departure is argued, where a review reports one.** In code the change writes, the writer
restructures it, or argues it in a comment at the code. In an existing unit the change edits, only
the lines the change writes are weighed; the rest of the unit is a lead, never an obligation.

**Every test the change adds names its claim**, whatever its kind: a test with no claim is cost with
no information. **A test of logic carries a recorded mutation**; for a test of another kind the
writer picks the evidence, and the table says what each form tells:

| test kind | what a mutation tells | the economy question |
| --- | --- | --- |
| pure logic | a recorded mutation, named | does it map to a claim |
| golden or snapshot | a mutation of the code that produces the output: it fails, which tells little | does another golden test already cover the same claim |
| heavy mocks | a mutation of the logic under test, never of the mock | a test that only checks a pass-through to the mock tests the side-effect layer, which needs no test of its own: that is usually the finding |
| integrated, through the public API | a mutation of the logic it drives, owed as by a test of logic where its claim is one of logic | does it observe the claim in what the API returns, or reach for an internal |
| nondeterministic or external | none is reliable | the writer's judgement |

A test's kind is decided by what it exercises: "integrated" names how a test drives the code, and
exempts no test of logic from its mutation.
