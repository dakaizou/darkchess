use yew::prelude::*;
use yew::{Component, Properties};

use crate::piece::Piece;
use crate::cell::Cell;


#[derive(PartialEq, Properties)]
pub struct Props {
    pub pieces: Vec<Option<Piece>>,
    pub select_cell_messenger: Callback<usize>,
    pub selected_cell: Option<usize>,
}

pub struct Board;

impl Component for Board {
    type Message = ();

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props {
            pieces,
            select_cell_messenger,
            selected_cell,
        } = &ctx.props();

        let render_cell = |row: usize, col: usize, piece: Option<Piece>| {
            let select_cell_messenger = select_cell_messenger.clone();
            let row = row.clone();
            let col = col.clone();
            let onclick = Callback::from(move |_| {
                select_cell_messenger.emit(8 * row + col);
            });

            let selected = match selected_cell {
                Some(pos) => *pos == 8 * row + col,
                None => false,
            };

            let red = match &piece {
                Some(p) => !p.is_black() && p.is_revealed(),
                None => false,
            };

            html! {
                <div {onclick} class={classes!("cell", selected.then(|| Some("selected")), red.then(|| Some("red")))}>
                    <Cell piece={piece}></Cell>
                </div>
            }
        };

        let render_row = |row: usize, pieces: Vec<Option<Piece>>| {
            html! {
                <div class={classes!("board-row")}>
                    { pieces.into_iter().enumerate().map(|(col, piece)| render_cell(row, col, piece)).collect::<Html>() }
                </div>
            }
        };

        let mut rows = Vec::new();
        for i in 0..4 {
            rows.push(
                pieces
                    .clone()
                    .drain(i * 8..i * 8 + 8)
                    .collect::<Vec<Option<Piece>>>(),
            );
        }

        html! {
            <div class={classes!("board")}>
                { rows.into_iter().enumerate().map(|(row, pieces)| render_row(row, pieces)).collect::<Html>()}
            </div>
        }
    }
}
