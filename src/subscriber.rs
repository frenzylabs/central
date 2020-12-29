//
// subscriber.rs
// Ancilla
// 
// Author: Wess Cope (wess@frenzylabs.com)
// Created: 12/22/2020
// 
// Copywrite (c) 2020 FrenzyLabs, LLC.
//

pub type Subscriber<State> = fn(&State);