use super::Rule;

/// Convert [`pest`] rules to human readable names
///
/// This is useful for error reporting.
pub fn renamed_rules(rule: &Rule) -> String {
    use Rule as R;

    match rule {
        R::stmt => "new statement".into(),
        R::eq => "=".into(),
        R::arexpr => "arithmetic expression".into(),
        R::relop => "relational operator".into(),
        R::ident => "identifier".into(),
        R::expr => "expression".into(),
        R::cond => "condition".into(),
        r => format!("{:?}", r),
    }
}
