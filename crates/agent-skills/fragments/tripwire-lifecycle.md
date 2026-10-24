A tripwire leaves its home when:

- **it fires**: its heading and body are deleted, and an issue is opened or the decision is
  reopened, as its response says. A fired tripwire never stays listed;
- **its horizon passes unfired**: it is deleted, and the commit that deletes it names the
  occasions the decision held through. A `rejected` entry kept because a doubt remained, and naming
  this tripwire, is then judged again by the recording tests, and leaves if none holds;
- **its decision is reversed**: it is deleted outright;
- **another entry guards the same premise**: its clause is folded into that entry, since a premise
  guarded from two places is voided by half when its decision is reversed;
- **a firing's repair adds a test or a check that now catches the failure**: the failure is no
  longer silent, and the check carries it.

A `standing` tripwire that fires is restated, naming the class rather than the instance, only while
its failure stays silent after the repair; the instance is the issue. A firing whose only response
would be to change the tripwire's own bound shows that the bound had no basis: rewrite the clause
to name the event, or a bound with its basis.
