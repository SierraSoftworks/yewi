crate::page! {
    title: "Pipeline",
    category: Data,
    summary: "A pipeline shows ordered stages side by side, each with its own content. A gate in front of a stage shows where the work has to wait.",
    examples: [
        basic => "Stages": "A stage has a title, a short `status`, some quiet `meta` and any content you like. Mark the stage that's in progress with `current`. Stages wrap onto more rows, and they stack when the container is narrower than 34rem.",
        gates => "Gates": "Pass a `PipelineGate` to the stage it guards. `Waiting` means it hasn't been reached yet, `Closed` means it's holding the pipeline, and `Open` means it's been passed. Use the gate's label to say what it's waiting for, and put the action that opens it in the stage.",
        rollout => "Units of work": "`PipelineUnit` draws one block per host, job or shard, so you can see a stage's progress at a glance. The tone carries the state, and `fill` tells apart states that share a tone. Always pair the blocks with a legend. `placeholder` marks a stage that's outside the run.",
        compact => "Compact": "`compact` shrinks the stages and units and drops the `meta`. It suits an overview that links to the full pipeline. `fill` shares the width equally between stages.",
    ],
}
