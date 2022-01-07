use crate::lexer::SyntaxKind;
use rowan::SmolStr;

#[derive(Clone, Debug)]
pub(super) enum Event {
    StartNode { kind: SyntaxKind },
    StartNodeAt { kind: SyntaxKind, checkpoint: usize },
    AddToken { kind: SyntaxKind, text: SmolStr },
    FinishNode,
}
