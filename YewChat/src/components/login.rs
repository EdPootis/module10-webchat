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
        <div class="bg-indigo-900 flex w-screen">
            <div class="container mx-auto flex flex-col justify-center items-center">
                <h1 class="text-3xl font-bold text-white mb-6 mt-10">{"Welcome to YewChat"}</h1>
                <p class="text-indigo-200 mb-8 text-center max-w-md">
                    {"Join the conversation! Enter your username below to start chatting with others."}
                </p>
                <form class="m-4 flex shadow-lg">
                    <input 
                        {oninput} 
                        class="rounded-l-lg p-4 border-t mr-0 border-b border-l text-gray-800 border-indigo-300 bg-white focus:outline-none focus:ring-2 focus:ring-indigo-400" 
                        placeholder="Enter your username"
                    />
                    <Link<Route> to={Route::Chat}>
                        <button 
                            {onclick} 
                            disabled={username.len()<1} 
                            class="px-8 rounded-r-lg bg-emerald-600 text-white font-bold p-4 uppercase border-emerald-600 border-t border-b border-r transition-colors hover:bg-emerald-700 disabled:bg-gray-400 disabled:border-gray-400"
                        >
                            {"Start Chatting"}
                        </button>
                    </Link<Route>>
                </form>
                <div class="mt-6 bg-indigo-800 p-4 rounded-lg text-indigo-200 max-w-md">
                    <p class="text-sm">{"Tips: Choose a unique username to stand out in the chat!"}</p>
                </div>
            </div>
        </div>
    }
}