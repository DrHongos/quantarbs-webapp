use leptos::*;

/* 

add callback with datetime selected


day/month/year logic
- detect current datetime and set default
- add relative selectors (+x months, etc)

*/

#[component]
pub fn DatePicker(setter: WriteSignal<String>) -> impl IntoView {
    let (day, set_day) = create_signal(1);
    let (month, set_month) = create_signal(1);
    let (year, set_year) = create_signal(2022);

    create_effect(move |_| {
        let d = day.get();
        let m = month.get();
        let y = year.get();
        setter.set(format!("{:#?}-{:#?}-{:#?}", y, m, d));
    });

    view! {
       <div class="datePicker">       
        <label>"Day"</label>
        <input type="number"
            on:input=move |ev| {
                let v = event_target_value(&ev).parse::<i32>().expect("Error parsing");
                if v > 0 && v <= 31 {
                    set_day(v);
                } else {
                    set_day(day.get());
                    logging::log!("Error!");
                }
            }
            prop:value=day
        /><br />

        <label>"Month"</label>
        <input type="number"
            on:input=move |ev| {
                let v = event_target_value(&ev).parse::<i32>().expect("Error parsing");
                if v > 0 && v <= 12 { set_month(v) }
                else {
                   set_month(month.get())
                }
            }
            prop:value=month
        /><br />

        <label>"Year"</label>
        <input type="number"
            on:input=move |ev| {
                let y = event_target_value(&ev).parse::<i32>().expect("Error parsing");
                if y >= 2022 && y <= 2023 {set_year(y)}
                else {
                    set_year(year.get())
                }
            }
            prop:value=year
        /><br />
/*         <button
            on:click = move |_| setter.set(format!("{:#?}-{:#?}-{:#?}", year.get(), month.get(), day.get()))
        >
            "set"
        </button> */
       </div> 
    }
}