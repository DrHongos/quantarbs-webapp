use leptos::*;
use leptos_meta::*;
use leptos_router::*;
use crate::components::date_picker::DatePicker;
use std::collections::HashMap;
use serde_json::Value;

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
    data
}

#[derive(Debug, Clone)]
pub struct ClosePrices {
    opentime: String,
    close: f64
}

impl From<Value> for ClosePrices {
    fn from(value: Value) -> Self {
        ClosePrices { 
            opentime: value["opentime"].as_str().unwrap().to_owned(), 
            close: value["close"].as_f64().unwrap() 
        }
    }   
}

// get opentime & close price of each token
async fn fetch_token_data(start: String, end: String, name: String) -> Vec<Value> {
        let mut map = HashMap::new();
        map.insert("start", &start);
        map.insert("end", &end);
        map.insert("name", &name);
        // 
        let client = reqwest::Client::new();

        let res: Vec<Value> = client.post("http://opinologos.xyz:8080/get_range_close")
            .json(&map)
            .send()
            .await
            .expect("Could not send post: get token data")
            .json::<Vec<Value>>()
            .await
            .expect("Server did not return token data");
        //logging::log!("Received {:#?}", res);
        res
}
    
pub fn simplify(d: String) -> String {
    format!("{} 00:00:00", d)           // parsed for server query
}
/// Renders the home page of your application.
#[component]
fn HomePage() -> impl IntoView {
    let (correlation_factor, set_correlation_factor) = create_signal(0.7);
    let (days_execution, set_days_execution) = create_signal(30);
    let (name, set_name) = create_signal("test".to_string()); 
    let (start, set_start) = create_signal(String::new());        
    let (end, set_end) = create_signal(String::new());        

    let (token_selected, set_token_selected) = create_signal::<Option<String>>(None);

    let tokens =
        create_local_resource(move || (), |_| { fetch_tokens() });

    let token_data = create_resource(
        token_selected, 
        move |n| async move {
            let s = move || start.get();
            let e = move || end.get();
            if let Some(name) = n {
                logging::log!("Fetching {:#?} data", name);
                fetch_token_data(simplify(s()), simplify(e()), name).await
            } else {
                logging::log!("no token, no fetch");
                Vec::new()
            }
        }
    );

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

                        {move || match token_data.get() {
                            None => view! { <p>"Loading data..."</p> }.into_view(),
                            Some(data) => {
                                let token_table = data.into_iter().map(|t| 
                                    {
                                    let c: ClosePrices = t.into();
                                    view!{
                                        <tr>
                                        <td>{c.opentime}</td>
                                        <td>{c.close}</td>
                                        </tr>
                                    }}).collect_view();
                                view! { 
                                    <table>
                                    <tr>
                                        <th>"Open time"</th>
                                        <th>"Close price (USDT)"</th>
                                    </tr>
                                    {token_table}
                                    </table> 
                                }.into_view()
                            }
                        }}
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
