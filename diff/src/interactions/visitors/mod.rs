pub mod diff;

use super::HttpInteraction;
use crate::state::endpoint::PathComponentId;

pub trait InteractionVisitors<R> {
  type Path: PathVisitor<R>;

  fn path(&mut self) -> &mut Self::Path;

  fn take_results(&mut self) -> Option<Vec<R>> {
    // TODO: flatten results once we have more types of visitors
    self.path().take_results()
  }
}

pub trait InteractionVisitor<R> {
  fn results(&mut self) -> Option<&mut VisitorResults<R>> {
    None
  }

  fn push(&mut self, result: R) {
    if let Some(results) = self.results() {
      results.push(result);
    }
  }

  fn take_results(&mut self) -> Option<Vec<R>> {
    if let Some(results) = self.results() {
      results.take_results()
    } else {
      None
    }
  }
}

pub trait PathVisitor<R>: InteractionVisitor<R> {
  fn visit(&mut self, interaction: &HttpInteraction, context: PathVisitorContext);
}

pub struct PathVisitorContext<'a> {
  pub path: Option<&'a PathComponentId>,
}

// Results
// -------

pub struct VisitorResults<R> {
  results: Option<Vec<R>>,
}

impl<R> VisitorResults<R> {
  fn new() -> Self {
    VisitorResults {
      results: Some(vec![]),
    }
  }

  fn push(&mut self, result: R) {
    if let Some(results) = &mut self.results {
      results.push(result);
    }
  }

  fn take_results(&mut self) -> Option<Vec<R>> {
    let flushed_results = self.results.take();
    self.results = Some(vec![]);
    flushed_results
  }
}
