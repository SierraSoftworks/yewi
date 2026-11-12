use yew::prelude::*;
use yewi::prelude::*;

const PASSWORD: &str = "Xk7-mQ2v-9cL1-sT8b";
const ENROL: &str = "tak://com.atakmap.app/enroll?host=tak.example.org&username=field-07&token=Xk7-mQ2v-9cL1-sT8b";

#[function_component]
pub fn Example() -> Html {
    let warning = html! {
        { "Only a hash of this password is stored, so nobody, including an administrator, can show it again. \
           If it is lost, issue a new credential." }
    };
    let aside = html! {
        <QrCode value={ENROL} label="Enrolment link for field-07">{ "Scan with the device to enrol." }</QrCode>
    };

    html! {
        <SecretReveal title="Credential issued for field-07" {warning} {aside} on_dismiss={Callback::noop()}>
            <SecretValue label="Password" value={PASSWORD} concealed=true />
            <SecretValue label="Enrolment link" value={ENROL} />
        </SecretReveal>
    }
}
