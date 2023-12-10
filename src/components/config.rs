use leptos::*;
use crate::components::date_picker::DatePicker;
use crate::structs::ClosePrices;
use crate::utils::{simplify, fetch_token_data};

#[component]
pub fn Config(
    tokens: Resource<(), Vec<String>>,
) -> impl IntoView {
    
    let (correlation_factor, set_correlation_factor) = create_signal(0.7);
    let (days_execution, set_days_execution) = create_signal(30);
    let (name, set_name) = create_signal("test".to_string()); 
    let (start, set_start) = create_signal(String::new());        
    let (end, set_end) = create_signal(String::new());        

    let (token_selected, set_token_selected) = create_signal::<Vec<String>>(Vec::new());

/* 
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
 */
    view! {
    <div class="config">    
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
                            on:click= move |_|  
                                if let Some(index) = token_selected.get().iter().position(|x| *x == c) {
                                    set_token_selected.update(|curr| {
                                        curr.swap_remove(index);
                                        curr;
                                    })
                                } else {
                                    set_token_selected.update(|curr| curr.push(c.clone()))
                                } 
                            class=if token_selected.get().contains(&t) {"token"} else {"token-u"}>
                            {t}
                        </button>
                    }}).collect_view();
                view! { <div>
                            {tokens_view}
                            <hr />
                            <button
                                on:click = move |_| set_token_selected.set(tokens.get().unwrap())
                            >"Select all"</button>
                            <button
                                on:click = move |_| set_token_selected.set(Vec::new())
                            >"Unselect all"</button>
                        </div> 
                }.into_view()
            }
        }}

         {move || token_selected.get().into_iter().map(|t| {view! {<p>{t}</p>}}).collect_view().into_view()}
            
/*             
             {
            None => view! { <p>"Select a token pair"</p> }.into_view(),
            Some(selected) => {
                view! {
                    <>
                        <p>{selected}</p>

/*                         {move || match token_data.get() {
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
                                        <th>"TODO: selected"</th>
                                    </tr>
                                    {token_table}
                                    </table> 
                                }.into_view()
                            }
                        }} */
                    </>
                }.into_view()
            }
        }} 
 */
    </div>
    }.into_view()
}