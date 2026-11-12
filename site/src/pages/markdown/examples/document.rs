use yew::prelude::*;
use yewi::prelude::*;

const SOURCE: &str = r#"# Rolling out a release

A rollout moves the fleet to a new revision **one stage at a time**. Each stage
waits for its hosts to report healthy before the next begins; see the
[stage reference](/reference/stages) for the checks involved.

## Before you start

- [x] The revision passed its build
- [x] A maintenance window is open
- [ ] Someone is on call for the ~~first hour~~ whole rollout

## Starting it

1. Pick the revision to deploy.
2. Choose the stages it should reach.
3. Run the command below, or press *Deploy* in the console.

```bash
# Deploys to staging, then waits for approval.
fleet rollout start --revision 4f2a91c --until staging
```

The job file names the image, so a rollout never guesses at it:

```hcl
job "caddy" {
  count = 3
  image = "caddy:2.8"
}
```

> A rollout that is paused keeps its place. Resume it with `fleet rollout resume`
> and it carries on from the stage it stopped at.

## Stages

| Stage      | Hosts | Approval | Typical duration |
|:-----------|------:|:--------:|-----------------:|
| Canary     |     2 | no       |            4 min |
| Staging    |     9 | no       |           12 min |
| Production |    18 | yes      |           35 min |

---

Questions go to [ops@example.com](mailto:ops@example.com).
"#;

#[function_component]
pub fn Example() -> Html {
    html! { <Markdown source={SOURCE} attrs={attrs! { "style" => "max-inline-size: 44rem" }} /> }
}
