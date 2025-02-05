// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

// Disable clippy warnings that can result in the way how parol generates code.
#![allow(clippy::enum_variant_names)]
#![allow(clippy::large_enum_variant)]
#![allow(clippy::upper_case_acronyms)]

use parol_runtime::id_tree::Tree;

use crate::calc_grammar::CalcGrammar;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, UserActionsTrait};
use parol_runtime::{ParserError, Result};

///
/// The `CalcGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait CalcGrammarTrait {
    /// Semantic action for production 0:
    ///
    /// Calc: CalcLst1;
    ///
    fn calc(
        &mut self,
        _calc_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// CalcLst1: CalcLst1Itm1 CalcLst1;
    ///
    fn calc_lst1_0(
        &mut self,
        _calc_lst1_itm1: &ParseTreeStackEntry,
        _calc_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// CalcLst1Itm1: Instruction ";";
    ///
    fn calc_lst1_itm1(
        &mut self,
        _instruction: &ParseTreeStackEntry,
        _semicolon: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// CalcLst1: ;
    ///
    fn calc_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// Instruction: Assignment;
    ///
    fn instruction_0(
        &mut self,
        _assignment: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// Instruction: LogicalOr;
    ///
    fn instruction_1(
        &mut self,
        _logical_or: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// EqualityOp: "==|!=";
    ///
    fn equality_op(
        &mut self,
        _equality_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// AssignOp: "(\+|-|\*|/|%|<<|>>|&|\^|\|)?=";
    ///
    fn assign_op(
        &mut self,
        _assign_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// AssignItem: Id AssignOp;
    ///
    fn assign_item(
        &mut self,
        _id: &ParseTreeStackEntry,
        _assign_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// Assignment: AssignItem AssignmentLst1 LogicalOr;
    ///
    fn assignment(
        &mut self,
        _assign_item: &ParseTreeStackEntry,
        _assignment_lst1: &ParseTreeStackEntry,
        _logical_or: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// AssignmentLst1: AssignmentLst1Itm1 AssignmentLst1;
    ///
    fn assignment_lst1_0(
        &mut self,
        _assignment_lst1_itm1: &ParseTreeStackEntry,
        _assignment_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// AssignmentLst1Itm1: AssignItem;
    ///
    fn assignment_lst1_itm1(
        &mut self,
        _assign_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// AssignmentLst1: ;
    ///
    fn assignment_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// LogicalOr: LogicalAnd LogicalOrLst1;
    ///
    fn logical_or(
        &mut self,
        _logical_and: &ParseTreeStackEntry,
        _logical_or_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// LogicalOrLst1: LogicalOrLst1Itm1 LogicalOrLst1;
    ///
    fn logical_or_lst1_0(
        &mut self,
        _logical_or_lst1_itm1: &ParseTreeStackEntry,
        _logical_or_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// LogicalOrLst1Itm1: LogicalOrItem;
    ///
    fn logical_or_lst1_itm1(
        &mut self,
        _logical_or_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// LogicalOrLst1: ;
    ///
    fn logical_or_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// LogicalOrOp: "\|\|";
    ///
    fn logical_or_op(
        &mut self,
        _logical_or_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// LogicalOrItem: LogicalOrOp LogicalAnd;
    ///
    fn logical_or_item(
        &mut self,
        _logical_or_op: &ParseTreeStackEntry,
        _logical_and: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// LogicalAnd: BitwiseOr LogicalAndLst1;
    ///
    fn logical_and(
        &mut self,
        _bitwise_or: &ParseTreeStackEntry,
        _logical_and_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// LogicalAndLst1: LogicalAndLst1Itm1 LogicalAndLst1;
    ///
    fn logical_and_lst1_0(
        &mut self,
        _logical_and_lst1_itm1: &ParseTreeStackEntry,
        _logical_and_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 21:
    ///
    /// LogicalAndLst1Itm1: LogicalAndItem;
    ///
    fn logical_and_lst1_itm1(
        &mut self,
        _logical_and_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 22:
    ///
    /// LogicalAndLst1: ;
    ///
    fn logical_and_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 23:
    ///
    /// LogicalAndOp: "&&";
    ///
    fn logical_and_op(
        &mut self,
        _logical_and_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 24:
    ///
    /// LogicalAndItem: LogicalAndOp BitwiseOr;
    ///
    fn logical_and_item(
        &mut self,
        _logical_and_op: &ParseTreeStackEntry,
        _bitwise_or: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 25:
    ///
    /// BitwiseOr: BitwiseAnd BitwiseOrLst1;
    ///
    fn bitwise_or(
        &mut self,
        _bitwise_and: &ParseTreeStackEntry,
        _bitwise_or_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 26:
    ///
    /// BitwiseOrLst1: BitwiseOrLst1Itm1 BitwiseOrLst1;
    ///
    fn bitwise_or_lst1_0(
        &mut self,
        _bitwise_or_lst1_itm1: &ParseTreeStackEntry,
        _bitwise_or_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 27:
    ///
    /// BitwiseOrLst1Itm1: BitwiseOrItem;
    ///
    fn bitwise_or_lst1_itm1(
        &mut self,
        _bitwise_or_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 28:
    ///
    /// BitwiseOrLst1: ;
    ///
    fn bitwise_or_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 29:
    ///
    /// BitwiseOrOp: "\|";
    ///
    fn bitwise_or_op(
        &mut self,
        _bitwise_or_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 30:
    ///
    /// BitwiseOrItem: BitwiseOrOp BitwiseAnd;
    ///
    fn bitwise_or_item(
        &mut self,
        _bitwise_or_op: &ParseTreeStackEntry,
        _bitwise_and: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 31:
    ///
    /// BitwiseAnd: Equality BitwiseAndLst1;
    ///
    fn bitwise_and(
        &mut self,
        _equality: &ParseTreeStackEntry,
        _bitwise_and_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 32:
    ///
    /// BitwiseAndLst1: BitwiseAndLst1Itm1 BitwiseAndLst1;
    ///
    fn bitwise_and_lst1_0(
        &mut self,
        _bitwise_and_lst1_itm1: &ParseTreeStackEntry,
        _bitwise_and_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 33:
    ///
    /// BitwiseAndLst1Itm1: BitwiseAndItem;
    ///
    fn bitwise_and_lst1_itm1(
        &mut self,
        _bitwise_and_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 34:
    ///
    /// BitwiseAndLst1: ;
    ///
    fn bitwise_and_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 35:
    ///
    /// BitwiseAndOp: "&";
    ///
    fn bitwise_and_op(
        &mut self,
        _bitwise_and_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 36:
    ///
    /// BitwiseAndItem: BitwiseAndOp Equality;
    ///
    fn bitwise_and_item(
        &mut self,
        _bitwise_and_op: &ParseTreeStackEntry,
        _equality: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 37:
    ///
    /// Equality: Relational EqualityLst1;
    ///
    fn equality(
        &mut self,
        _relational: &ParseTreeStackEntry,
        _equality_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 38:
    ///
    /// EqualityLst1: EqualityLst1Itm1 EqualityLst1;
    ///
    fn equality_lst1_0(
        &mut self,
        _equality_lst1_itm1: &ParseTreeStackEntry,
        _equality_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 39:
    ///
    /// EqualityLst1Itm1: EqualityItem;
    ///
    fn equality_lst1_itm1(
        &mut self,
        _equality_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 40:
    ///
    /// EqualityLst1: ;
    ///
    fn equality_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 41:
    ///
    /// EqualityItem: EqualityOp Relational;
    ///
    fn equality_item(
        &mut self,
        _equality_op: &ParseTreeStackEntry,
        _relational: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 42:
    ///
    /// BitwiseShiftOp: "<<|>>";
    ///
    fn bitwise_shift_op(
        &mut self,
        _bitwise_shift_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 43:
    ///
    /// Relational: BitwiseShift RelationalLst1;
    ///
    fn relational(
        &mut self,
        _bitwise_shift: &ParseTreeStackEntry,
        _relational_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 44:
    ///
    /// RelationalLst1: RelationalLst1Itm1 RelationalLst1;
    ///
    fn relational_lst1_0(
        &mut self,
        _relational_lst1_itm1: &ParseTreeStackEntry,
        _relational_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 45:
    ///
    /// RelationalLst1Itm1: RelationalItem;
    ///
    fn relational_lst1_itm1(
        &mut self,
        _relational_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 46:
    ///
    /// RelationalLst1: ;
    ///
    fn relational_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 47:
    ///
    /// RelationalOp: "<=|<|>=|>";
    ///
    fn relational_op(
        &mut self,
        _relational_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 48:
    ///
    /// RelationalItem: RelationalOp BitwiseShift;
    ///
    fn relational_item(
        &mut self,
        _relational_op: &ParseTreeStackEntry,
        _bitwise_shift: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 49:
    ///
    /// BitwiseShift: Summ BitwiseShiftLst1;
    ///
    fn bitwise_shift(
        &mut self,
        _summ: &ParseTreeStackEntry,
        _bitwise_shift_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 50:
    ///
    /// BitwiseShiftLst1: BitwiseShiftLst1Itm1 BitwiseShiftLst1;
    ///
    fn bitwise_shift_lst1_0(
        &mut self,
        _bitwise_shift_lst1_itm1: &ParseTreeStackEntry,
        _bitwise_shift_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 51:
    ///
    /// BitwiseShiftLst1Itm1: BitwiseShiftItem;
    ///
    fn bitwise_shift_lst1_itm1(
        &mut self,
        _bitwise_shift_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 52:
    ///
    /// BitwiseShiftLst1: ;
    ///
    fn bitwise_shift_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 53:
    ///
    /// BitwiseShiftItem: BitwiseShiftOp Summ;
    ///
    fn bitwise_shift_item(
        &mut self,
        _bitwise_shift_op: &ParseTreeStackEntry,
        _summ: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 54:
    ///
    /// Summ: Mult SummLst1;
    ///
    fn summ(
        &mut self,
        _mult: &ParseTreeStackEntry,
        _summ_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 55:
    ///
    /// SummLst1: SummLst1Itm1 SummLst1;
    ///
    fn summ_lst1_0(
        &mut self,
        _summ_lst1_itm1: &ParseTreeStackEntry,
        _summ_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 56:
    ///
    /// SummLst1Itm1: SummItem;
    ///
    fn summ_lst1_itm1(
        &mut self,
        _summ_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 57:
    ///
    /// SummLst1: ;
    ///
    fn summ_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 58:
    ///
    /// Plus: "\+";
    ///
    fn plus(
        &mut self,
        _plus: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 59:
    ///
    /// Minus: "-";
    ///
    fn minus(
        &mut self,
        _minus: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 60:
    ///
    /// AddOp: Plus;
    ///
    fn add_op_0(
        &mut self,
        _plus: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 61:
    ///
    /// AddOp: Minus;
    ///
    fn add_op_1(
        &mut self,
        _minus: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 62:
    ///
    /// SummItem: AddOp Mult;
    ///
    fn summ_item(
        &mut self,
        _add_op: &ParseTreeStackEntry,
        _mult: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 63:
    ///
    /// PowOp: "\*\*";
    ///
    fn pow_op(
        &mut self,
        _pow_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 64:
    ///
    /// Mult: Power MultLst1;
    ///
    fn mult(
        &mut self,
        _power: &ParseTreeStackEntry,
        _mult_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 65:
    ///
    /// MultLst1: MultLst1Itm1 MultLst1;
    ///
    fn mult_lst1_0(
        &mut self,
        _mult_lst1_itm1: &ParseTreeStackEntry,
        _mult_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 66:
    ///
    /// MultLst1Itm1: MultItem;
    ///
    fn mult_lst1_itm1(
        &mut self,
        _mult_item: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 67:
    ///
    /// MultLst1: ;
    ///
    fn mult_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 68:
    ///
    /// MultOp: "\*|/|%";
    ///
    fn mult_op(
        &mut self,
        _mult_op: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 69:
    ///
    /// MultItem: MultOp Power;
    ///
    fn mult_item(
        &mut self,
        _mult_op: &ParseTreeStackEntry,
        _power: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 70:
    ///
    /// Power: Factor PowerLst1;
    ///
    fn power(
        &mut self,
        _factor: &ParseTreeStackEntry,
        _power_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 71:
    ///
    /// PowerLst1: PowerLst1Itm1 PowerLst1;
    ///
    fn power_lst1_0(
        &mut self,
        _power_lst1_itm1: &ParseTreeStackEntry,
        _power_lst1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 72:
    ///
    /// PowerLst1Itm1: PowOp Factor;
    ///
    fn power_lst1_itm1(
        &mut self,
        _pow_op: &ParseTreeStackEntry,
        _factor: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 73:
    ///
    /// PowerLst1: ;
    ///
    fn power_lst1_1(&mut self, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 74:
    ///
    /// Negate: Minus;
    ///
    fn negate(
        &mut self,
        _minus: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 75:
    ///
    /// Factor: Number;
    ///
    fn factor_0(
        &mut self,
        _number: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 76:
    ///
    /// Factor: IdRef;
    ///
    fn factor_1(
        &mut self,
        _id_ref: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 77:
    ///
    /// Factor: Negate Factor;
    ///
    fn factor_2(
        &mut self,
        _negate: &ParseTreeStackEntry,
        _factor: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 78:
    ///
    /// Factor: "\(" LogicalOr "\)";
    ///
    fn factor_3(
        &mut self,
        _l_paren: &ParseTreeStackEntry,
        _logical_or: &ParseTreeStackEntry,
        _r_paren: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 79:
    ///
    /// Number: "0|[1-9][0-9]*";
    ///
    fn number(
        &mut self,
        _number: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 80:
    ///
    /// IdRef: Id;
    ///
    fn id_ref(
        &mut self,
        _id: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 81:
    ///
    /// Id: "[a-zA-Z_][a-zA-Z0-9_]*";
    ///
    fn id(&mut self, _id: &ParseTreeStackEntry, _parse_tree: &Tree<ParseTreeType>) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait<'_> for CalcGrammar {
    ///
    /// This function is implemented automatically for the user's item CalcGrammar.
    ///
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry],
        parse_tree: &Tree<ParseTreeType>,
    ) -> Result<()> {
        match prod_num {
            0 => self.calc(&children[0], parse_tree),
            1 => self.calc_lst1_0(&children[0], &children[1], parse_tree),
            2 => self.calc_lst1_itm1(&children[0], &children[1], parse_tree),
            3 => self.calc_lst1_1(parse_tree),
            4 => self.instruction_0(&children[0], parse_tree),
            5 => self.instruction_1(&children[0], parse_tree),
            6 => self.equality_op(&children[0], parse_tree),
            7 => self.assign_op(&children[0], parse_tree),
            8 => self.assign_item(&children[0], &children[1], parse_tree),
            9 => self.assignment(&children[0], &children[1], &children[2], parse_tree),
            10 => self.assignment_lst1_0(&children[0], &children[1], parse_tree),
            11 => self.assignment_lst1_itm1(&children[0], parse_tree),
            12 => self.assignment_lst1_1(parse_tree),
            13 => self.logical_or(&children[0], &children[1], parse_tree),
            14 => self.logical_or_lst1_0(&children[0], &children[1], parse_tree),
            15 => self.logical_or_lst1_itm1(&children[0], parse_tree),
            16 => self.logical_or_lst1_1(parse_tree),
            17 => self.logical_or_op(&children[0], parse_tree),
            18 => self.logical_or_item(&children[0], &children[1], parse_tree),
            19 => self.logical_and(&children[0], &children[1], parse_tree),
            20 => self.logical_and_lst1_0(&children[0], &children[1], parse_tree),
            21 => self.logical_and_lst1_itm1(&children[0], parse_tree),
            22 => self.logical_and_lst1_1(parse_tree),
            23 => self.logical_and_op(&children[0], parse_tree),
            24 => self.logical_and_item(&children[0], &children[1], parse_tree),
            25 => self.bitwise_or(&children[0], &children[1], parse_tree),
            26 => self.bitwise_or_lst1_0(&children[0], &children[1], parse_tree),
            27 => self.bitwise_or_lst1_itm1(&children[0], parse_tree),
            28 => self.bitwise_or_lst1_1(parse_tree),
            29 => self.bitwise_or_op(&children[0], parse_tree),
            30 => self.bitwise_or_item(&children[0], &children[1], parse_tree),
            31 => self.bitwise_and(&children[0], &children[1], parse_tree),
            32 => self.bitwise_and_lst1_0(&children[0], &children[1], parse_tree),
            33 => self.bitwise_and_lst1_itm1(&children[0], parse_tree),
            34 => self.bitwise_and_lst1_1(parse_tree),
            35 => self.bitwise_and_op(&children[0], parse_tree),
            36 => self.bitwise_and_item(&children[0], &children[1], parse_tree),
            37 => self.equality(&children[0], &children[1], parse_tree),
            38 => self.equality_lst1_0(&children[0], &children[1], parse_tree),
            39 => self.equality_lst1_itm1(&children[0], parse_tree),
            40 => self.equality_lst1_1(parse_tree),
            41 => self.equality_item(&children[0], &children[1], parse_tree),
            42 => self.bitwise_shift_op(&children[0], parse_tree),
            43 => self.relational(&children[0], &children[1], parse_tree),
            44 => self.relational_lst1_0(&children[0], &children[1], parse_tree),
            45 => self.relational_lst1_itm1(&children[0], parse_tree),
            46 => self.relational_lst1_1(parse_tree),
            47 => self.relational_op(&children[0], parse_tree),
            48 => self.relational_item(&children[0], &children[1], parse_tree),
            49 => self.bitwise_shift(&children[0], &children[1], parse_tree),
            50 => self.bitwise_shift_lst1_0(&children[0], &children[1], parse_tree),
            51 => self.bitwise_shift_lst1_itm1(&children[0], parse_tree),
            52 => self.bitwise_shift_lst1_1(parse_tree),
            53 => self.bitwise_shift_item(&children[0], &children[1], parse_tree),
            54 => self.summ(&children[0], &children[1], parse_tree),
            55 => self.summ_lst1_0(&children[0], &children[1], parse_tree),
            56 => self.summ_lst1_itm1(&children[0], parse_tree),
            57 => self.summ_lst1_1(parse_tree),
            58 => self.plus(&children[0], parse_tree),
            59 => self.minus(&children[0], parse_tree),
            60 => self.add_op_0(&children[0], parse_tree),
            61 => self.add_op_1(&children[0], parse_tree),
            62 => self.summ_item(&children[0], &children[1], parse_tree),
            63 => self.pow_op(&children[0], parse_tree),
            64 => self.mult(&children[0], &children[1], parse_tree),
            65 => self.mult_lst1_0(&children[0], &children[1], parse_tree),
            66 => self.mult_lst1_itm1(&children[0], parse_tree),
            67 => self.mult_lst1_1(parse_tree),
            68 => self.mult_op(&children[0], parse_tree),
            69 => self.mult_item(&children[0], &children[1], parse_tree),
            70 => self.power(&children[0], &children[1], parse_tree),
            71 => self.power_lst1_0(&children[0], &children[1], parse_tree),
            72 => self.power_lst1_itm1(&children[0], &children[1], parse_tree),
            73 => self.power_lst1_1(parse_tree),
            74 => self.negate(&children[0], parse_tree),
            75 => self.factor_0(&children[0], parse_tree),
            76 => self.factor_1(&children[0], parse_tree),
            77 => self.factor_2(&children[0], &children[1], parse_tree),
            78 => self.factor_3(&children[0], &children[1], &children[2], parse_tree),
            79 => self.number(&children[0], parse_tree),
            80 => self.id_ref(&children[0], parse_tree),
            81 => self.id(&children[0], parse_tree),
            _ => Err(ParserError::InternalError(format!(
                "Unhandled production number: {}",
                prod_num
            ))
            .into()),
        }
    }
}
