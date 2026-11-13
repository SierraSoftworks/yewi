use yewi_test::render;

use super::*;
use crate::attrs;

/// The opening tag of each element with `role="<role>"`.
fn with_role<'a>(html: &'a str, role: &str) -> Vec<&'a str> {
    let marker = format!(r#"role="{role}""#);
    html.split('<').filter(|tag| tag.split('>').next().unwrap().contains(&marker)).collect()
}

fn account(props: TabsProps) -> String {
    render(move || {
        html! {
            <Tabs
                label={props.label.clone()}
                selected={props.selected.clone()}
                default_selected={props.default_selected.clone()}
                keep_mounted={props.keep_mounted}
            >
                <Tab value="profile" label="Profile">{ "Profile panel" }</Tab>
                <Tab value="devices" label="Devices" count="3">{ "Devices panel" }</Tab>
                <Tab value="billing" label="Billing" disabled=true>{ "Billing panel" }</Tab>
            </Tabs>
        }
    })
}

#[test]
fn renders_a_named_tab_list_with_one_tab_per_child() {
    let html = account(yew::props!(TabsProps { label: "Account" }));
    assert!(with_role(&html, "tablist")[0].contains(r#"aria-label="Account""#));
    let tabs = with_role(&html, "tab");
    assert_eq!(tabs.len(), 3);
    assert!(html.contains("Devices") && html.contains('3'));
    assert!(tabs[2].contains(" disabled"));
}

#[test]
fn the_first_enabled_tab_is_selected_and_only_its_panel_renders() {
    let html = account(yew::props!(TabsProps {}));
    let tabs = with_role(&html, "tab");
    assert!(tabs[0].contains(r#"aria-selected="true""#) && tabs[0].contains(r#"tabindex="0""#));
    assert!(tabs[1].contains(r#"aria-selected="false""#) && tabs[1].contains(r#"tabindex="-1""#));
    assert_eq!(with_role(&html, "tabpanel").len(), 1);
    assert!(html.contains("Profile panel") && !html.contains("Devices panel"));
}

#[test]
fn selection_can_be_set_by_default_or_controlled() {
    for props in [
        yew::props!(TabsProps { default_selected: "devices" }),
        yew::props!(TabsProps { selected: "devices" }),
        yew::props!(TabsProps { selected: "devices", default_selected: "profile" }),
    ] {
        let html = account(props);
        assert!(with_role(&html, "tab")[1].contains(r#"aria-selected="true""#));
        assert!(html.contains("Devices panel") && !html.contains("Profile panel"));
    }
}

#[test]
fn an_unknown_selection_falls_back_to_the_first_enabled_tab() {
    let html = account(yew::props!(TabsProps { selected: "nope" }));
    assert!(with_role(&html, "tab")[0].contains(r#"aria-selected="true""#));
}

#[test]
fn the_selected_tab_and_its_panel_reference_each_other() {
    let html = account(yew::props!(TabsProps {}));
    let attr = |tag: &str, name: &str| {
        let rest = tag.split(&format!(r#"{name}=""#)).nth(1).unwrap_or_else(|| panic!("no {name} in {tag}"));
        rest.split('"').next().unwrap().to_owned()
    };
    let (tab, panel) = (with_role(&html, "tab")[0], with_role(&html, "tabpanel")[0]);
    assert_eq!(attr(tab, "aria-controls"), attr(panel, " id"));
    assert_eq!(attr(panel, "aria-labelledby"), attr(tab, " id"));
}

#[test]
fn keep_mounted_renders_every_panel_and_hides_the_rest() {
    let html = account(yew::props!(TabsProps { keep_mounted: true }));
    let panels = with_role(&html, "tabpanel");
    assert_eq!(panels.len(), 3);
    assert_eq!(panels.iter().filter(|panel| panel.contains(" hidden")).count(), 2);
}

#[test]
fn every_variant_is_distinguishable() {
    let [line, pill] = TabsVariant::ALL.map(|variant| {
        render(move || html! { <Tabs {variant}><Tab value="a" label="A" /></Tabs> })
    });
    assert_ne!(line, pill);
}

#[test]
fn tab_nav_is_a_landmark_of_links_with_the_current_page_marked() {
    let html = render(|| {
        html! {
            <TabNav label="Settings">
                <TabLink href="/settings/profile" current=true>{ "Profile" }</TabLink>
                <TabLink href="/settings/tokens" count="2">{ "Tokens" }</TabLink>
            </TabNav>
        }
    });
    assert!(html.starts_with("<nav") && html.contains(r#"aria-label="Settings""#));
    assert!(html.contains(r#"href="/settings/profile""#) && html.contains(r#"href="/settings/tokens""#));
    assert_eq!(html.matches(r#"aria-current="page""#).count(), 1);
    assert!(!html.contains("role=\"tab\""));
}

#[test]
fn id_class_and_attrs_reach_the_root_element() {
    let html = render(|| {
        html! {
            <Tabs id="views" class="mine" attrs={attrs! { "data-test" => "x" }}>
                <Tab value="a" label="A" />
            </Tabs>
        }
    });
    let root = &html[..html.find('>').unwrap()];
    assert!(root.contains(r#"id="views""#) && root.contains("mine") && root.contains(r#"data-test="x""#));
}

#[test]
fn without_a_client_every_panel_is_in_the_page_and_all_but_one_hidden() {
    let html = render(|| {
        html! {
            <crate::ServerOnly script="/yewi.js">
                <Tabs label="Account">
                    <Tab value="a" label="A">{ "First panel" }</Tab>
                    <Tab value="b" label="B">{ "Second panel" }</Tab>
                </Tabs>
            </crate::ServerOnly>
        }
    });
    assert!(html.contains("data-yi-tabs"));
    assert!(html.contains("First panel") && html.contains("Second panel"));
    assert_eq!(html.matches(r#"role="tabpanel""#).count(), 2);
    assert_eq!(html.matches(" hidden").count(), 1);
    assert_eq!(html.matches("aria-controls").count(), 2);
}
