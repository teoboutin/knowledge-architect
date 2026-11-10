# Rejected designs — gates

The designs the Component gates did not keep, each with the reason it was not kept. Each is cited by the
design heads of `register@gates@design` whose argument covers it. An entry stays `live` while its
reason holds, and is marked `void` when it no longer does.

### Capturing a child's two streams through one regular file instead of two pipes `##streams-through-one-file`

`live`. Its one reason was a child that refuses a pipe on stdout, which the checker's `check` did
for a while, per the entry on that refusal, `rejected@check@check-refusing-a-pipe`; no child
refuses one now. Its one gain is that both streams keep the order the child wrote them in. It was
built, with the file in the system temporary directory, opened in append mode for both streams and
read back by polling, and an adversarial review reproduced three losses the pipes do not have:

- a child that reopens its standard output by the device path, /dev/stdout, truncates the capture:
  `sh -c 'echo first-line-that-is-long; echo b > /dev/stdout; echo c'` captured `b` and `c` alone;
- output a descendant writes after the child exits is dropped:
  `sh -c '(sleep 0.3; echo late) & echo early'` captured `early` alone;
- a killed run leaves the file behind, and since the name was the process id and a counter,
  created with `create_new`, a later run that reuses the process id fails every gate.

No gate child did any of the three, so the losses are latent. They are what a reintroduction has to
answer.
