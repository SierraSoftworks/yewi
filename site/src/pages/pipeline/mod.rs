crate::page! {
    title: "Pipeline",
    category: Data,
    summary: "Ordered stages side by side, with a gate in front of a stage where the work must wait and the stage's own content inside it.",
    examples: [
        basic => "Stages": "A stage has a title, a short `status`, quiet `meta` and any content. Mark the stage in flight with `current`. Stages wrap onto further rows, and stack when the container is narrower than 34rem.",
        gates => "Gates": "Pass a `PipelineGate` to the stage it guards. `Waiting` has not been reached, `Closed` holds the pipeline and `Open` has been passed. Say what the gate waits for in its label, and put the action that opens it in the stage.",
        rollout => "Units of work": "`PipelineUnit` draws one block per host, job or shard, so a stage shows its progress at a glance. Tone carries the state and `fill` separates states that share a tone; always pair the blocks with a legend. `placeholder` marks a stage outside the run.",
        compact => "Compact": "`compact` shrinks stages and units and drops the `meta`, for an overview that links to the full pipeline. `fill` shares the width equally.",
    ],
}
