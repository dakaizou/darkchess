use yew::prelude::*;
use yew::{Component, Properties};

use crate::piece::Piece;
use crate::rank::Rank;


#[derive(PartialEq, Properties)]
pub struct Props {
    pub piece: Option<Piece>,
}

pub enum Msg {}

pub struct Cell;

impl Component for Cell {
    type Message = Msg;

    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Cell
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { piece } = &ctx.props();

        html! {
            <div>
            {
                match piece {
                    Some(p) => {
                        match p.is_revealed() {
                            true => {
                                match p.is_black() {
                                    true => match p.rank() {
                                        Rank::King => "將",
                                        Rank::General => "士",
                                        Rank::Colonel => "象",
                                        Rank::Major => "車",
                                        Rank::Captain => "馬",
                                        Rank::Cannon => "包",
                                        Rank::Sodier => "卒",
                                    }
                                    false => match p.rank() {
                                        Rank::King => "帥",
                                        Rank::General => "仕",
                                        Rank::Colonel => "相",
                                        Rank::Major => "俥",
                                        Rank::Captain => "傌",
                                        Rank::Cannon => "炮",
                                        Rank::Sodier => "兵",
                                    }
                                }
                            }
                            false => {
                                "暗"
                            }
                        }
                    },
                    None => "",
                }
            }
            </div>
        }
    }

}