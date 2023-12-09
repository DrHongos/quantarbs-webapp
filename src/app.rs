use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::date_picker::DatePicker;

#[component]
pub fn App() -> impl IntoView {
    // Provides context that manages stylesheets, titles, meta tags, etc.
    provide_meta_context();
    view! {
        // injects a stylesheet into the document <head>
        // id=leptos means cargo-leptos will hot-reload this stylesheet
        <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

        // sets the document title
        <Title text="Quantarbs"/>

        // content for this welcome page
        <Router>
            <main>
                <Routes>
                    <Route path="" view=HomePage/>
                    <Route path="/*any" view=NotFound/>
                </Routes>
            </main>
        </Router>
    }
}

async fn fetch_tokens() -> Vec<String> {
    let data = reqwest::get("http://opinologos.xyz:8080/get_tokens")
        .await
        .expect("Error calling api: tokens")
        .json::<Vec<String>>()
        .await
        .expect("Error parsing: tokens");

//    logging::log!("data is {:#?}", data);
    data
}

/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
/* 
 to call /get_range_close
#[derive(Debug, Deserialize, Serialize)]
pub struct PairRange {
    pub name: String,
    pub start: String,      // 'YYYY-MM-DD HH:MM:DD'
    pub end: String
}

*/

    // move configs to a outter component 
    // select a subset of tokens
    // launch analysis
    // download results

    let (correlation_factor, set_correlation_factor) = create_signal(0.7);
    let (days_execution, set_days_execution) = create_signal(30);
    let (name, set_name) = create_signal("test".to_string()); 
    let (start, set_start) = create_signal(String::new());        
    let (end, set_end) = create_signal(String::new());        

    let (token_selected, set_token_selected) = create_signal::<Option<String>>(None);

    let tokens =
        create_local_resource(move || (), |_| { fetch_tokens() });
    
    // create function to query get_range_close
    pub fn simplify(d: String) -> String {
        format!("{} 00:00:00", d)           // parsed for server query
    }

    view! {
        <h1>"Quantarbs"</h1>
        <hr />
        <p>"Configuration of analysis"</p>
        <label>"Study name"</label>
        <input type="text"
            on:input=move |ev| {
                set_name(event_target_value(&ev));
            }
            prop:value=name
        /><br />
        <label>"Start"</label>
        <DatePicker 
            setter=set_start
        />
        <br/>
        <label>"End"</label>
        <DatePicker
            setter=set_end
         />
        <br />
        <p>"Selected: "{move || start.get()}" to "{move || end.get()}</p>
        <br />
        <label>"Correlation factor"</label> 
        <input type="float"
        on:input=move |ev| {
            set_correlation_factor(event_target_value(&ev).parse::<f64>().unwrap());
        }
        prop:value=correlation_factor
        />
        <br />
        <label>"Days of trading"</label>
        <input type="number"
            on:input=move |ev| {
                set_days_execution(event_target_value(&ev).parse::<i32>().unwrap());
            }
            prop:value=days_execution
        />
        <hr />
        <p>"Tokens"</p>
        {move || match tokens.get() {
            None => view! { <p>"Loading..."</p> }.into_view(),
            Some(data) => {
                let tokens_view = data.into_iter().map(|t| 
                    {
                    let c = t.clone();
                    view!{
                        <button 
                        on:click= move |_| {
//                            logging::log!("Passing {}", &c);
                            set_token_selected.set(Some(c.clone()))
                        }
                        class={"token"}>
                        {t}
                    </button>
                    }}).collect_view();
                view! { <div>{tokens_view}</div> }.into_view()
            }
        }}
        <hr />

        {move || match token_selected.get() {
            None => view! { <p>"Select a token pair"</p> }.into_view(),
            Some(selected) => {
                view! {
                    <>
                        <p>{selected}</p>
                        <button class="fetcher">"Get data"</button>
                    </>
                }.into_view()
            }
        }}
    }
}

/// 404 - Not Found
#[component]
fn NotFound() -> impl IntoView {
    // set an HTTP status code 404
    // this is feature gated because it can only be done during
    // initial server-side rendering
    // if you navigate to the 404 page subsequently, the status
    // code will not be set because there is not a new HTTP request
    // to the server
    #[cfg(feature = "ssr")]
    {
        // this can be done inline because it's synchronous
        // if it were async, we'd use a server function
        let resp = expect_context::<leptos_actix::ResponseOptions>();
        resp.set_status(actix_web::http::StatusCode::NOT_FOUND);
    }

    view! {
        <h1>"Not Found"</h1>
    }
}
