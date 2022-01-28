# Dependability: A library for making your programs more reliable

Especially when dealing with hardware, a lot can go wrong. Here are some of the biggest problems that can arise in code and what you can do about them:

- hardware fails or "external" components fail (network, IO)
  - handle similar to faulty software
  - additionally: retry (because the behavior isn't deterministic)
- software faulty
  - produces wrong value -> see environment fails
  - takes too long -> deadlines
  - uses too many resources -> limit resources
- environment "fails", e.g. unexpected (sensor) inputs, where normal handling is not appropriate
  -> detect anomalies
  -> redundancy

## Usage

This project exposes four crates in total.

### dependability

In most cases, you want to use this crate. It has the features `retry` and `runtime`
which can be enabled to re-export the respective crates.

### retry and retry_proc_macro

These crates are used to retry fallible operations. The `retry!` macro can be used
to retry expressions, the `#[retry]` proc macro will automatically retry the annotated
function. `retry` re-exports `retry_proc_macro`.

### runtime

This is an async runtime that respects deadlines. It works on embedded systems and
behavior upon exceeding a deadline can be configured.

## Todos

- [ ] provide waiting future
- [ ] add (optional) expected execution times to tasks
- [ ] add priorities for scheduler
- [ ] add fallback functions (service levels that are automatically chosen based on time pressure)
- [ ] add keyword similar to return, but for intermediary values to be registered
  - [ ] if the future is killed because it didn't meet the deadline, return the last intermediary value
- [ ] provide an ambient executor like Tokio
- [ ] offline scheduling of repeated tasks?

## Benchmarking todos

Measure:

- reliability (for failures that just happen naturally and intentionally introduced failures)
- performance overhead of using this library

Setups:

- [ ] adapt temperature sensor code using dependability library
- [ ] adapt other embedded program using this library

## Acknowledgements

The async runtime is based on the work of [Philipp Oppermann](https://os.phil-opp.com/async-await/) and [Leonard Seibold](https://github.com/zortax).
This project is being developed as part of the [Embedded Operating Systems](https://osm.hpi.de/eos/2021) lecture at Hasso-Plattner-Institut.
