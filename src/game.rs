use rand::seq::SliceRandom;
use rand::thread_rng;
use std::mem;
use yew::prelude::*;
use yew::Component;

use crate::board::Board;
use crate::piece::Piece;
use crate::rank::{Rank, RANK_SET};

pub enum Msg {
    Reveal(usize),
    Move(usize, usize),
    Attack(usize, usize),
    Select(usize),
    Unselect,
}

#[derive(Debug, Clone)]
pub enum SelectionState {
    NoSelection,
    Cell(usize),
}

#[derive(Debug, Clone, PartialEq)]
pub enum Color {
    Black,
    Red,
}

#[derive(Debug, Clone)]
pub enum TurnState {
    First,
    Rest(Color),
}

pub struct Game {
    current_board: Vec<Option<Piece>>,
    turn_state: TurnState,
    selection_state: SelectionState,
}

impl Component for Game {
    type Message = Msg;

    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        let mut pieces = Vec::new();

        for rank in RANK_SET.iter() {
            pieces.push(Some(Piece::new(rank, Color::Black)));
            pieces.push(Some(Piece::new(rank, Color::Red)));
        }

        pieces.shuffle(&mut thread_rng());
        Self {
            current_board: pieces,
            turn_state: TurnState::First,
            selection_state: SelectionState::NoSelection,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        // no select pieces that aren't revealed
        // no select None
        match msg {
            Msg::Reveal(pos) => {
                if let Some(piece) = self.current_board[pos].as_mut() {
                    piece.reveal();
                }
                match self.turn_state {
                    TurnState::First => {
                        self.flip_turn_first(self.current_board[pos].as_ref().unwrap().color());
                    }
                    _ => self.flip_turn(),
                };
            }
            Msg::Move(from, to) => self.mv(from, to),
            Msg::Select(pos) => self.selection_state = SelectionState::Cell(pos),
            Msg::Unselect => self.selection_state = SelectionState::NoSelection,
            Msg::Attack(from, to) => {
                if self.current_board[from]
                    .as_ref()
                    .unwrap()
                    .can_attack(self.current_board[to].as_ref().unwrap())
                {
                    self.attack(from, to);
                } else {
                    log::debug!("Rank error attack")
                }
            }
        };
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link().clone();
        let board = self.current_board.clone();
        let selection_state = self.selection_state.clone();
        let turn_state = self.turn_state.clone();
        let select_cell_messenger: Callback<usize> = Callback::from(move |pos: usize| {
            match selection_state {
                SelectionState::NoSelection => {
                    if let Some(piece) = board[pos].as_ref() {
                        if piece.is_revealed() {
                            link.send_message(Msg::Select(pos));
                        } else {
                            link.send_message(Msg::Reveal(pos));
                        }
                    }
                }
                SelectionState::Cell(current_pos) => {
                    if let Some(piece) = &board[pos] {
                        if piece.is_revealed() {
                            if pos == current_pos {
                                link.send_message(Msg::Unselect);
                            } else if Game::is_valid_attack(current_pos, pos, &turn_state, &board) {
                                link.send_message(Msg::Attack(current_pos, pos))
                            } else {
                                link.send_message(Msg::Select(pos))
                            }
                        } else {
                            link.send_message(Msg::Reveal(pos));
                        }
                    } else {
                        if Game::is_valid_move(current_pos, pos, &turn_state, &board) {
                            link.send_message(Msg::Move(current_pos, pos))
                        } else {
                            link.send_message(Msg::Unselect)
                        }
                    }
                }
            };
        });
        let selected_cell = match self.selection_state {
            SelectionState::NoSelection => None,
            SelectionState::Cell(pos) => Some(pos),
        };
        html! {
            <>
                <Board pieces={self.current_board.clone()} {select_cell_messenger} selected_cell={selected_cell}></Board>
                <div id="turn">
                    {"Turn: "}
                    {
                        format!("{:?}", self.turn_state)
                    }
                </div>
            </>
        }
    }
}

impl Game {
    fn flip_turn_first(&mut self, reveal_color: Color) {
        self.turn_state = match reveal_color {
            Color::Black => TurnState::Rest(Color::Red),
            Color::Red => TurnState::Rest(Color::Black),
        };
    }
    fn flip_turn(&mut self) {
        self.turn_state = match self.turn_state {
            TurnState::First => panic!("call flip_turn_first instead"),
            TurnState::Rest(Color::Black) => TurnState::Rest(Color::Red),
            TurnState::Rest(Color::Red) => TurnState::Rest(Color::Black),
        };
    }

    fn mv(&mut self, from: usize, to: usize) {
        self.current_board[to] = mem::take(&mut self.current_board[from]);
        self.selection_state = SelectionState::NoSelection;
        self.flip_turn();
    }

    fn attack(&mut self, from: usize, to: usize) {
        self.current_board[to] = self.current_board[from].clone();
        self.current_board[from] = None;
        self.selection_state = SelectionState::NoSelection;
        self.flip_turn();
    }

    fn is_valid_move(
        from: usize,
        to: usize,
        turn_state: &TurnState,
        board: &Vec<Option<Piece>>,
    ) -> bool {
        match turn_state {
            TurnState::First => panic!("Impossible to move in the first turn"),
            TurnState::Rest(color) => {
                if *color != board[from].as_ref().unwrap().color() {
                    log::debug!("Turn error move");
                    return false;
                }
            }
        };
        if board[to].is_some() {
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
            (from_col == to_col.wrapping_add(1)) || (from_col == to_col.wrapping_sub(1))
        } else {
            (from_row == to_row.wrapping_add(1)) || (from_row == to_row.wrapping_sub(1))
        }
    }

    fn is_valid_attack(
        from: usize,
        to: usize,
        turn_state: &TurnState,
        board: &Vec<Option<Piece>>,
    ) -> bool {
        match turn_state {
            TurnState::First => panic!("Impossible to attack in the first turn"),
            TurnState::Rest(color) => {
                if *color != board[from].as_ref().unwrap().color() {
                    log::debug!("Turn error attack");
                    return false;
                }
            }
        };

        let (from_row, from_col) = get_pos(from);
        let (to_row, to_col) = get_pos(to);
        let from_piece = board[from].as_ref().unwrap();
        let to_piece = board[to].as_ref().unwrap();

        if from_piece.color() == to_piece.color() {
            log::debug!("same color attack");
            return false;
        }

        if from_row != to_row && from_col != to_col {
            log::debug!("out of position attack");
            return false;
        }

        if *board[from].as_ref().unwrap().rank() != Rank::Cannon {
            if from_row == to_row {
                (from_col == to_col.wrapping_add(1)) || (from_col == to_col.wrapping_sub(1))
            } else {
                (from_row == to_row.wrapping_add(1)) || (from_row == to_row.wrapping_sub(1))
            }
        } else {
            let mut count = 0;
            if from_row == to_row {
                let (from_col, to_col) = minmax(from_col, to_col);
                for i in from_col..to_col {
                    if board[get_index(from_row, i)].is_some() {
                        count += 1;
                    }
                }
            } else {
                let (from_row, to_row) = minmax(from_row, to_row);
                for i in from_row..to_row {
                    if board[get_index(i, from_col)].is_some() {
                        count += 1;
                    }
                }
            }
            count == 2
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
