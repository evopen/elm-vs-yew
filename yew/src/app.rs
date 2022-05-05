use crate::utils;
use gloo_events::EventListener;
use log::*;
use serde_derive::{Deserialize, Serialize};
use yew::{html::Scope, prelude::*};

const DIV_SIZE: f64 = 10.0;

#[allow(dead_code)]
pub struct App {
    state: State,
    listener: EventListener,
}

#[derive(Serialize, Deserialize)]
pub struct State {
    entries: Vec<Vec<bool>>,
}

pub enum Msg {
    Toggle(usize, usize, bool),
    Resize,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let link = ctx.link().clone();
        let root_element = utils::get_root_element().get_bounding_client_rect();
        let inner_height = (root_element.height() / DIV_SIZE).ceil() as usize;
        let inner_width = (root_element.width() / DIV_SIZE).floor() as usize;

        debug!("Creating a grid {:?}x{:?}", inner_height, inner_width);

        let entries = vec![vec![false; inner_width]; inner_height];
        let state = State { entries };

        let link = link.clone();
        let listener = EventListener::new(&web_sys::window().unwrap(), "resize", move |_| {
            link.send_message(Msg::Resize)
        });

        App { state, listener }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        debug!("Update");
        match msg {
            Msg::Toggle(i, j, state) => {
                self.state.toggle(i, j, state);
            }
            Msg::Resize => {
                self.state.resize();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        debug!("Rendered");

        html! {
            <div class="grid-wrapper" >
                {
                    for self.state.entries.iter().enumerate().map(|(i, row)| {
                        html! {
                            <div class="row" key={format!("row-{}", i)}>
                                {
                                    for row.iter().enumerate().map(|(j, cell)| self.view_cell(i, j, *cell, link))
                                }
                            </div>
                        }
                    })
                }
            </div>
        }
    }
}

impl App {
    fn view_cell(&self, i: usize, j: usize, cell: bool, link: &Scope<Self>) -> Html {
        html! {
        <div key={format!("cell-{}-{}", i, j)}
             class={classes!("cell", cell.then(|| "cell--active"))}
             onmouseenter={link.callback(move |_| Msg::Toggle(i, j, true))}
             onmouseleave={link.callback(move |_| Msg::Toggle(i, j, false))}>
        </div>        }
    }
}

impl State {
    fn toggle(&mut self, i: usize, j: usize, state: bool) {
        debug!("Toggle");
        self.entries[i][j] = state;
    }

    fn resize(&mut self) {
        debug!("Resize");

        let root_element = utils::get_root_element().get_bounding_client_rect();
        let inner_height = (root_element.height() / DIV_SIZE).ceil() as usize;
        let inner_width = (root_element.width() / DIV_SIZE).floor() as usize;

        debug!("Creating a grid {:?}x{:?}", inner_height, inner_width);

        let entries = vec![vec![false; inner_width]; inner_height];
        self.entries = entries;
    }
}
