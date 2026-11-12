use yew::prelude::*;
use yewi::prelude::*;

const PATCH: &str = r#"--- a/src/retry.rs
+++ b/src/retry.rs
@@ -12,7 +12,8 @@ impl Retry {
     pub fn delay(&self, attempt: u32) -> Duration {
-        self.base * 2_u32.pow(attempt)
+        let backoff = self.base * 2_u32.pow(attempt.min(8));
+        backoff.min(self.max)
     }
 
     pub fn exhausted(&self, attempt: u32) -> bool {
         attempt >= self.limit
     }
@@ -41,5 +42,5 @@ mod tests {
     fn delay_doubles() {
         let retry = Retry::new(Duration::from_millis(100));
-        assert_eq!(retry.delay(3), Duration::from_millis(800));
+        assert_eq!(retry.delay(3).as_millis(), 800);
     }
 }
\ No newline at end of file
"#;

#[function_component]
pub fn Example() -> Html {
    html! { <Diff unified={PATCH} /> }
}
