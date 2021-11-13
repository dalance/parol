// ---------------------------------------------------------
// This file was generated by parol.
// It is not intended for manual editing and changes will be
// lost after next build.
// ---------------------------------------------------------

use crate::calc_grammar::CalcGrammar;
use id_tree::Tree;
use parol_runtime::errors::*;
use parol_runtime::parser::{ParseTreeStackEntry, ParseTreeType, ScannerAccess, UserActionsTrait};
use std::cell::RefMut;

///
/// The `CalcGrammarTrait` trait is automatically generated for the
/// given grammar.
/// All functions have default implementations.
///
pub trait CalcGrammarTrait {
    /// Semantic action for production 0:
    ///
    /// calc: calc_lst1;
    ///
    fn calc_0(
        &mut self,
        _calc_lst1_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 1:
    ///
    /// calc_lst1: calc_lst1_itm1 calc_lst1;
    ///
    fn calc_lst1_1(
        &mut self,
        _calc_lst1_itm1_0: &ParseTreeStackEntry,
        _calc_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 2:
    ///
    /// calc_lst1_itm1: instruction ";";
    ///
    fn calc_lst1_itm1_2(
        &mut self,
        _instruction_0: &ParseTreeStackEntry,
        _semicolon_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 3:
    ///
    /// calc_lst1: ;
    ///
    fn calc_lst1_3(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 4:
    ///
    /// instruction: assignment;
    ///
    fn instruction_4(
        &mut self,
        _assignment_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 5:
    ///
    /// instruction: logical_or;
    ///
    fn instruction_5(
        &mut self,
        _logical_or_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 6:
    ///
    /// equality_op: "==|!=";
    ///
    fn equality_op_6(
        &mut self,
        _equality_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 7:
    ///
    /// assign_op: "(\+|-|\*|/|%|<<|>>|&|^|\|)?=";
    ///
    fn assign_op_7(
        &mut self,
        _assign_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 8:
    ///
    /// assign_item: id assign_op;
    ///
    fn assign_item_8(
        &mut self,
        _id_0: &ParseTreeStackEntry,
        _assign_op_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 9:
    ///
    /// assignment: assign_item assignment_lst1 logical_or;
    ///
    fn assignment_9(
        &mut self,
        _assign_item_0: &ParseTreeStackEntry,
        _assignment_lst1_1: &ParseTreeStackEntry,
        _logical_or_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 10:
    ///
    /// assignment_lst1: assignment_lst1_itm1 assignment_lst1;
    ///
    fn assignment_lst1_10(
        &mut self,
        _assignment_lst1_itm1_0: &ParseTreeStackEntry,
        _assignment_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 11:
    ///
    /// assignment_lst1_itm1: assign_item;
    ///
    fn assignment_lst1_itm1_11(
        &mut self,
        _assign_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 12:
    ///
    /// assignment_lst1: ;
    ///
    fn assignment_lst1_12(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 13:
    ///
    /// logical_or: logical_and logical_or_lst1;
    ///
    fn logical_or_13(
        &mut self,
        _logical_and_0: &ParseTreeStackEntry,
        _logical_or_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 14:
    ///
    /// logical_or_lst1: logical_or_lst1_itm1 logical_or_lst1;
    ///
    fn logical_or_lst1_14(
        &mut self,
        _logical_or_lst1_itm1_0: &ParseTreeStackEntry,
        _logical_or_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 15:
    ///
    /// logical_or_lst1_itm1: logical_or_item;
    ///
    fn logical_or_lst1_itm1_15(
        &mut self,
        _logical_or_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 16:
    ///
    /// logical_or_lst1: ;
    ///
    fn logical_or_lst1_16(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 17:
    ///
    /// logical_or_op: "\|\|";
    ///
    fn logical_or_op_17(
        &mut self,
        _logical_or_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 18:
    ///
    /// logical_or_item: logical_or_op logical_and;
    ///
    fn logical_or_item_18(
        &mut self,
        _logical_or_op_0: &ParseTreeStackEntry,
        _logical_and_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 19:
    ///
    /// logical_and: bitwise_or logical_and_lst1;
    ///
    fn logical_and_19(
        &mut self,
        _bitwise_or_0: &ParseTreeStackEntry,
        _logical_and_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 20:
    ///
    /// logical_and_lst1: logical_and_lst1_itm1 logical_and_lst1;
    ///
    fn logical_and_lst1_20(
        &mut self,
        _logical_and_lst1_itm1_0: &ParseTreeStackEntry,
        _logical_and_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 21:
    ///
    /// logical_and_lst1_itm1: logical_and_item;
    ///
    fn logical_and_lst1_itm1_21(
        &mut self,
        _logical_and_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 22:
    ///
    /// logical_and_lst1: ;
    ///
    fn logical_and_lst1_22(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 23:
    ///
    /// logical_and_op: "&&";
    ///
    fn logical_and_op_23(
        &mut self,
        _logical_and_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 24:
    ///
    /// logical_and_item: logical_and_op bitwise_or;
    ///
    fn logical_and_item_24(
        &mut self,
        _logical_and_op_0: &ParseTreeStackEntry,
        _bitwise_or_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 25:
    ///
    /// bitwise_or: bitwise_and bitwise_or_lst1;
    ///
    fn bitwise_or_25(
        &mut self,
        _bitwise_and_0: &ParseTreeStackEntry,
        _bitwise_or_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 26:
    ///
    /// bitwise_or_lst1: bitwise_or_lst1_itm1 bitwise_or_lst1;
    ///
    fn bitwise_or_lst1_26(
        &mut self,
        _bitwise_or_lst1_itm1_0: &ParseTreeStackEntry,
        _bitwise_or_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 27:
    ///
    /// bitwise_or_lst1_itm1: bitwise_or_item;
    ///
    fn bitwise_or_lst1_itm1_27(
        &mut self,
        _bitwise_or_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 28:
    ///
    /// bitwise_or_lst1: ;
    ///
    fn bitwise_or_lst1_28(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 29:
    ///
    /// bitwise_or_op: "\|";
    ///
    fn bitwise_or_op_29(
        &mut self,
        _bitwise_or_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 30:
    ///
    /// bitwise_or_item: bitwise_or_op bitwise_and;
    ///
    fn bitwise_or_item_30(
        &mut self,
        _bitwise_or_op_0: &ParseTreeStackEntry,
        _bitwise_and_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 31:
    ///
    /// bitwise_and: equality bitwise_and_lst1;
    ///
    fn bitwise_and_31(
        &mut self,
        _equality_0: &ParseTreeStackEntry,
        _bitwise_and_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 32:
    ///
    /// bitwise_and_lst1: bitwise_and_lst1_itm1 bitwise_and_lst1;
    ///
    fn bitwise_and_lst1_32(
        &mut self,
        _bitwise_and_lst1_itm1_0: &ParseTreeStackEntry,
        _bitwise_and_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 33:
    ///
    /// bitwise_and_lst1_itm1: bitwise_and_item;
    ///
    fn bitwise_and_lst1_itm1_33(
        &mut self,
        _bitwise_and_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 34:
    ///
    /// bitwise_and_lst1: ;
    ///
    fn bitwise_and_lst1_34(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 35:
    ///
    /// bitwise_and_op: "&";
    ///
    fn bitwise_and_op_35(
        &mut self,
        _bitwise_and_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 36:
    ///
    /// bitwise_and_item: bitwise_and_op equality;
    ///
    fn bitwise_and_item_36(
        &mut self,
        _bitwise_and_op_0: &ParseTreeStackEntry,
        _equality_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 37:
    ///
    /// equality: relational equality_lst1;
    ///
    fn equality_37(
        &mut self,
        _relational_0: &ParseTreeStackEntry,
        _equality_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 38:
    ///
    /// equality_lst1: equality_lst1_itm1 equality_lst1;
    ///
    fn equality_lst1_38(
        &mut self,
        _equality_lst1_itm1_0: &ParseTreeStackEntry,
        _equality_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 39:
    ///
    /// equality_lst1_itm1: equality_item;
    ///
    fn equality_lst1_itm1_39(
        &mut self,
        _equality_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 40:
    ///
    /// equality_lst1: ;
    ///
    fn equality_lst1_40(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 41:
    ///
    /// equality_item: equality_op relational;
    ///
    fn equality_item_41(
        &mut self,
        _equality_op_0: &ParseTreeStackEntry,
        _relational_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 42:
    ///
    /// bitwise_shift_op: "<<|>>";
    ///
    fn bitwise_shift_op_42(
        &mut self,
        _bitwise_shift_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 43:
    ///
    /// relational: bitwise_shift relational_lst1;
    ///
    fn relational_43(
        &mut self,
        _bitwise_shift_0: &ParseTreeStackEntry,
        _relational_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 44:
    ///
    /// relational_lst1: relational_lst1_itm1 relational_lst1;
    ///
    fn relational_lst1_44(
        &mut self,
        _relational_lst1_itm1_0: &ParseTreeStackEntry,
        _relational_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 45:
    ///
    /// relational_lst1_itm1: relational_item;
    ///
    fn relational_lst1_itm1_45(
        &mut self,
        _relational_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 46:
    ///
    /// relational_lst1: ;
    ///
    fn relational_lst1_46(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 47:
    ///
    /// relational_op: "<=|<|>=|>";
    ///
    fn relational_op_47(
        &mut self,
        _relational_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 48:
    ///
    /// relational_item: relational_op bitwise_shift;
    ///
    fn relational_item_48(
        &mut self,
        _relational_op_0: &ParseTreeStackEntry,
        _bitwise_shift_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 49:
    ///
    /// bitwise_shift: summ bitwise_shift_lst1;
    ///
    fn bitwise_shift_49(
        &mut self,
        _summ_0: &ParseTreeStackEntry,
        _bitwise_shift_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 50:
    ///
    /// bitwise_shift_lst1: bitwise_shift_lst1_itm1 bitwise_shift_lst1;
    ///
    fn bitwise_shift_lst1_50(
        &mut self,
        _bitwise_shift_lst1_itm1_0: &ParseTreeStackEntry,
        _bitwise_shift_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 51:
    ///
    /// bitwise_shift_lst1_itm1: bitwise_shift_item;
    ///
    fn bitwise_shift_lst1_itm1_51(
        &mut self,
        _bitwise_shift_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 52:
    ///
    /// bitwise_shift_lst1: ;
    ///
    fn bitwise_shift_lst1_52(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 53:
    ///
    /// bitwise_shift_item: bitwise_shift_op summ;
    ///
    fn bitwise_shift_item_53(
        &mut self,
        _bitwise_shift_op_0: &ParseTreeStackEntry,
        _summ_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 54:
    ///
    /// summ: mult summ_lst1;
    ///
    fn summ_54(
        &mut self,
        _mult_0: &ParseTreeStackEntry,
        _summ_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 55:
    ///
    /// summ_lst1: summ_lst1_itm1 summ_lst1;
    ///
    fn summ_lst1_55(
        &mut self,
        _summ_lst1_itm1_0: &ParseTreeStackEntry,
        _summ_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 56:
    ///
    /// summ_lst1_itm1: summ_item;
    ///
    fn summ_lst1_itm1_56(
        &mut self,
        _summ_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 57:
    ///
    /// summ_lst1: ;
    ///
    fn summ_lst1_57(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 58:
    ///
    /// plus: "\+";
    ///
    fn plus_58(
        &mut self,
        _plus_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 59:
    ///
    /// minus: "-";
    ///
    fn minus_59(
        &mut self,
        _minus_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 60:
    ///
    /// add_op: plus;
    ///
    fn add_op_60(
        &mut self,
        _plus_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 61:
    ///
    /// add_op: minus;
    ///
    fn add_op_61(
        &mut self,
        _minus_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 62:
    ///
    /// summ_item: add_op mult;
    ///
    fn summ_item_62(
        &mut self,
        _add_op_0: &ParseTreeStackEntry,
        _mult_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 63:
    ///
    /// pow_op: "\*\*";
    ///
    fn pow_op_63(
        &mut self,
        _pow_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 64:
    ///
    /// mult: power mult_lst1;
    ///
    fn mult_64(
        &mut self,
        _power_0: &ParseTreeStackEntry,
        _mult_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 65:
    ///
    /// mult_lst1: mult_lst1_itm1 mult_lst1;
    ///
    fn mult_lst1_65(
        &mut self,
        _mult_lst1_itm1_0: &ParseTreeStackEntry,
        _mult_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 66:
    ///
    /// mult_lst1_itm1: mult_item;
    ///
    fn mult_lst1_itm1_66(
        &mut self,
        _mult_item_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 67:
    ///
    /// mult_lst1: ;
    ///
    fn mult_lst1_67(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 68:
    ///
    /// mult_op: "\*|/|%";
    ///
    fn mult_op_68(
        &mut self,
        _mult_op_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 69:
    ///
    /// mult_item: mult_op power;
    ///
    fn mult_item_69(
        &mut self,
        _mult_op_0: &ParseTreeStackEntry,
        _power_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 70:
    ///
    /// power: factor power_lst1;
    ///
    fn power_70(
        &mut self,
        _factor_0: &ParseTreeStackEntry,
        _power_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 71:
    ///
    /// power_lst1: power_lst1_itm1 power_lst1;
    ///
    fn power_lst1_71(
        &mut self,
        _power_lst1_itm1_0: &ParseTreeStackEntry,
        _power_lst1_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 72:
    ///
    /// power_lst1_itm1: pow_op factor;
    ///
    fn power_lst1_itm1_72(
        &mut self,
        _pow_op_0: &ParseTreeStackEntry,
        _factor_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 73:
    ///
    /// power_lst1: ;
    ///
    fn power_lst1_73(
        &mut self,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 74:
    ///
    /// negate: minus;
    ///
    fn negate_74(
        &mut self,
        _minus_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 75:
    ///
    /// factor: number;
    ///
    fn factor_75(
        &mut self,
        _number_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 76:
    ///
    /// factor: idref;
    ///
    fn factor_76(
        &mut self,
        _idref_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 77:
    ///
    /// factor: negate factor;
    ///
    fn factor_77(
        &mut self,
        _negate_0: &ParseTreeStackEntry,
        _factor_1: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 78:
    ///
    /// factor: "\(" logical_or "\)";
    ///
    fn factor_78(
        &mut self,
        _l_paren_0: &ParseTreeStackEntry,
        _logical_or_1: &ParseTreeStackEntry,
        _r_paren_2: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 79:
    ///
    /// number: "\d+";
    ///
    fn number_79(
        &mut self,
        _number_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 80:
    ///
    /// idref: id;
    ///
    fn idref_80(
        &mut self,
        _id_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }

    /// Semantic action for production 81:
    ///
    /// id: "[a-zA-Z_]\w*";
    ///
    fn id_81(
        &mut self,
        _id_0: &ParseTreeStackEntry,
        _parse_tree: &Tree<ParseTreeType>,
        mut _scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        Ok(())
    }
}

impl UserActionsTrait for CalcGrammar {
    fn call_semantic_action_for_production_number(
        &mut self,
        prod_num: usize,
        children: &[ParseTreeStackEntry],
        parse_tree: &Tree<ParseTreeType>,
        scanner_access: RefMut<dyn ScannerAccess>,
    ) -> Result<()> {
        match prod_num {
            0 => self.calc_0(&children[0], parse_tree, scanner_access),

            1 => self.calc_lst1_1(&children[0], &children[1], parse_tree, scanner_access),

            2 => self.calc_lst1_itm1_2(&children[0], &children[1], parse_tree, scanner_access),

            3 => self.calc_lst1_3(parse_tree, scanner_access),

            4 => self.instruction_4(&children[0], parse_tree, scanner_access),

            5 => self.instruction_5(&children[0], parse_tree, scanner_access),

            6 => self.equality_op_6(&children[0], parse_tree, scanner_access),

            7 => self.assign_op_7(&children[0], parse_tree, scanner_access),

            8 => self.assign_item_8(&children[0], &children[1], parse_tree, scanner_access),

            9 => self.assignment_9(
                &children[0],
                &children[1],
                &children[2],
                parse_tree,
                scanner_access,
            ),

            10 => self.assignment_lst1_10(&children[0], &children[1], parse_tree, scanner_access),

            11 => self.assignment_lst1_itm1_11(&children[0], parse_tree, scanner_access),

            12 => self.assignment_lst1_12(parse_tree, scanner_access),

            13 => self.logical_or_13(&children[0], &children[1], parse_tree, scanner_access),

            14 => self.logical_or_lst1_14(&children[0], &children[1], parse_tree, scanner_access),

            15 => self.logical_or_lst1_itm1_15(&children[0], parse_tree, scanner_access),

            16 => self.logical_or_lst1_16(parse_tree, scanner_access),

            17 => self.logical_or_op_17(&children[0], parse_tree, scanner_access),

            18 => self.logical_or_item_18(&children[0], &children[1], parse_tree, scanner_access),

            19 => self.logical_and_19(&children[0], &children[1], parse_tree, scanner_access),

            20 => self.logical_and_lst1_20(&children[0], &children[1], parse_tree, scanner_access),

            21 => self.logical_and_lst1_itm1_21(&children[0], parse_tree, scanner_access),

            22 => self.logical_and_lst1_22(parse_tree, scanner_access),

            23 => self.logical_and_op_23(&children[0], parse_tree, scanner_access),

            24 => self.logical_and_item_24(&children[0], &children[1], parse_tree, scanner_access),

            25 => self.bitwise_or_25(&children[0], &children[1], parse_tree, scanner_access),

            26 => self.bitwise_or_lst1_26(&children[0], &children[1], parse_tree, scanner_access),

            27 => self.bitwise_or_lst1_itm1_27(&children[0], parse_tree, scanner_access),

            28 => self.bitwise_or_lst1_28(parse_tree, scanner_access),

            29 => self.bitwise_or_op_29(&children[0], parse_tree, scanner_access),

            30 => self.bitwise_or_item_30(&children[0], &children[1], parse_tree, scanner_access),

            31 => self.bitwise_and_31(&children[0], &children[1], parse_tree, scanner_access),

            32 => self.bitwise_and_lst1_32(&children[0], &children[1], parse_tree, scanner_access),

            33 => self.bitwise_and_lst1_itm1_33(&children[0], parse_tree, scanner_access),

            34 => self.bitwise_and_lst1_34(parse_tree, scanner_access),

            35 => self.bitwise_and_op_35(&children[0], parse_tree, scanner_access),

            36 => self.bitwise_and_item_36(&children[0], &children[1], parse_tree, scanner_access),

            37 => self.equality_37(&children[0], &children[1], parse_tree, scanner_access),

            38 => self.equality_lst1_38(&children[0], &children[1], parse_tree, scanner_access),

            39 => self.equality_lst1_itm1_39(&children[0], parse_tree, scanner_access),

            40 => self.equality_lst1_40(parse_tree, scanner_access),

            41 => self.equality_item_41(&children[0], &children[1], parse_tree, scanner_access),

            42 => self.bitwise_shift_op_42(&children[0], parse_tree, scanner_access),

            43 => self.relational_43(&children[0], &children[1], parse_tree, scanner_access),

            44 => self.relational_lst1_44(&children[0], &children[1], parse_tree, scanner_access),

            45 => self.relational_lst1_itm1_45(&children[0], parse_tree, scanner_access),

            46 => self.relational_lst1_46(parse_tree, scanner_access),

            47 => self.relational_op_47(&children[0], parse_tree, scanner_access),

            48 => self.relational_item_48(&children[0], &children[1], parse_tree, scanner_access),

            49 => self.bitwise_shift_49(&children[0], &children[1], parse_tree, scanner_access),

            50 => {
                self.bitwise_shift_lst1_50(&children[0], &children[1], parse_tree, scanner_access)
            }

            51 => self.bitwise_shift_lst1_itm1_51(&children[0], parse_tree, scanner_access),

            52 => self.bitwise_shift_lst1_52(parse_tree, scanner_access),

            53 => {
                self.bitwise_shift_item_53(&children[0], &children[1], parse_tree, scanner_access)
            }

            54 => self.summ_54(&children[0], &children[1], parse_tree, scanner_access),

            55 => self.summ_lst1_55(&children[0], &children[1], parse_tree, scanner_access),

            56 => self.summ_lst1_itm1_56(&children[0], parse_tree, scanner_access),

            57 => self.summ_lst1_57(parse_tree, scanner_access),

            58 => self.plus_58(&children[0], parse_tree, scanner_access),

            59 => self.minus_59(&children[0], parse_tree, scanner_access),

            60 => self.add_op_60(&children[0], parse_tree, scanner_access),

            61 => self.add_op_61(&children[0], parse_tree, scanner_access),

            62 => self.summ_item_62(&children[0], &children[1], parse_tree, scanner_access),

            63 => self.pow_op_63(&children[0], parse_tree, scanner_access),

            64 => self.mult_64(&children[0], &children[1], parse_tree, scanner_access),

            65 => self.mult_lst1_65(&children[0], &children[1], parse_tree, scanner_access),

            66 => self.mult_lst1_itm1_66(&children[0], parse_tree, scanner_access),

            67 => self.mult_lst1_67(parse_tree, scanner_access),

            68 => self.mult_op_68(&children[0], parse_tree, scanner_access),

            69 => self.mult_item_69(&children[0], &children[1], parse_tree, scanner_access),

            70 => self.power_70(&children[0], &children[1], parse_tree, scanner_access),

            71 => self.power_lst1_71(&children[0], &children[1], parse_tree, scanner_access),

            72 => self.power_lst1_itm1_72(&children[0], &children[1], parse_tree, scanner_access),

            73 => self.power_lst1_73(parse_tree, scanner_access),

            74 => self.negate_74(&children[0], parse_tree, scanner_access),

            75 => self.factor_75(&children[0], parse_tree, scanner_access),

            76 => self.factor_76(&children[0], parse_tree, scanner_access),

            77 => self.factor_77(&children[0], &children[1], parse_tree, scanner_access),

            78 => self.factor_78(
                &children[0],
                &children[1],
                &children[2],
                parse_tree,
                scanner_access,
            ),

            79 => self.number_79(&children[0], parse_tree, scanner_access),

            80 => self.idref_80(&children[0], parse_tree, scanner_access),

            81 => self.id_81(&children[0], parse_tree, scanner_access),

            _ => panic!("Unhandled production number: {}", prod_num),
        }
    }
}
