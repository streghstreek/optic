use super::{
  JlasArrayVisitor, JlasPrimitiveVisitor, JsonBodyVisitor, JsonBodyVisitors, VisitorResults,
};
use crate::queries::shape::ChoiceOutput;
use crate::shapes::ShapeDiffResult;
use crate::shapes::{JsonTrail, ShapeTrail};
use crate::state::shape::ShapeKind;
use serde_json::Value as JsonValue;

pub struct DiffVisitors {
  array: DiffArrayVisitor,
  primitive: DiffPrimitiveVisitor,
}

impl DiffVisitors {
  pub fn new() -> Self {
    DiffVisitors {
      array: DiffArrayVisitor::new(),
      primitive: DiffPrimitiveVisitor::new(),
    }
  }
}

type DiffResults = VisitorResults<ShapeDiffResult>;

impl JsonBodyVisitors<ShapeDiffResult> for DiffVisitors {
  type Array = DiffArrayVisitor;
  type Primitive = DiffPrimitiveVisitor;

  fn array(&mut self) -> &mut DiffArrayVisitor {
    &mut self.array
  }

  fn primitive(&mut self) -> &mut DiffPrimitiveVisitor {
    &mut self.primitive
  }
}

// Primitive visitor
// -----------------

pub struct DiffPrimitiveVisitor {
  results: DiffResults,
}

impl DiffPrimitiveVisitor {
  pub fn new() -> Self {
    Self {
      results: DiffResults::new(),
    }
  }
}

impl JsonBodyVisitor<ShapeDiffResult> for DiffPrimitiveVisitor {
  fn results(&mut self) -> Option<&mut DiffResults> {
    Some(&mut self.results)
  }
}

impl JlasPrimitiveVisitor<ShapeDiffResult> for DiffPrimitiveVisitor {
  fn visit(
    &mut self,
    json: JsonValue,
    json_trail: JsonTrail,
    trail_origin: ShapeTrail,
    trail_choices: Vec<ChoiceOutput>,
  ) {
    if trail_choices.is_empty() {
      self.results.push(ShapeDiffResult::UnspecifiedShape {
        json_trail,
        shape_trail: trail_origin,
      });
      return;
    }

    let (matched, unmatched): (Vec<&ChoiceOutput>, Vec<&ChoiceOutput>) =
      trail_choices.iter().partition(|choice| match &json {
        JsonValue::Bool(x) => match choice.core_shape_kind {
          ShapeKind::BooleanKind => true,
          _ => false,
        },
        JsonValue::Number(x) => match choice.core_shape_kind {
          ShapeKind::NumberKind => true,
          _ => false,
        },
        JsonValue::String(x) => match choice.core_shape_kind {
          ShapeKind::StringKind => true,
          _ => false,
        },
        JsonValue::Null => match choice.core_shape_kind {
          ShapeKind::NullableKind => true,
          _ => false,
        },
        _ => unreachable!("should not call primitive visitor without a json primitive value"),
      });
    if matched.is_empty() {
      unmatched.iter().for_each(|&choice| {
        self.results.push(ShapeDiffResult::UnmatchedShape {
          json_trail: json_trail.clone(),
          shape_trail: choice.shape_trail(),
        });
      });
    }
  }
}

// Array visitor
// -------------

pub struct DiffArrayVisitor {
  results: DiffResults,
}

impl DiffArrayVisitor {
  pub fn new() -> Self {
    Self {
      results: DiffResults::new(),
    }
  }
}

impl JsonBodyVisitor<ShapeDiffResult> for DiffArrayVisitor {
  fn results(&mut self) -> Option<&mut DiffResults> {
    Some(&mut self.results)
  }
}

impl JlasArrayVisitor<ShapeDiffResult> for DiffArrayVisitor {
  fn visit(
    &mut self,
    json: JsonValue,
    json_trail: JsonTrail,
    trail_origin: ShapeTrail,
    trail_choices: Vec<ChoiceOutput>,
  ) -> Vec<ChoiceOutput> {
    if trail_choices.is_empty() {
      self.results.push(ShapeDiffResult::UnspecifiedShape {
        json_trail,
        shape_trail: trail_origin,
      });
      return vec![];
    }

    let (matched, unmatched): (Vec<&ChoiceOutput>, Vec<&ChoiceOutput>) =
      trail_choices.iter().partition(|choice| match &json {
        JsonValue::Array(_) => match choice.core_shape_kind {
          ShapeKind::ListKind => true,
          _ => false,
        },
        _ => unreachable!("should only call array visitor for array json types"),
      });

    if matched.is_empty() {
      unmatched.iter().for_each(|&choice| {
        self.results.push(ShapeDiffResult::UnmatchedShape {
          json_trail: json_trail.clone(),
          shape_trail: choice.shape_trail(),
        });
      });
    }

    vec![]
  }
}