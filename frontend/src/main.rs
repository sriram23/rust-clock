use chrono::prelude::*;
use chrono_tz::Asia::Kolkata;
use yew::prelude::*;
use gloo_timers::callback::Interval;

struct Time {
    hr: String,
    min: String,
    sec: String,
    hr_angle: String,
    min_angle: String,
    sec_angle: String
}

struct Angle {
    hr: String,
    min: String,
    sec: String
}
fn main() {
    Interval::new(1_000, move || {
        yew::Renderer::<App>::new().render();
    }).forget();
}

fn calculate_angle (hour: &String, minute:&String, second:&String) -> Angle {
    let start_value = -90;
    let current_angle = Angle {
        hr: format!("{}",((((hour.parse::<i32>().unwrap())%12)*30)+start_value)),
        min: format!("{}",(((minute.parse::<i32>().unwrap())*6)+start_value)),
        sec: format!("{}",(((second.parse::<i32>().unwrap())*6)+start_value))
    };
    return current_angle;
}

fn calculate_values() -> Time {
    // TODO: Make the clock generic. Allow multiple timezone, use dropdown kind of thing
    let hr =  Utc::now().with_timezone(&Kolkata).hour().to_string();
    let min = Utc::now().with_timezone(&Kolkata).minute().to_string();
    let sec = Utc::now().with_timezone(&Kolkata).second().to_string();
    let angle = calculate_angle(&hr, &min, &sec);
    let current_time = Time {
        hr,
        min,
        sec,
        hr_angle: angle.hr,
        min_angle: angle.min,
        sec_angle: angle.sec,
    };
    return current_time;
}

#[function_component(App)]
pub fn app() -> Html {
    let time = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12];
    let now = calculate_values();
    let now_title = "Current time: ";
    html! {
        <div class="main-container">
            <div class="clock-wrapper">
                <div class="clock">
                    <div class="sec" style={format!("transform: rotate({}deg)",now.sec_angle)}></div>
                    <div class="min" style={format!("transform: rotate({}deg)",now.min_angle)}></div>
                    <div class="hr" style={format!("transform: rotate({}deg)",now.hr_angle)}></div>
                    <div class="center-point"></div>
                    <div class="inner-center-point"></div>
                    <div class="number n1">{time[0]}</div>
                    <div class="number n2">{time[1]}</div>
                    <div class="number n3">{time[2]}</div>
                    <div class="number n4">{time[3]}</div>
                    <div class="number n5">{time[4]}</div>
                    <div class="number n6">{time[5]}</div>
                    <div class="number n7">{time[6]}</div>
                    <div class="number n8">{time[7]}</div>
                    <div class="number n9">{time[8]}</div>
                    <div class="number n10">{time[9]}</div>
                    <div class="number n11">{time[10]}</div>
                    <div class="number n12">{time[11]}</div>
                </div>
                <div class="knob"></div>
            </div>
            <p>{now_title}<span id="time">{format!("{:0>2}:{:0>2}:{:0>2}",now.hr, now.min, now.sec)}</span></p>
    </div>
    }
}
