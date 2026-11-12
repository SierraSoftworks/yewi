use yew::prelude::*;
use yewi::prelude::*;

const SAMPLES: &[(&str, &str)] = &[
    ("rust", "#[derive(Debug, Clone)]\npub struct Rollout<'a> {\n    job: &'a str,\n    healthy: u32, // of `count`\n}\n\nimpl Rollout<'_> {\n    pub fn done(&self, count: u32) -> bool {\n        println!(\"{}: {}/{count}\", self.job, self.healthy);\n        self.healthy >= count && count != 0\n    }\n}"),
    ("typescript", "interface Rollout {\n  readonly job: string;\n  healthy: number; // of `count`\n}\n\nexport async function wait(rollout: Rollout, count = 4): Promise<boolean> {\n  const response = await fetch(`/v1/jobs/${rollout.job}`);\n  return response.ok && rollout.healthy >= count;\n}"),
    ("javascript", "import { status } from \"./api.js\";\n\n/* Poll until every allocation is healthy. */\nexport function watch(job, onDone) {\n  const timer = setInterval(async () => {\n    const { healthy, count } = await status(job);\n    if (healthy === count) clearInterval(timer), onDone(null);\n  }, 2_000);\n}"),
    ("python", "from dataclasses import dataclass\n\n@dataclass\nclass Rollout:\n    \"\"\"One job being rolled out.\"\"\"\n    job: str\n    healthy: int = 0\n\n    def done(self, count: int) -> bool:\n        print(f\"{self.job}: {self.healthy}/{count}\")  # progress\n        return self.healthy >= count and count != 0"),
    ("go", "package rollout\n\nimport \"fmt\"\n\n// Done reports whether every allocation is healthy.\nfunc (r *Rollout) Done(count int) bool {\n\tfmt.Printf(\"%s: %d/%d\\n\", r.Job, r.Healthy, count)\n\treturn r.Healthy >= count && count != 0\n}"),
    ("sql", "SELECT job, count(*) AS healthy\nFROM allocations\nWHERE status = 'running' AND started_at > now() - interval '5 minutes'\nGROUP BY job\nHAVING count(*) >= 4 -- the whole group\nORDER BY job;"),
    ("bash", "#!/usr/bin/env bash\nset -euo pipefail\n\nexport NOMAD_ADDR=\"https://nomad.example.com\"\nfor region in fra ams; do\n  if ! nomad job run -var \"region=$region\" docs.hcl; then\n    echo \"rollout failed in ${region}\" >&2 # keep going\n  fi\ndone"),
    ("hcl", "job \"docs\" {\n  datacenters = [\"fra\", \"ams\"]\n\n  group \"web\" {\n    count = 4 # one per node\n\n    task \"caddy\" {\n      driver = \"docker\"\n      config {\n        image = \"caddy:2.8-alpine\"\n      }\n    }\n  }\n}"),
    ("toml", "[package]\nname = \"docs\"\nversion = \"1.4.2\"\nedition = \"2024\"\n\n[dependencies]\n# The design system.\nyewi = { version = \"0.1\", features = [\"csr\", \"code\"] }\n\n[profile.release]\nopt-level = \"z\"\nlto = true"),
    ("yaml", "# Local preview of the docs site.\nservices:\n  docs:\n    image: ghcr.io/sierrasoftworks/docs:1.4.2\n    ports: [\"8080:80\"]\n    environment:\n      LOG_LEVEL: info\n      WORKERS: 4\n    read_only: true"),
    ("json", "{\n  \"job\": \"docs\",\n  \"healthy\": true,\n  \"allocations\": 4,\n  \"regions\": [\"fra\", \"ams\"],\n  \"previous\": null\n}"),
    ("html", "<!doctype html>\n<html lang=\"en\">\n  <head>\n    <!-- Set the theme before first paint. -->\n    <style>:root { --yi-accent: #409eff; }</style>\n    <script>document.documentElement.dataset.theme = \"dark\";</script>\n  </head>\n  <body>\n    <a href=\"/docs\" class=\"link\">Docs &amp; guides</a>\n  </body>\n</html>"),
    ("css", "@layer app {\n  .card:hover {\n    --lift: 2px; /* how far it rises */\n    transform: translateY(calc(-1 * var(--lift)));\n    border: 1px solid #d0d7de;\n    font-family: \"Inter\", sans-serif;\n  }\n}"),
    ("diff", "--- a/docs.hcl\n+++ b/docs.hcl\n@@ -4,3 +4,3 @@\n   group \"web\" {\n-    count = 2\n+    count = 4\n "),
];

#[function_component]
pub fn Example() -> Html {
    html! {
        <Grid min="24rem">
            { for SAMPLES.iter().map(|(language, code)| html! {
                <CodeBlock code={*code} language={*language} copy=false />
            }) }
        </Grid>
    }
}
