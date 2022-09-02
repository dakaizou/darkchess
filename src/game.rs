use rand::seq::SliceRandom;
use rand::thread_rng;
use yew::prelude::*;
use yew::Component;

use crate::board::Board;
use crate::piece::Piece;
use crate::rank::{Rank, RANK_SET};

pub enum Msg {
    Select(usize),
}

pub struct Game {
    current_board: Vec<Option<Piece>>,
    is_black_turn: Option<bool>,
    selected_cell: Option<usize>,
}

impl Component for Game {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut pieces = Vec::new();

        for rank in RANK_SET.iter() {
            pieces.push(Some(Piece::new(rank, true)));
            pieces.push(Some(Piece::new(rank, false)));
        }

        pieces.shuffle(&mut thread_rng());
        Self {
            current_board: pieces,
            is_black_turn: None,
            selected_cell: None,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // no select pieces that aren't revealed
        // no select None
        match msg {
            Msg::Select(pos) => {
                if let Some(current_piece) = &self.current_board[pos] {
                    // current select not empty
                    match current_piece.is_revealed() {
                        true => {
                            // current select is revealed

                            if let Some(prev_pos) = self.selected_cell {
                                // previous select not empty

                                if prev_pos == pos {
                                    // same cell
                                    self.selected_cell = None;
                                } else if !self.current_board[prev_pos]
                                    .as_ref()
                                    .unwrap()
                                    .can_attack(&current_piece)
                                    || !self.is_valid_attack(prev_pos, pos)
                                {
                                    log::warn!("invalid attack");
                                    self.selected_cell = None;
                                } else {
                                    // attack
                                    self.current_board[pos] = self.current_board[prev_pos].clone();
                                    self.current_board[prev_pos] = None;
                                    self.selected_cell = None;
                                    self.is_black_turn = self.next_turn_black(false);
                                }
                            } else {
                                // previous selection is empty
                                self.selected_cell = Some(pos);
                            }
                        }
                        false => {
                            self.current_board[pos].as_mut().unwrap().reveal();
                            self.selected_cell = None;
                            self.is_black_turn = self.next_turn_black(
                                self.current_board[pos].as_ref().unwrap().is_black(),
                            );
                        }
                    }
                } else {
                    if let Some(prev_pos) = self.selected_cell {
                        // move
                        if self.is_valid_move(prev_pos, pos) {
                            self.current_board[pos] = self.current_board[prev_pos].clone();
                            self.current_board[prev_pos] = None;
                            self.selected_cell = None;
                            self.is_black_turn = self.next_turn_black(false);
                        } else {
                            self.selected_cell = None;
                        }
                    } else {
                        self.selected_cell = None
                    }
                }
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let select_cell_messenger = Callback::from(move |pos| {
            link.send_message(Msg::Select(pos));
        });
        html! {
            <>
                <Board pieces={self.current_board.clone()} {select_cell_messenger} selected_cell={self.selected_cell}></Board>
                <div id="turn">
                    {"black turn: "}
                    {
                        if let Some(black_turn) = self.is_black_turn {
                            format!("{}", black_turn)
                        } else {
                            "".to_owned()
                        }
                    }
                </div>
            </>
        }
    }
}

impl Game {
    fn next_turn_black(&self, revealed_black: bool) -> Option<bool> {
        match self.is_black_turn {
            Some(black_turn) => Some(!black_turn),
            None => Some(!revealed_black),
        }
    }

    fn is_valid_move(&self, from: usize, to: usize) -> bool {
        if let Some(black_turn) = self.is_black_turn {
            if black_turn != self.current_board[from].as_ref().unwrap().is_black() {
                log::debug!("turn error move");
                return false;
            }
        } else {
            return false;
        }

        if self.current_board[to].is_some() {
            log::debug!("attempted to move to non empty cell");
            return false;
        }

        let (from_row, from_col) = get_pos(from);
        let (to_row, to_col) = get_pos(to);

        if from_row != to_row && from_col != to_col {
            log::debug!("out of position move");
            return false;
        }

        if from_row == to_row {
            return (from_col == to_col.wrapping_add(1)) || (from_col == to_col.wrapping_sub(1));
        } else {
            return (from_row == to_row.wrapping_add(1)) || (from_row == to_row.wrapping_sub(1));
        }
    }

    fn is_valid_attack(&self, from: usize, to: usize) -> bool {
        if let Some(black_turn) = self.is_black_turn {
            if black_turn != self.current_board[from].as_ref().unwrap().is_black() {
                log::debug!("turn error attack");
                return false;
            }
        } else {
            return false;
        }

        let (from_row, from_col) = get_pos(from);
        let (to_row, to_col) = get_pos(to);
        let from_piece = self.current_board[from].as_ref().unwrap();
        let to_piece = self.current_board[to].as_ref().unwrap();

        if from_piece.is_black() == to_piece.is_black() {
            log::debug!("same color attack");
            return false;
        }

        if from_row != to_row && from_col != to_col {
            log::debug!("out of position attack");
            return false;
        }

        if *self.current_board[from].as_ref().unwrap().rank() != Rank::Cannon {
            if from_row == to_row {
                return (from_col == to_col.wrapping_add(1))
                    || (from_col == to_col.wrapping_sub(1));
            } else {
                return (from_row == to_row.wrapping_add(1))
                    || (from_row == to_row.wrapping_sub(1));
            }
        } else {
            let mut count = 0;
            if from_row == to_row {
                let (from_col, to_col) = minmax(from_col, to_col);
                for i in from_col..to_col {
                    if self.current_board[get_index(from_row, i)].is_some() {
                        count += 1;
                    }
                }
            } else {
                let (from_row, to_row) = minmax(from_row, to_row);
                for i in from_row..to_row {
                    if self.current_board[get_index(i, from_col)].is_some() {

                        count += 1;
                    }
                }
            }
            return count == 2;
        }
    }
}

fn get_index(row: usize, col: usize) -> usize {
    row * 8 + col
}

fn get_pos(pos: usize) -> (usize, usize) {
    (pos / 8, pos % 8)
}

fn minmax(a: usize, b: usize) -> (usize, usize) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}
