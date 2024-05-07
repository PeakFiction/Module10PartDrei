
use web_sys::HtmlInputElement;
use yew::functional::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;
use crate::User;

#[function_component(Login)]
pub fn login() -> Html {
    let username = use_state(|| String::new());
    let user = use_context::<User>().expect("No context found.");

    let oninput = {
        let current_username = username.clone();

        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            current_username.set(input.value());
        })
    };

    let onclick = {
        let username = username.clone();
        let user = user.clone();
        Callback::from(move |_| *user.username.borrow_mut() = (*username).clone())
    };

    html! {
        <div class="bg-[#0d0d0d] flex w-screen h-screen items-center justify-center">
            <div class="flex flex-col items-center">
                <div class="text-3xl text-[#1bff80] mb-8">{"DATA RELAY NETWORK"}</div>
                <form class="flex flex-col items-center">
                    <input {oninput} class="rounded-lg p-4 border text-[#1bff80] border-[#1bff80] bg-[#0d0d0d] mb-4 text-center" placeholder="USERNAME" />
                    <Link<Route> to={Route::Chat}>
                        <button {onclick} disabled={username.len()<1} class="px-8 rounded-lg bg-[#1bff80] text-black font-bold p-4 uppercase">{"ENTER"}</button>
                    </Link<Route>>
                </form>
            </div>
        </div>
    }
}