use super::hint;

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub enum RuleCode {
    Unsafe,
    UseUnsafe,
    Unwrap,
    Expect,
    IndexExpression,
}

impl ToString for RuleCode {
    fn to_string(&self) -> String {
        format!("{:?}", self)
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Rule {
    pub code: RuleCode,
    pub desc: &'static str,
    pub hint: Option<&'static str>,
}

pub static RULE_UNSAFE_CODE: Rule = Rule {
    code: RuleCode::Unsafe,
    desc: "Unsafe keyword is forbidden.",
    hint: None,
};
pub static RULE_USE_UNSAFE: Rule = Rule {
    code: RuleCode::UseUnsafe,
    desc: "using unsafe identifier like function or macro is forbidden.",
    hint: Some(hint::UNSAFE_HINT),
};
pub static RULE_UNWRAP_CALL: Rule = Rule {
    code: RuleCode::Unwrap,
    desc: "Unwrap call may panic.",
    hint: Some(hint::FUNCTOR_HINT),
};
pub static RULE_EXPECT_CALL: Rule = Rule {
    code: RuleCode::Expect,
    desc: "expect call may panic.",
    hint: Some(hint::FUNCTOR_HINT),
};
pub static RULE_INDEX_EXPRESSION: Rule = Rule {
    code: RuleCode::IndexExpression,
    desc: "index operation may panic, use get method instead.",
    hint: Some(hint::INDEX_EXPR_HINT),
};

pub static RULES: [Rule; 5] = [
    RULE_UNSAFE_CODE,
    RULE_USE_UNSAFE,
    RULE_UNWRAP_CALL,
    RULE_EXPECT_CALL,
    RULE_INDEX_EXPRESSION,
];
