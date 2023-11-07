pub trait IterDirection {}

#[derive(Debug, Clone, Copy)]
pub struct Column;

impl IterDirection for Column {}

#[derive(Debug, Clone, Copy)]
pub struct Row;

impl IterDirection for Row {}

#[derive(Debug, Clone, Copy)]
pub struct ElementsReverse;

impl IterDirection for ElementsReverse {}
