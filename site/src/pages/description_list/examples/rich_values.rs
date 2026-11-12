use yew::prelude::*;
use yewi::prelude::*;

#[function_component]
pub fn Example() -> Html {
    html! {
        <DescriptionList attrs={attrs! { "aria-label" => "Certificate" }}>
            <DescriptionItem term="Host names">
                <Stack gap={Space::Xxs}>
                    <span>{ "tak.example.org" }</span>
                    <span>{ "*.tak.example.org" }</span>
                </Stack>
            </DescriptionItem>
            <DescriptionItem term="Status">
                <Cluster gap={Space::Xs} align={Align::Center}>
                    <Icon icon={icons::CIRCLE_CHECK} attrs={attrs! { "style" => "color: var(--yi-ok)" }} />
                    { "Valid for 61 days" }
                </Cluster>
            </DescriptionItem>
            <DescriptionItem term="Issuer">
                <a href="https://letsencrypt.org">{ "Let's Encrypt" }</a>
            </DescriptionItem>
            <DescriptionItem term="Certificate file" mono=true>{ "/etc/rustak/tls/fullchain.pem" }</DescriptionItem>
            <DescriptionItem term="Fingerprint" mono=true>
                { "9f:91:07:c3:fb:b3:1e:e7:5c:07:81:3b:c4:98:ee:2d:4a:11:0c:6e" }
            </DescriptionItem>
            <DescriptionItem term="Last attempt" />
        </DescriptionList>
    }
}
