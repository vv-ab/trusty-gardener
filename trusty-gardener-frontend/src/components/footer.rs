use leptos::*;

#[component]
pub fn Footer(cx: Scope) -> impl IntoView {
    view! { cx,
        <footer class="footer mt-auto">
            <div class="content">
            <a href="https://media.tenor.com/JHePvU1xhFgAAAAM/pump-crypto.gif">{"Hilfe"}</a>
            <br/>
            <a href="https://images.wagwalkingweb.com/media/daily_wag/behavior_guides/hero/1629934048.5675797/he-gong-kbuycu1swik-unsplash.jpg">{"Privatsphäre"}</a>
            <br/>
            <a href="https://media.tenor.com/YGWxkk8hm7UAAAAd/cat-telephone-cat.gif">{"Kontakt"}</a>
            <br/>
            <a href="https://encrypted-tbn0.gstatic.com/images?q=tbn:ANd9GcRAfL6JExBpyNxNEIpTcxhYVySM3yOmopdNpUqnL8_eIgXLvtXChBfBvoDrs7u8_E3nhBs&usqp=CAU">{"Bedingungen"}</a>
            <br/>
            </div>
        </footer>
    }
}