use yew::prelude::*;
use yewi::prelude::*;

const BEFORE: &str = r#"job "caddy" {
  datacenters = ["home"]
  type        = "service"

  group "web" {
    count = 1

    network {
      port "http" {
        static = 80
      }
    }

    task "caddy" {
      driver = "docker"

      config {
        image = "caddy:2.7"
        ports = ["http"]
      }

      resources {
        cpu    = 100
        memory = 64
      }
    }
  }
}
"#;

const AFTER: &str = r#"job "caddy" {
  datacenters = ["home"]
  type        = "service"

  group "web" {
    count = 2

    network {
      port "http" {
        static = 80
      }
      port "https" {
        static = 443
      }
    }

    task "caddy" {
      driver = "docker"

      config {
        image = "caddy:2.8"
        ports = ["http", "https"]
      }

      resources {
        cpu    = 100
        memory = 64
      }
    }
  }
}
"#;

#[function_component]
pub fn Example() -> Html {
    html! { <Diff title="caddy.nomad.hcl" before={BEFORE} after={AFTER} context={2} /> }
}
