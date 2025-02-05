// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use parol_runtime::id_tree::Tree;
use parol_runtime::once_cell::sync::Lazy;
#[allow(unused_imports)]
use parol_runtime::parser::{
    DFATransition, LLKParser, LookaheadDFA, ParseTreeType, ParseType, Production,
};
use parol_runtime::ParolError;
use parol_runtime::{TokenStream, Tokenizer};
use std::cell::RefCell;
use std::path::Path;

use crate::calc_grammar::CalcGrammar;
use crate::calc_grammar_trait::CalcGrammarAuto;

use parol_runtime::lexer::tokenizer::{
    ERROR_TOKEN, NEW_LINE_TOKEN, UNMATCHABLE_TOKEN, WHITESPACE_TOKEN,
};

pub const TERMINALS: &[&str; 23] = &[
    /*  0 */ UNMATCHABLE_TOKEN,
    /*  1 */ UNMATCHABLE_TOKEN,
    /*  2 */ UNMATCHABLE_TOKEN,
    /*  3 */ UNMATCHABLE_TOKEN,
    /*  4 */ UNMATCHABLE_TOKEN,
    /*  5 */ r###";"###,
    /*  6 */ r###"==|!="###,
    /*  7 */ r###"(\+|-|\*|/|%|<<|>>|&|\^|\|)?="###,
    /*  8 */ r###"\|\|"###,
    /*  9 */ r###"&&"###,
    /* 10 */ r###"\|"###,
    /* 11 */ r###"&"###,
    /* 12 */ r###"<<|>>"###,
    /* 13 */ r###"<=|<|>=|>"###,
    /* 14 */ r###"\+"###,
    /* 15 */ r###"-"###,
    /* 16 */ r###"\*\*"###,
    /* 17 */ r###"\*|/|%"###,
    /* 18 */ r###"\("###,
    /* 19 */ r###"\)"###,
    /* 20 */ r###"0|[1-9][0-9]*"###,
    /* 21 */ r###"[a-zA-Z_][a-zA-Z0-9_]*"###,
    /* 22 */ ERROR_TOKEN,
];

pub const TERMINAL_NAMES: &[&str; 23] = &[
    /*  0 */ "EndOfInput",
    /*  1 */ "Newline",
    /*  2 */ "Whitespace",
    /*  3 */ "LineComment",
    /*  4 */ "BlockComment",
    /*  5 */ "Semicolon",
    /*  6 */ "EqualityOp",
    /*  7 */ "AssignOp",
    /*  8 */ "LogicalOrOp",
    /*  9 */ "LogicalAndOp",
    /* 10 */ "BitwiseOrOp",
    /* 11 */ "BitwiseAndOp",
    /* 12 */ "BitwiseShiftOp",
    /* 13 */ "RelationalOp",
    /* 14 */ "Plus",
    /* 15 */ "Minus",
    /* 16 */ "PowOp",
    /* 17 */ "MultOp",
    /* 18 */ "LParen",
    /* 19 */ "RParen",
    /* 20 */ "Number",
    /* 21 */ "Id",
    /* 22 */ "Error",
];

/* SCANNER_0: "INITIAL" */
const SCANNER_0: (&[&str; 5], &[usize; 17]) = (
    &[
        /*  0 */ UNMATCHABLE_TOKEN,
        /*  1 */ NEW_LINE_TOKEN,
        /*  2 */ WHITESPACE_TOKEN,
        /*  3 */ r###"(//.*(\r\n|\r|\n|$))"###,
        /*  4 */ r###"((?ms)/\*.*?\*/)"###,
    ],
    &[
        5,  /* Semicolon */
        6,  /* EqualityOp */
        7,  /* AssignOp */
        8,  /* LogicalOrOp */
        9,  /* LogicalAndOp */
        10, /* BitwiseOrOp */
        11, /* BitwiseAndOp */
        12, /* BitwiseShiftOp */
        13, /* RelationalOp */
        14, /* Plus */
        15, /* Minus */
        16, /* PowOp */
        17, /* MultOp */
        18, /* LParen */
        19, /* RParen */
        20, /* Number */
        21, /* Id */
    ],
);

const MAX_K: usize = 2;

pub const NON_TERMINALS: &[&str; 44] = &[
    /*  0 */ "AddOp",
    /*  1 */ "AssignItem",
    /*  2 */ "AssignOp",
    /*  3 */ "Assignment",
    /*  4 */ "AssignmentList",
    /*  5 */ "BitwiseAnd",
    /*  6 */ "BitwiseAndList",
    /*  7 */ "BitwiseAndOp",
    /*  8 */ "BitwiseOr",
    /*  9 */ "BitwiseOrList",
    /* 10 */ "BitwiseOrOp",
    /* 11 */ "BitwiseShift",
    /* 12 */ "BitwiseShiftList",
    /* 13 */ "BitwiseShiftOp",
    /* 14 */ "Calc",
    /* 15 */ "CalcList",
    /* 16 */ "Equality",
    /* 17 */ "EqualityList",
    /* 18 */ "EqualityOp",
    /* 19 */ "Factor",
    /* 20 */ "Id",
    /* 21 */ "IdRef",
    /* 22 */ "Instruction",
    /* 23 */ "LogicalAnd",
    /* 24 */ "LogicalAndList",
    /* 25 */ "LogicalAndOp",
    /* 26 */ "LogicalOr",
    /* 27 */ "LogicalOrList",
    /* 28 */ "LogicalOrOp",
    /* 29 */ "Minus",
    /* 30 */ "Mult",
    /* 31 */ "MultList",
    /* 32 */ "MultOp",
    /* 33 */ "Negate",
    /* 34 */ "Number",
    /* 35 */ "Plus",
    /* 36 */ "PowOp",
    /* 37 */ "Power",
    /* 38 */ "PowerList",
    /* 39 */ "Relational",
    /* 40 */ "RelationalList",
    /* 41 */ "RelationalOp",
    /* 42 */ "Summ",
    /* 43 */ "SummList",
];

pub const LOOKAHEAD_AUTOMATA: &[LookaheadDFA; 44] = &[
    /* 0 - "AddOp" */
    LookaheadDFA {
        states: &[None, Some(42), Some(43)],
        transitions: &[DFATransition(0, 14, 1), DFATransition(0, 15, 2)],
        k: 1,
    },
    /* 1 - "AssignItem" */
    LookaheadDFA {
        states: &[Some(17)],
        transitions: &[],
        k: 0,
    },
    /* 2 - "AssignOp" */
    LookaheadDFA {
        states: &[Some(4)],
        transitions: &[],
        k: 0,
    },
    /* 3 - "Assignment" */
    LookaheadDFA {
        states: &[Some(18)],
        transitions: &[],
        k: 0,
    },
    /* 4 - "AssignmentList" */
    LookaheadDFA {
        states: &[None, None, Some(19), None, None, None, Some(20)],
        transitions: &[
            DFATransition(0, 15, 3),
            DFATransition(0, 18, 4),
            DFATransition(0, 20, 5),
            DFATransition(0, 21, 1),
            DFATransition(1, 5, 6),
            DFATransition(1, 6, 6),
            DFATransition(1, 7, 2),
            DFATransition(1, 8, 6),
            DFATransition(1, 9, 6),
            DFATransition(1, 10, 6),
            DFATransition(1, 11, 6),
            DFATransition(1, 12, 6),
            DFATransition(1, 13, 6),
            DFATransition(1, 14, 6),
            DFATransition(1, 15, 6),
            DFATransition(1, 16, 6),
            DFATransition(1, 17, 6),
            DFATransition(3, 15, 6),
            DFATransition(3, 18, 6),
            DFATransition(3, 20, 6),
            DFATransition(3, 21, 6),
            DFATransition(4, 15, 6),
            DFATransition(4, 18, 6),
            DFATransition(4, 20, 6),
            DFATransition(4, 21, 6),
            DFATransition(5, 5, 6),
            DFATransition(5, 6, 6),
            DFATransition(5, 8, 6),
            DFATransition(5, 9, 6),
            DFATransition(5, 10, 6),
            DFATransition(5, 11, 6),
            DFATransition(5, 12, 6),
            DFATransition(5, 13, 6),
            DFATransition(5, 14, 6),
            DFATransition(5, 15, 6),
            DFATransition(5, 16, 6),
            DFATransition(5, 17, 6),
        ],
        k: 2,
    },
    /* 5 - "BitwiseAnd" */
    LookaheadDFA {
        states: &[Some(30)],
        transitions: &[],
        k: 0,
    },
    /* 6 - "BitwiseAndList" */
    LookaheadDFA {
        states: &[None, Some(31), Some(32)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 7 - "BitwiseAndOp" */
    LookaheadDFA {
        states: &[Some(8)],
        transitions: &[],
        k: 0,
    },
    /* 8 - "BitwiseOr" */
    LookaheadDFA {
        states: &[Some(27)],
        transitions: &[],
        k: 0,
    },
    /* 9 - "BitwiseOrList" */
    LookaheadDFA {
        states: &[None, Some(28), Some(29)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 10 - "BitwiseOrOp" */
    LookaheadDFA {
        states: &[Some(7)],
        transitions: &[],
        k: 0,
    },
    /* 11 - "BitwiseShift" */
    LookaheadDFA {
        states: &[Some(39)],
        transitions: &[],
        k: 0,
    },
    /* 12 - "BitwiseShiftList" */
    LookaheadDFA {
        states: &[None, Some(40), Some(41)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 12, 1),
            DFATransition(0, 13, 2),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 13 - "BitwiseShiftOp" */
    LookaheadDFA {
        states: &[Some(9)],
        transitions: &[],
        k: 0,
    },
    /* 14 - "Calc" */
    LookaheadDFA {
        states: &[Some(0)],
        transitions: &[],
        k: 0,
    },
    /* 15 - "CalcList" */
    LookaheadDFA {
        states: &[None, Some(1), Some(2)],
        transitions: &[
            DFATransition(0, 0, 2),
            DFATransition(0, 15, 1),
            DFATransition(0, 18, 1),
            DFATransition(0, 20, 1),
            DFATransition(0, 21, 1),
        ],
        k: 1,
    },
    /* 16 - "Equality" */
    LookaheadDFA {
        states: &[Some(33)],
        transitions: &[],
        k: 0,
    },
    /* 17 - "EqualityList" */
    LookaheadDFA {
        states: &[None, Some(34), Some(35)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 1),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 18 - "EqualityOp" */
    LookaheadDFA {
        states: &[Some(3)],
        transitions: &[],
        k: 0,
    },
    /* 19 - "Factor" */
    LookaheadDFA {
        states: &[None, Some(54), Some(55), Some(56), Some(57)],
        transitions: &[
            DFATransition(0, 15, 3),
            DFATransition(0, 18, 4),
            DFATransition(0, 20, 1),
            DFATransition(0, 21, 2),
        ],
        k: 1,
    },
    /* 20 - "Id" */
    LookaheadDFA {
        states: &[Some(60)],
        transitions: &[],
        k: 0,
    },
    /* 21 - "IdRef" */
    LookaheadDFA {
        states: &[Some(59)],
        transitions: &[],
        k: 0,
    },
    /* 22 - "Instruction" */
    LookaheadDFA {
        states: &[None, None, Some(15), None, None, None, Some(16)],
        transitions: &[
            DFATransition(0, 15, 3),
            DFATransition(0, 18, 4),
            DFATransition(0, 20, 5),
            DFATransition(0, 21, 1),
            DFATransition(1, 5, 6),
            DFATransition(1, 6, 6),
            DFATransition(1, 7, 2),
            DFATransition(1, 8, 6),
            DFATransition(1, 9, 6),
            DFATransition(1, 10, 6),
            DFATransition(1, 11, 6),
            DFATransition(1, 12, 6),
            DFATransition(1, 13, 6),
            DFATransition(1, 14, 6),
            DFATransition(1, 15, 6),
            DFATransition(1, 16, 6),
            DFATransition(1, 17, 6),
            DFATransition(3, 15, 6),
            DFATransition(3, 18, 6),
            DFATransition(3, 20, 6),
            DFATransition(3, 21, 6),
            DFATransition(4, 15, 6),
            DFATransition(4, 18, 6),
            DFATransition(4, 20, 6),
            DFATransition(4, 21, 6),
            DFATransition(5, 5, 6),
            DFATransition(5, 6, 6),
            DFATransition(5, 8, 6),
            DFATransition(5, 9, 6),
            DFATransition(5, 10, 6),
            DFATransition(5, 11, 6),
            DFATransition(5, 12, 6),
            DFATransition(5, 13, 6),
            DFATransition(5, 14, 6),
            DFATransition(5, 15, 6),
            DFATransition(5, 16, 6),
            DFATransition(5, 17, 6),
        ],
        k: 2,
    },
    /* 23 - "LogicalAnd" */
    LookaheadDFA {
        states: &[Some(24)],
        transitions: &[],
        k: 0,
    },
    /* 24 - "LogicalAndList" */
    LookaheadDFA {
        states: &[None, Some(25), Some(26)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 25 - "LogicalAndOp" */
    LookaheadDFA {
        states: &[Some(6)],
        transitions: &[],
        k: 0,
    },
    /* 26 - "LogicalOr" */
    LookaheadDFA {
        states: &[Some(21)],
        transitions: &[],
        k: 0,
    },
    /* 27 - "LogicalOrList" */
    LookaheadDFA {
        states: &[None, Some(22), Some(23)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 8, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 28 - "LogicalOrOp" */
    LookaheadDFA {
        states: &[Some(5)],
        transitions: &[],
        k: 0,
    },
    /* 29 - "Minus" */
    LookaheadDFA {
        states: &[Some(12)],
        transitions: &[],
        k: 0,
    },
    /* 30 - "Mult" */
    LookaheadDFA {
        states: &[Some(47)],
        transitions: &[],
        k: 0,
    },
    /* 31 - "MultList" */
    LookaheadDFA {
        states: &[None, Some(48), Some(49)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 12, 2),
            DFATransition(0, 13, 2),
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 2),
            DFATransition(0, 17, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 32 - "MultOp" */
    LookaheadDFA {
        states: &[Some(14)],
        transitions: &[],
        k: 0,
    },
    /* 33 - "Negate" */
    LookaheadDFA {
        states: &[Some(53)],
        transitions: &[],
        k: 0,
    },
    /* 34 - "Number" */
    LookaheadDFA {
        states: &[Some(58)],
        transitions: &[],
        k: 0,
    },
    /* 35 - "Plus" */
    LookaheadDFA {
        states: &[Some(11)],
        transitions: &[],
        k: 0,
    },
    /* 36 - "PowOp" */
    LookaheadDFA {
        states: &[Some(13)],
        transitions: &[],
        k: 0,
    },
    /* 37 - "Power" */
    LookaheadDFA {
        states: &[Some(50)],
        transitions: &[],
        k: 0,
    },
    /* 38 - "PowerList" */
    LookaheadDFA {
        states: &[None, Some(51), Some(52)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 12, 2),
            DFATransition(0, 13, 2),
            DFATransition(0, 14, 2),
            DFATransition(0, 15, 2),
            DFATransition(0, 16, 1),
            DFATransition(0, 17, 2),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 39 - "Relational" */
    LookaheadDFA {
        states: &[Some(36)],
        transitions: &[],
        k: 0,
    },
    /* 40 - "RelationalList" */
    LookaheadDFA {
        states: &[None, Some(37), Some(38)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 13, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
    /* 41 - "RelationalOp" */
    LookaheadDFA {
        states: &[Some(10)],
        transitions: &[],
        k: 0,
    },
    /* 42 - "Summ" */
    LookaheadDFA {
        states: &[Some(44)],
        transitions: &[],
        k: 0,
    },
    /* 43 - "SummList" */
    LookaheadDFA {
        states: &[None, Some(45), Some(46)],
        transitions: &[
            DFATransition(0, 5, 2),
            DFATransition(0, 6, 2),
            DFATransition(0, 8, 2),
            DFATransition(0, 9, 2),
            DFATransition(0, 10, 2),
            DFATransition(0, 11, 2),
            DFATransition(0, 12, 2),
            DFATransition(0, 13, 2),
            DFATransition(0, 14, 1),
            DFATransition(0, 15, 1),
            DFATransition(0, 19, 2),
        ],
        k: 1,
    },
];

pub const PRODUCTIONS: &[Production; 61] = &[
    // 0 - Calc: CalcList /* Vec */;
    Production {
        lhs: 14,
        production: &[ParseType::N(15)],
    },
    // 1 - CalcList: Instruction ";"^ /* Clipped */ CalcList;
    Production {
        lhs: 15,
        production: &[ParseType::N(15), ParseType::T(5), ParseType::N(22)],
    },
    // 2 - CalcList: ;
    Production {
        lhs: 15,
        production: &[],
    },
    // 3 - EqualityOp: "==|!=";
    Production {
        lhs: 18,
        production: &[ParseType::T(6)],
    },
    // 4 - AssignOp: "(\+|-|\*|/|%|<<|>>|&|\^|\|)?=";
    Production {
        lhs: 2,
        production: &[ParseType::T(7)],
    },
    // 5 - LogicalOrOp: "\|\|";
    Production {
        lhs: 28,
        production: &[ParseType::T(8)],
    },
    // 6 - LogicalAndOp: "&&";
    Production {
        lhs: 25,
        production: &[ParseType::T(9)],
    },
    // 7 - BitwiseOrOp: "\|";
    Production {
        lhs: 10,
        production: &[ParseType::T(10)],
    },
    // 8 - BitwiseAndOp: "&";
    Production {
        lhs: 7,
        production: &[ParseType::T(11)],
    },
    // 9 - BitwiseShiftOp: "<<|>>";
    Production {
        lhs: 13,
        production: &[ParseType::T(12)],
    },
    // 10 - RelationalOp: "<=|<|>=|>";
    Production {
        lhs: 41,
        production: &[ParseType::T(13)],
    },
    // 11 - Plus: "\+";
    Production {
        lhs: 35,
        production: &[ParseType::T(14)],
    },
    // 12 - Minus: "-";
    Production {
        lhs: 29,
        production: &[ParseType::T(15)],
    },
    // 13 - PowOp: "\*\*";
    Production {
        lhs: 36,
        production: &[ParseType::T(16)],
    },
    // 14 - MultOp: "\*|/|%";
    Production {
        lhs: 32,
        production: &[ParseType::T(17)],
    },
    // 15 - Instruction: Assignment;
    Production {
        lhs: 22,
        production: &[ParseType::N(3)],
    },
    // 16 - Instruction: LogicalOr;
    Production {
        lhs: 22,
        production: &[ParseType::N(26)],
    },
    // 17 - AssignItem: Id AssignOp;
    Production {
        lhs: 1,
        production: &[ParseType::N(2), ParseType::N(20)],
    },
    // 18 - Assignment: AssignItem AssignmentList /* Vec */ LogicalOr;
    Production {
        lhs: 3,
        production: &[ParseType::N(26), ParseType::N(4), ParseType::N(1)],
    },
    // 19 - AssignmentList: AssignItem AssignmentList;
    Production {
        lhs: 4,
        production: &[ParseType::N(4), ParseType::N(1)],
    },
    // 20 - AssignmentList: ;
    Production {
        lhs: 4,
        production: &[],
    },
    // 21 - LogicalOr: LogicalAnd LogicalOrList /* Vec */;
    Production {
        lhs: 26,
        production: &[ParseType::N(27), ParseType::N(23)],
    },
    // 22 - LogicalOrList: LogicalOrOp LogicalAnd LogicalOrList;
    Production {
        lhs: 27,
        production: &[ParseType::N(27), ParseType::N(23), ParseType::N(28)],
    },
    // 23 - LogicalOrList: ;
    Production {
        lhs: 27,
        production: &[],
    },
    // 24 - LogicalAnd: BitwiseOr LogicalAndList /* Vec */;
    Production {
        lhs: 23,
        production: &[ParseType::N(24), ParseType::N(8)],
    },
    // 25 - LogicalAndList: LogicalAndOp BitwiseOr LogicalAndList;
    Production {
        lhs: 24,
        production: &[ParseType::N(24), ParseType::N(8), ParseType::N(25)],
    },
    // 26 - LogicalAndList: ;
    Production {
        lhs: 24,
        production: &[],
    },
    // 27 - BitwiseOr: BitwiseAnd BitwiseOrList /* Vec */;
    Production {
        lhs: 8,
        production: &[ParseType::N(9), ParseType::N(5)],
    },
    // 28 - BitwiseOrList: BitwiseOrOp BitwiseAnd BitwiseOrList;
    Production {
        lhs: 9,
        production: &[ParseType::N(9), ParseType::N(5), ParseType::N(10)],
    },
    // 29 - BitwiseOrList: ;
    Production {
        lhs: 9,
        production: &[],
    },
    // 30 - BitwiseAnd: Equality BitwiseAndList /* Vec */;
    Production {
        lhs: 5,
        production: &[ParseType::N(6), ParseType::N(16)],
    },
    // 31 - BitwiseAndList: BitwiseAndOp Equality BitwiseAndList;
    Production {
        lhs: 6,
        production: &[ParseType::N(6), ParseType::N(16), ParseType::N(7)],
    },
    // 32 - BitwiseAndList: ;
    Production {
        lhs: 6,
        production: &[],
    },
    // 33 - Equality: Relational EqualityList /* Vec */;
    Production {
        lhs: 16,
        production: &[ParseType::N(17), ParseType::N(39)],
    },
    // 34 - EqualityList: EqualityOp Relational EqualityList;
    Production {
        lhs: 17,
        production: &[ParseType::N(17), ParseType::N(39), ParseType::N(18)],
    },
    // 35 - EqualityList: ;
    Production {
        lhs: 17,
        production: &[],
    },
    // 36 - Relational: BitwiseShift RelationalList /* Vec */;
    Production {
        lhs: 39,
        production: &[ParseType::N(40), ParseType::N(11)],
    },
    // 37 - RelationalList: RelationalOp BitwiseShift RelationalList;
    Production {
        lhs: 40,
        production: &[ParseType::N(40), ParseType::N(11), ParseType::N(41)],
    },
    // 38 - RelationalList: ;
    Production {
        lhs: 40,
        production: &[],
    },
    // 39 - BitwiseShift: Summ BitwiseShiftList /* Vec */;
    Production {
        lhs: 11,
        production: &[ParseType::N(12), ParseType::N(42)],
    },
    // 40 - BitwiseShiftList: BitwiseShiftOp Summ BitwiseShiftList;
    Production {
        lhs: 12,
        production: &[ParseType::N(12), ParseType::N(42), ParseType::N(13)],
    },
    // 41 - BitwiseShiftList: ;
    Production {
        lhs: 12,
        production: &[],
    },
    // 42 - AddOp: Plus;
    Production {
        lhs: 0,
        production: &[ParseType::N(35)],
    },
    // 43 - AddOp: Minus;
    Production {
        lhs: 0,
        production: &[ParseType::N(29)],
    },
    // 44 - Summ: Mult SummList /* Vec */;
    Production {
        lhs: 42,
        production: &[ParseType::N(43), ParseType::N(30)],
    },
    // 45 - SummList: AddOp Mult SummList;
    Production {
        lhs: 43,
        production: &[ParseType::N(43), ParseType::N(30), ParseType::N(0)],
    },
    // 46 - SummList: ;
    Production {
        lhs: 43,
        production: &[],
    },
    // 47 - Mult: Power MultList /* Vec */;
    Production {
        lhs: 30,
        production: &[ParseType::N(31), ParseType::N(37)],
    },
    // 48 - MultList: MultOp Power MultList;
    Production {
        lhs: 31,
        production: &[ParseType::N(31), ParseType::N(37), ParseType::N(32)],
    },
    // 49 - MultList: ;
    Production {
        lhs: 31,
        production: &[],
    },
    // 50 - Power: Factor PowerList /* Vec */;
    Production {
        lhs: 37,
        production: &[ParseType::N(38), ParseType::N(19)],
    },
    // 51 - PowerList: PowOp Factor PowerList;
    Production {
        lhs: 38,
        production: &[ParseType::N(38), ParseType::N(19), ParseType::N(36)],
    },
    // 52 - PowerList: ;
    Production {
        lhs: 38,
        production: &[],
    },
    // 53 - Negate: Minus;
    Production {
        lhs: 33,
        production: &[ParseType::N(29)],
    },
    // 54 - Factor: Number;
    Production {
        lhs: 19,
        production: &[ParseType::N(34)],
    },
    // 55 - Factor: IdRef;
    Production {
        lhs: 19,
        production: &[ParseType::N(21)],
    },
    // 56 - Factor: Negate Factor;
    Production {
        lhs: 19,
        production: &[ParseType::N(19), ParseType::N(33)],
    },
    // 57 - Factor: "\("^ /* Clipped */ LogicalOr "\)"^ /* Clipped */;
    Production {
        lhs: 19,
        production: &[ParseType::T(19), ParseType::N(26), ParseType::T(18)],
    },
    // 58 - Number: "0|[1-9][0-9]*";
    Production {
        lhs: 34,
        production: &[ParseType::T(20)],
    },
    // 59 - IdRef: Id;
    Production {
        lhs: 21,
        production: &[ParseType::N(20)],
    },
    // 60 - Id: "[a-zA-Z_][a-zA-Z0-9_]*";
    Production {
        lhs: 20,
        production: &[ParseType::T(21)],
    },
];

static TOKENIZERS: Lazy<Vec<(&'static str, Tokenizer)>> = Lazy::new(|| {
    vec![(
        "INITIAL",
        Tokenizer::build(TERMINALS, SCANNER_0.0, SCANNER_0.1).unwrap(),
    )]
});

pub fn parse<'t, T>(
    input: &'t str,
    file_name: T,
    user_actions: &mut CalcGrammar<'t>,
) -> Result<Tree<ParseTreeType<'t>>, ParolError>
where
    T: AsRef<Path>,
{
    let mut llk_parser = LLKParser::new(
        14,
        LOOKAHEAD_AUTOMATA,
        PRODUCTIONS,
        TERMINAL_NAMES,
        NON_TERMINALS,
    );
    let token_stream =
        RefCell::new(TokenStream::new(input, file_name, &TOKENIZERS, MAX_K).unwrap());
    // Initialize wrapper
    let mut user_actions = CalcGrammarAuto::new(user_actions);
    let result = llk_parser.parse(token_stream, &mut user_actions);
    match result {
        Ok(()) => Ok(llk_parser.parse_tree),
        Err(e) => Err(e),
    }
}
