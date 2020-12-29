//
// middleware.rs
// Ancilla
// 
// Author: Wess Cope (wess@frenzylabs.com)
// Created: 12/22/2020
// 
// Copywrite (c) 2020 FrenzyLabs, LLC.
//

use crate::Store;

pub type Middleware<State, Action> = fn(&mut Store<State, Action>, Action) -> Option<Action>;