//
// store.rs
// Ancilla
// 
// Author: Wess Cope (wess@frenzylabs.com)
// Created: 12/22/2020
// 
// Copywrite (c) 2020 FrenzyLabs, LLC.
//

use crate::{
  Reducer,
  Middleware,
  Subscriber
};

pub struct Store<State, Action> {
  state:State,
  reducer: Reducer<State, Action>,
  middleware: Vec<Middleware<State, Action>>,
  subscribers: Vec<Subscriber<State>>
}

impl <State, Action> Store<State, Action> {
  pub fn new(reducer:Reducer<State, Action>, initial_state: State) -> Self {
    Self {
      reducer,
      state: initial_state,
      middleware: Vec::new(),
      subscribers: Vec::new()
    }
  }

  pub fn state(&self) -> &State {
    &self.state
  }

  pub fn dispatch(&mut self, action:Action) {
    if self.middleware.is_empty() {
      self.run_reducer(&action);
    } else {
      self.run_middleware(0, action);
    }
  }

  pub fn subscribe(&mut self, subscriber: Subscriber<State>) {
    self.subscribers.push(subscriber)
  }

  pub fn add(&mut self, middleware:Middleware<State, Action>) {
    self.middleware.push(middleware);
  }

  pub fn replace(&mut self, reducer:Reducer<State, Action>) {
    self.reducer = reducer;
  }

  /// Private
  fn run_middleware(&mut self, index:usize, action:Action) {
    if index == self.middleware.len() {
      self.run_reducer(&action);
      return;
    }

    let next = self.middleware[index](self, action);

    if next.is_none() {
      return;
    }

    self.run_middleware(index + 1, next.unwrap());
  }

  fn run_reducer(&mut self, action:&Action) {
    self.state = (&self.reducer)(self.state(), action);

    self.publish();
  }

  fn publish(&self) {
    for sub in &self.subscribers {
      sub(self.state());
    }
  }

  
}