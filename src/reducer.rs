//
// reducer.rs
// Ancilla
// 
// Author: Wess Cope (wess@frenzylabs.com)
// Created: 12/22/2020
// 
// Copywrite (c) 2020 FrenzyLabs, LLC.
//

pub type Reducer <State, Action> = fn(&State, &Action) -> State;

#[macro_export]
macro_rules! combine_reducers {
  ($state:ty, $action:ty, $reducer:ident) => ($reducer);
  ($state:ty, $action:ty, $first:ident, $($second:ident),+) => (
      |state: &$state, action: $action| -> $state {
          (combine_reducers!($state, $action, $($second),+))(&$first(state, action), action)
      }
  )
}