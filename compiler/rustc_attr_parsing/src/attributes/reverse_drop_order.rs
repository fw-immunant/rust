use rustc_attr_ir::AttributeKind;
use rustc_attr_ir::target::Target;
use rustc_feature::AttributeStability;
use rustc_span::{Span, Symbol, sym};

use crate::attributes::{NoArgsAttributeParser, OnDuplicate};
use crate::target_checking::AllowedTargets;
use crate::target_checking::Policy::Allow;
//use crate::unstable;

pub(crate) struct ReverseDropOrderParser;

impl NoArgsAttributeParser for ReverseDropOrderParser {
    const PATH: &[Symbol] = &[sym::reverse_drop_order];
    const ON_DUPLICATE: OnDuplicate = OnDuplicate::Warn;
    const ALLOWED_TARGETS: AllowedTargets<'_> =
        AllowedTargets::AllowList(&[Allow(Target::Enum), Allow(Target::Struct)]);
    const STABILITY: AttributeStability = AttributeStability::Stable; //unstable!(reverse_drop_order_attr);//AttributeStability::Unstable;
    const CREATE: fn(Span) -> AttributeKind = AttributeKind::ReverseDropOrder;
}
