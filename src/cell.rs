use yew::prelude::*;
use yew::{Component, Properties};

use crate::piece::Piece;
use crate::rank::Rank;
use crate::game::Color;

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
                                match p.color() {
                                    Color::Black => match p.rank() {
                                        Rank::General => "將",
                                        Rank::Advisor => "士",
                                        Rank::Elephant => "象",
                                        Rank::Chariot => "車",
                                        Rank::Horse => "馬",
                                        Rank::Cannon => "包",
                                        Rank::Sodier => "卒",
                                    }
                                    Color::Red => match p.rank() {
                                        Rank::General => "帥",
                                        Rank::Advisor => "仕",
                                        Rank::Elephant => "相",
                                        Rank::Chariot => "俥",
                                        Rank::Horse => "傌",
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
