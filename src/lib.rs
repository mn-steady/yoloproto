use leptos::*;

#[component]
pub fn App(cx: Scope) -> impl IntoView {
    view! {
        cx,
        <div class="top-bar">
            <span class="logo">"YoloProto"</span>
            <button class="connect-wallet" on:click=move |_| connectKeplrWallet()>
                "Connect Wallet"
            </button>
        </div>
    }
}

// Import the connectKeplrWallet function using wasm_bindgen
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/static/wallet.js")]
extern "C" {
    #[wasm_bindgen]
    fn connectKeplrWallet();
}

#[wasm_bindgen(start)]
pub fn start() {
    mount_to_body(|cx| view! { cx, <App /> });
}
