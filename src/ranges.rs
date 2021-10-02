use swc::common::*;
use swc_ecmascript::ast::*;
use std::{
    fmt::{self, Debug}
};
use swc_common::source_map::Pos;
use swc_common::pass::{Repeat, Repeated};
use swc_common::DUMMY_SP;
use swc_ecmascript::ast::*;
use swc_ecmascript::utils::ident::IdentLike;
use swc_ecmascript::visit::FoldWith;
use swc_ecmascript::{
    utils::Id,
    visit::{noop_fold_type, Fold, Visit, VisitWith, VisitMut
    },
};
use swc_ecmascript::codegen;
use swc_ecma_ast::*;
use swc::ecmascript::visit::{as_folder, Folder};
use swc_common::{pass::CompilerPass, Span};
use swc_common::source_map::SourceMap;

// For if we implemented it as a "pass".
// pub fn range_collector() -> impl Fold + Visit {
//     immutable_visit_as_folder(RangeCollector { ranges: vec![vec![]] } )
// }

pub struct RangeCollector<'a> {
    pub spans: &'a mut Vec<Span>,
}

pub fn get_ranges(program: &Program, cm: Arc<SourceMap>) -> Ranges {
    // Create Wallaby ranges.
    // See: https://wallabyjs.com/docs/config/compilers.html#writing-a-custom-compiler
    let ranges = {
        let mut spans: Vec<Span> = vec![];
        let mut visitor = RangeCollector { spans: &mut spans };
        program.visit_with(&Invalid { span: DUMMY_SP }, &mut visitor);

        let mut ranges: Ranges = vec![];
        for span in spans {
            let lo = cm.lookup_char_pos(span.lo());
            let hi = cm.lookup_char_pos(span.hi());
            let new_range = vec![lo.line, lo.col.0, hi.line, hi.col.0];
            ranges.push(new_range);
        }
        ranges
    };
    // dbg!(&ranges);
    ranges
}

impl RangeCollector<'_> {
    fn show<N>(&mut self, _name: &str, node: &N)
        where
            N: Spanned + fmt::Debug + swc_ecmascript::codegen::Node,
    {
        let span = node.span();
        if !span.is_dummy() {
            self.spans.push(span);
        }
    }
    fn show_name<N>(&mut self, _name: &str, node: &N)
        where
            N: Spanned + fmt::Debug,
    {
        let span = node.span();
        if !span.is_dummy() {
            self.spans.push(span);
        }
    }
}

impl Visit for RangeCollector<'_> {
    //noop_visit_type!();

    //fn visit_module(&mut self, n: &swc_ecma_ast::Module, _parent: &dyn swc_ecmascript::visit::Node) {
    //    let span = n.span;
    //    let new_range = vec![0, span.lo().to_u32(), 0, span.hi().to_u32()];
    //    self.ranges.push(new_range);
    //    n.visit_children_with(self)
    //}

    //fn visit_expr(&mut self, n: &Expr, _: &dyn swc_ecmascript::visit::Node)

    // TODO(vjpr): Use a macro to implement these instead.
    // Waiting for generic visitors: https://discord.com/channels/889779439272075314/889785438938726441/893008828520607806
    fn visit_array_lit(&mut self, n: &ArrayLit, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ArrayLit", n);
        n.visit_children_with(self)
    }
    fn visit_array_pat(&mut self, n: &ArrayPat, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ArrayPat", n);
        n.visit_children_with(self)
    }
    fn visit_arrow_expr(&mut self, n: &ArrowExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ArrowExpr", n);
        n.visit_children_with(self)
    }
    fn visit_assign_expr(&mut self, n: &AssignExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("AssignExpr", n);
        n.visit_children_with(self)
    }
    fn visit_assign_pat(&mut self, n: &AssignPat, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("AssignPat", n);
        n.visit_children_with(self)
    }
    fn visit_assign_pat_prop(&mut self, n: &AssignPatProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("AssignPatProp", n);
        n.visit_children_with(self)
    }
    fn visit_assign_prop(&mut self, n: &AssignProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("AssignProp", n);
        n.visit_children_with(self)
    }
    fn visit_await_expr(&mut self, n: &AwaitExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("AwaitExpr", n);
        n.visit_children_with(self)
    }
    fn visit_bin_expr(&mut self, n: &BinExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("BinExpr", n);
        n.visit_children_with(self)
    }
    fn visit_block_stmt(&mut self, n: &BlockStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("BlockStmt", n);
        n.visit_children_with(self)
    }
    fn visit_block_stmt_or_expr(&mut self, n: &BlockStmtOrExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("BlockStmtOrExpr", n);
        n.visit_children_with(self)
    }
    fn visit_bool(&mut self, n: &Bool, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Bool", n);
        n.visit_children_with(self)
    }
    fn visit_break_stmt(&mut self, n: &BreakStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("BreakStmt", n);
        n.visit_children_with(self)
    }
    fn visit_call_expr(&mut self, n: &CallExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("CallExpr", n);
        n.visit_children_with(self)
    }
    fn visit_catch_clause(&mut self, n: &CatchClause, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("CatchClause", n);
        n.visit_children_with(self)
    }
    fn visit_class(&mut self, n: &Class, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Class", n);
        n.visit_children_with(self)
    }
    fn visit_class_decl(&mut self, n: &ClassDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ClassDecl", n);
        n.visit_children_with(self)
    }
    fn visit_class_expr(&mut self, n: &ClassExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ClassExpr", n);
        n.visit_children_with(self)
    }
    fn visit_class_member(&mut self, n: &ClassMember, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ClassMember", n);
        n.visit_children_with(self)
    }
    fn visit_class_method(&mut self, n: &ClassMethod, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ClassMethod", n);
        n.visit_children_with(self)
    }
    fn visit_class_prop(&mut self, n: &ClassProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ClassProp", n);
        n.visit_children_with(self)
    }
    fn visit_computed_prop_name(&mut self, n: &ComputedPropName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ComputedPropName", n);
        n.visit_children_with(self)
    }
    fn visit_cond_expr(&mut self, n: &CondExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("CondExpr", n);
        n.visit_children_with(self)
    }
    fn visit_constructor(&mut self, n: &Constructor, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Constructor", n);
        n.visit_children_with(self)
    }
    fn visit_continue_stmt(&mut self, n: &ContinueStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ContinueStmt", n);
        n.visit_children_with(self)
    }
    fn visit_debugger_stmt(&mut self, n: &DebuggerStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("DebuggerStmt", n);
        n.visit_children_with(self)
    }
    fn visit_decl(&mut self, n: &Decl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Decl", n);
        n.visit_children_with(self)
    }
    fn visit_decorator(&mut self, n: &Decorator, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Decorator", n);
        n.visit_children_with(self)
    }
    fn visit_default_decl(&mut self, n: &DefaultDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show_name("DefaultDecl", n);
        n.visit_children_with(self)
    }
    fn visit_do_while_stmt(&mut self, n: &DoWhileStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("DoWhileStmt", n);
        n.visit_children_with(self)
    }
    fn visit_empty_stmt(&mut self, n: &EmptyStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("EmptyStmt", n);
        n.visit_children_with(self)
    }
    fn visit_export_all(&mut self, n: &ExportAll, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ExportAll", n);
        n.visit_children_with(self)
    }
    fn visit_export_decl(&mut self, n: &ExportDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ExportDecl", n);
        n.visit_children_with(self)
    }
    fn visit_export_default_decl(&mut self, n: &ExportDefaultDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ExportDefaultDecl", n);
        n.visit_children_with(self)
    }
    fn visit_export_default_expr(&mut self, n: &ExportDefaultExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ExportDefaultExpr", n);
        n.visit_children_with(self)
    }
    fn visit_export_default_specifier(&mut self, n: &ExportDefaultSpecifier, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show_name("ExportDefaultSpecifier", n);
        n.visit_children_with(self)
    }
    fn visit_export_named_specifier(&mut self, n: &ExportNamedSpecifier, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ExportNamedSpecifier", n);
        n.visit_children_with(self)
    }
    fn visit_export_namespace_specifier(
        &mut self,
        n: &ExportNamespaceSpecifier,
        _parent: &dyn swc_ecmascript::visit::Node,
    ) {
        self.show("ExportNamespaceSpecifier", n);
        n.visit_children_with(self)
    }
    fn visit_export_specifier(&mut self, n: &ExportSpecifier, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ExportSpecifier", n);
        n.visit_children_with(self)
    }
    fn visit_expr(&mut self, n: &Expr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Expr", n);
        n.visit_children_with(self)
    }
    fn visit_expr_or_spread(&mut self, n: &ExprOrSpread, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ExprOrSpread", n);
        n.visit_children_with(self)
    }
    fn visit_expr_or_super(&mut self, n: &ExprOrSuper, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ExprOrSuper", n);
        n.visit_children_with(self)
    }
    fn visit_fn_decl(&mut self, n: &FnDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("FnDecl", n);
        n.visit_children_with(self)
    }
    fn visit_fn_expr(&mut self, n: &FnExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("FnExpr", n);
        n.visit_children_with(self)
    }
    fn visit_for_in_stmt(&mut self, n: &ForInStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ForInStmt", n);
        n.visit_children_with(self)
    }
    fn visit_for_of_stmt(&mut self, n: &ForOfStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ForOfStmt", n);
        n.visit_children_with(self)
    }
    fn visit_for_stmt(&mut self, n: &ForStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ForStmt", n);
        n.visit_children_with(self)
    }
    fn visit_function(&mut self, n: &Function, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Function", n);
        n.visit_children_with(self)
    }
    fn visit_getter_prop(&mut self, n: &GetterProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("GetterProp", n);
        n.visit_children_with(self)
    }
    fn visit_ident(&mut self, n: &Ident, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Ident", n);
        n.visit_children_with(self)
    }
    fn visit_if_stmt(&mut self, n: &IfStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("IfStmt", n);
        n.visit_children_with(self)
    }
    fn visit_import_decl(&mut self, n: &ImportDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ImportDecl", n);
        n.visit_children_with(self)
    }
    fn visit_import_default_specifier(&mut self, n: &ImportDefaultSpecifier, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show_name("ImportDefaultSpecifier", n);
        n.visit_children_with(self)
    }
    fn visit_import_named_specifier(&mut self, n: &ImportNamedSpecifier, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ImportNamedSpecifier", n);
        n.visit_children_with(self)
    }
    fn visit_import_specifier(&mut self, n: &ImportSpecifier, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show_name("ImportSpecifier", n);
        n.visit_children_with(self)
    }
    fn visit_import_star_as_specifier(&mut self, n: &ImportStarAsSpecifier, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show_name("ImportStarAsSpecifier", n);
        n.visit_children_with(self)
    }
    fn visit_invalid(&mut self, n: &Invalid, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Invalid", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_attr(&mut self, n: &JSXAttr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXAttr", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_attr_name(&mut self, n: &JSXAttrName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXAttrName", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_attr_or_spread(&mut self, n: &JSXAttrOrSpread, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXAttrOrSpread", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_attr_value(&mut self, n: &JSXAttrValue, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXAttrValue", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_closing_element(&mut self, n: &JSXClosingElement, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXClosingElement", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_closing_fragment(&mut self, n: &JSXClosingFragment, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXClosingFragment", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_element(&mut self, n: &JSXElement, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXElement", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_element_child(&mut self, n: &JSXElementChild, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXElementChild", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_element_name(&mut self, n: &JSXElementName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXElementName", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_empty_expr(&mut self, n: &JSXEmptyExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXEmptyExpr", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_expr(&mut self, n: &JSXExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXExpr", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_expr_container(&mut self, n: &JSXExprContainer, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXExprContainer", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_fragment(&mut self, n: &JSXFragment, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXFragment", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_member_expr(&mut self, n: &JSXMemberExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXMemberExpr", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_namespaced_name(&mut self, n: &JSXNamespacedName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXNamespacedName", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_object(&mut self, n: &JSXObject, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXObject", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_opening_element(&mut self, n: &JSXOpeningElement, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXOpeningElement", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_opening_fragment(&mut self, n: &JSXOpeningFragment, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXOpeningFragment", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_spread_child(&mut self, n: &JSXSpreadChild, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXSpreadChild", n);
        n.visit_children_with(self)
    }
    fn visit_jsx_text(&mut self, n: &JSXText, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("JSXText", n);
        n.visit_children_with(self)
    }
    fn visit_key_value_pat_prop(&mut self, n: &KeyValuePatProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("KeyValuePatProp", n);
        n.visit_children_with(self)
    }
    fn visit_key_value_prop(&mut self, n: &KeyValueProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("KeyValueProp", n);
        n.visit_children_with(self)
    }
    fn visit_labeled_stmt(&mut self, n: &LabeledStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("LabeledStmt", n);
        n.visit_children_with(self)
    }
    fn visit_lit(&mut self, n: &Lit, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Lit", n);
        n.visit_children_with(self)
    }
    fn visit_member_expr(&mut self, n: &MemberExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("MemberExpr", n);
        n.visit_children_with(self)
    }
    fn visit_meta_prop_expr(&mut self, n: &MetaPropExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("MetaPropExpr", n);
        n.visit_children_with(self)
    }
    fn visit_method_prop(&mut self, n: &MethodProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("MethodProp", n);
        n.visit_children_with(self)
    }
    fn visit_module(&mut self, n: &Module, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Module", n);
        n.visit_children_with(self)
    }
    fn visit_module_decl(&mut self, n: &ModuleDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ModuleDecl", n);
        n.visit_children_with(self)
    }
    fn visit_named_export(&mut self, n: &NamedExport, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("NamedExport", n);
        n.visit_children_with(self)
    }
    fn visit_new_expr(&mut self, n: &NewExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("NewExpr", n);
        n.visit_children_with(self)
    }
    fn visit_null(&mut self, n: &Null, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show_name("Null", n);
        n.visit_children_with(self)
    }
    fn visit_number(&mut self, n: &Number, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Number", n);
        n.visit_children_with(self)
    }
    fn visit_object_lit(&mut self, n: &ObjectLit, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ObjectLit", n);
        n.visit_children_with(self)
    }
    fn visit_object_pat(&mut self, n: &ObjectPat, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ObjectPat", n);
        n.visit_children_with(self)
    }
    fn visit_object_pat_prop(&mut self, n: &ObjectPatProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ObjectPatProp", n);
        n.visit_children_with(self)
    }
    fn visit_opt_chain_expr(&mut self, n: &OptChainExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("OptChainExpr", n);
        n.visit_children_with(self)
    }
    fn visit_param(&mut self, n: &Param, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Param", n);
        n.visit_children_with(self)
    }
    fn visit_param_or_ts_param_prop(&mut self, n: &ParamOrTsParamProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ParamOrTsParamProp", n);
        n.visit_children_with(self)
    }
    fn visit_paren_expr(&mut self, n: &ParenExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ParenExpr", n);
        n.visit_children_with(self)
    }
    fn visit_pat(&mut self, n: &Pat, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Pat", n);
        n.visit_children_with(self)
    }
    fn visit_pat_or_expr(&mut self, n: &PatOrExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("PatOrExpr", n);
        n.visit_children_with(self)
    }
    fn visit_private_method(&mut self, n: &PrivateMethod, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("PrivateMethod", n);
        n.visit_children_with(self)
    }
    fn visit_private_name(&mut self, n: &PrivateName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("PrivateName", n);
        n.visit_children_with(self)
    }
    fn visit_private_prop(&mut self, n: &PrivateProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("PrivateProp", n);
        n.visit_children_with(self)
    }
    fn visit_program(&mut self, n: &Program, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Program", n);
        n.visit_children_with(self)
    }
    fn visit_prop(&mut self, n: &Prop, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Prop", n);
        n.visit_children_with(self)
    }
    fn visit_prop_name(&mut self, n: &PropName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("PropName", n);
        n.visit_children_with(self)
    }
    fn visit_prop_or_spread(&mut self, n: &PropOrSpread, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("PropOrSpread", n);
        n.visit_children_with(self)
    }
    fn visit_regex(&mut self, n: &Regex, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show_name("Regex", n);
        n.visit_children_with(self)
    }
    fn visit_rest_pat(&mut self, n: &RestPat, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("RestPat", n);
        n.visit_children_with(self)
    }
    fn visit_return_stmt(&mut self, n: &ReturnStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ReturnStmt", n);
        n.visit_children_with(self)
    }
    fn visit_script(&mut self, n: &Script, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Script", n);
        n.visit_children_with(self)
    }
    fn visit_seq_expr(&mut self, n: &SeqExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("SeqExpr", n);
        n.visit_children_with(self)
    }
    fn visit_setter_prop(&mut self, n: &SetterProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("SetterProp", n);
        n.visit_children_with(self)
    }
    fn visit_spread_element(&mut self, n: &SpreadElement, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("SpreadElement", n);
        n.visit_children_with(self)
    }
    fn visit_str(&mut self, n: &Str, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Str", n);
        n.visit_children_with(self)
    }
    fn visit_super(&mut self, n: &Super, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Super", n);
        n.visit_children_with(self)
    }
    fn visit_switch_case(&mut self, n: &SwitchCase, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("SwitchCase", n);
        n.visit_children_with(self)
    }
    fn visit_switch_stmt(&mut self, n: &SwitchStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("SwitchStmt", n);
        n.visit_children_with(self)
    }
    fn visit_tagged_tpl(&mut self, n: &TaggedTpl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TaggedTpl", n);
        n.visit_children_with(self)
    }
    fn visit_this_expr(&mut self, n: &ThisExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ThisExpr", n);
        n.visit_children_with(self)
    }
    fn visit_throw_stmt(&mut self, n: &ThrowStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("ThrowStmt", n);
        n.visit_children_with(self)
    }
    fn visit_tpl(&mut self, n: &Tpl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("Tpl", n);
        n.visit_children_with(self)
    }
    fn visit_tpl_element(&mut self, n: &TplElement, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TplElement", n);
        n.visit_children_with(self)
    }
    fn visit_try_stmt(&mut self, n: &TryStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TryStmt", n);
        n.visit_children_with(self)
    }
    fn visit_ts_array_type(&mut self, n: &TsArrayType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsArrayType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_as_expr(&mut self, n: &TsAsExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsAsExpr", n);
        n.visit_children_with(self)
    }
    fn visit_ts_call_signature_decl(&mut self, n: &TsCallSignatureDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsCallSignatureDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_conditional_type(&mut self, n: &TsConditionalType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsConditionalType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_const_assertion(&mut self, n: &TsConstAssertion, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsConstAssertion", n);
        n.visit_children_with(self)
    }
    fn visit_ts_construct_signature_decl(
        &mut self,
        n: &TsConstructSignatureDecl,
        _parent: &dyn swc_ecmascript::visit::Node,
    ) {
        self.show("TsConstructSignatureDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_constructor_type(&mut self, n: &TsConstructorType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsConstructorType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_entity_name(&mut self, n: &TsEntityName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsEntityName", n);
        n.visit_children_with(self)
    }
    fn visit_ts_enum_decl(&mut self, n: &TsEnumDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsEnumDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_enum_member(&mut self, n: &TsEnumMember, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsEnumMember", n);
        n.visit_children_with(self)
    }
    fn visit_ts_enum_member_id(&mut self, n: &TsEnumMemberId, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsEnumMemberId", n);
        n.visit_children_with(self)
    }
    fn visit_ts_export_assignment(&mut self, n: &TsExportAssignment, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsExportAssignment", n);
        n.visit_children_with(self)
    }
    fn visit_ts_expr_with_type_args(&mut self, n: &TsExprWithTypeArgs, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsExprWithTypeArgs", n);
        n.visit_children_with(self)
    }
    fn visit_ts_external_module_ref(&mut self, n: &TsExternalModuleRef, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsExternalModuleRef", n);
        n.visit_children_with(self)
    }
    fn visit_ts_fn_or_constructor_type(&mut self, n: &TsFnOrConstructorType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsFnOrConstructorType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_fn_param(&mut self, n: &TsFnParam, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsFnParam", n);
        n.visit_children_with(self)
    }
    fn visit_ts_fn_type(&mut self, n: &TsFnType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsFnType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_import_equals_decl(&mut self, n: &TsImportEqualsDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsImportEqualsDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_import_type(&mut self, n: &TsImportType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsImportType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_index_signature(&mut self, n: &TsIndexSignature, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsIndexSignature", n);
        n.visit_children_with(self)
    }
    fn visit_ts_indexed_access_type(&mut self, n: &TsIndexedAccessType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsIndexedAccessType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_infer_type(&mut self, n: &TsInferType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsInferType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_interface_body(&mut self, n: &TsInterfaceBody, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsInterfaceBody", n);
        n.visit_children_with(self)
    }
    fn visit_ts_interface_decl(&mut self, n: &TsInterfaceDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsInterfaceDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_intersection_type(&mut self, n: &TsIntersectionType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsIntersectionType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_keyword_type(&mut self, n: &TsKeywordType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsKeywordType", n);
        n.visit_children_with(self)
    }

    fn visit_ts_lit(&mut self, n: &TsLit, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsLit", n);
        n.visit_children_with(self)
    }
    fn visit_ts_lit_type(&mut self, n: &TsLitType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsLitType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_mapped_type(&mut self, n: &TsMappedType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsMappedType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_method_signature(&mut self, n: &TsMethodSignature, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsMethodSignature", n);
        n.visit_children_with(self)
    }
    fn visit_ts_module_block(&mut self, n: &TsModuleBlock, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsModuleBlock", n);
        n.visit_children_with(self)
    }
    fn visit_ts_module_decl(&mut self, n: &TsModuleDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsModuleDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_module_name(&mut self, n: &TsModuleName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsModuleName", n);
        n.visit_children_with(self)
    }
    fn visit_ts_module_ref(&mut self, n: &TsModuleRef, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsModuleRef", n);
        n.visit_children_with(self)
    }
    fn visit_ts_namespace_body(&mut self, n: &TsNamespaceBody, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsNamespaceBody", n);
        n.visit_children_with(self)
    }
    fn visit_ts_namespace_decl(&mut self, n: &TsNamespaceDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsNamespaceDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_namespace_export_decl(&mut self, n: &TsNamespaceExportDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsNamespaceExportDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_non_null_expr(&mut self, n: &TsNonNullExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsNonNullExpr", n);
        n.visit_children_with(self)
    }
    fn visit_ts_optional_type(&mut self, n: &TsOptionalType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsOptionalType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_param_prop(&mut self, n: &TsParamProp, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsParamProp", n);
        n.visit_children_with(self)
    }
    fn visit_ts_param_prop_param(&mut self, n: &TsParamPropParam, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsParamPropParam", n);
        n.visit_children_with(self)
    }
    fn visit_ts_parenthesized_type(&mut self, n: &TsParenthesizedType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsParenthesizedType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_property_signature(&mut self, n: &TsPropertySignature, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsPropertySignature", n);
        n.visit_children_with(self)
    }
    fn visit_ts_qualified_name(&mut self, n: &TsQualifiedName, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsQualifiedName", n);
        n.visit_children_with(self)
    }
    fn visit_ts_rest_type(&mut self, n: &TsRestType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsRestType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_this_type(&mut self, n: &TsThisType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsThisType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_this_type_or_ident(&mut self, n: &TsThisTypeOrIdent, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsThisTypeOrIdent", n);
        n.visit_children_with(self)
    }
    fn visit_ts_tuple_element(&mut self, n: &TsTupleElement, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTupleElement", n);
        n.visit_children_with(self)
    }
    fn visit_ts_tuple_type(&mut self, n: &TsTupleType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTupleType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type(&mut self, n: &TsType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_alias_decl(&mut self, n: &TsTypeAliasDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeAliasDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_ann(&mut self, n: &TsTypeAnn, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeAnn", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_assertion(&mut self, n: &TsTypeAssertion, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeAssertion", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_element(&mut self, n: &TsTypeElement, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeElement", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_lit(&mut self, n: &TsTypeLit, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeLit", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_operator(&mut self, n: &TsTypeOperator, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeOperator", n);
        n.visit_children_with(self)
    }

    fn visit_ts_type_param(&mut self, n: &TsTypeParam, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeParam", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_param_decl(&mut self, n: &TsTypeParamDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeParamDecl", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_param_instantiation(
        &mut self,
        n: &TsTypeParamInstantiation,
        _parent: &dyn swc_ecmascript::visit::Node,
    ) {
        self.show("TsTypeParamInstantiation", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_predicate(&mut self, n: &TsTypePredicate, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypePredicate", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_query(&mut self, n: &TsTypeQuery, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeQuery", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_query_expr(&mut self, n: &TsTypeQueryExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeQueryExpr", n);
        n.visit_children_with(self)
    }
    fn visit_ts_type_ref(&mut self, n: &TsTypeRef, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsTypeRef", n);
        n.visit_children_with(self)
    }
    fn visit_ts_union_or_intersection_type(
        &mut self,
        n: &TsUnionOrIntersectionType,
        _parent: &dyn swc_ecmascript::visit::Node,
    ) {
        self.show("TsUnionOrIntersectionType", n);
        n.visit_children_with(self)
    }
    fn visit_ts_union_type(&mut self, n: &TsUnionType, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("TsUnionType", n);
        n.visit_children_with(self)
    }
    fn visit_unary_expr(&mut self, n: &UnaryExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("UnaryExpr", n);
        n.visit_children_with(self)
    }
    fn visit_update_expr(&mut self, n: &UpdateExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("UpdateExpr", n);
        n.visit_children_with(self)
    }
    fn visit_var_decl(&mut self, n: &VarDecl, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("VarDecl", n);
        n.visit_children_with(self)
    }
    fn visit_var_decl_or_expr(&mut self, n: &VarDeclOrExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("VarDeclOrExpr", n);
        n.visit_children_with(self)
    }
    fn visit_var_decl_or_pat(&mut self, n: &VarDeclOrPat, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("VarDeclOrPat", n);
        n.visit_children_with(self)
    }
    fn visit_var_declarator(&mut self, n: &VarDeclarator, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("VarDeclarator", n);
        n.visit_children_with(self)
    }
    fn visit_while_stmt(&mut self, n: &WhileStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("WhileStmt", n);
        n.visit_children_with(self)
    }
    fn visit_with_stmt(&mut self, n: &WithStmt, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("WithStmt", n);
        n.visit_children_with(self)
    }
    fn visit_yield_expr(&mut self, n: &YieldExpr, _parent: &dyn swc_ecmascript::visit::Node) {
        self.show("YieldExpr", n);
        n.visit_children_with(self)
    }
}

pub type Ranges = Vec<Vec<usize>>;

use std::{any::Any, borrow::Cow};
use std::sync::Arc;

pub fn immutable_visit_as_folder<V>(v: V) -> ImmutableFolder<V>
where
    V: Visit,
{
    ImmutableFolder(v)
}

#[derive(Debug, Clone, Copy)]
pub struct ImmutableFolder<V: Visit>(V);

impl<V> CompilerPass for ImmutableFolder<V>
where
    V: Visit + CompilerPass,
{
    fn name() -> Cow<'static, str> {
        V::name()
    }
}


macro_rules! delegate {
    ($name:ident, $T:ty) => {
        #[inline(always)]
        fn $name(&mut self, n: &$T, _: &dyn swc_ecmascript::visit::Node) {
            n.visit_with(n, &mut self.0);
        }
    };
}

impl<V> Visit for ImmutableFolder<V>
where
    V: Visit,
{
    delegate!(visit_ident, Ident);
    delegate!(visit_span, Span);

    delegate!(visit_expr, Expr);
    delegate!(visit_decl, Decl);
    delegate!(visit_stmt, Stmt);
    delegate!(visit_pat, Pat);

    delegate!(visit_ts_type, TsType);

    delegate!(visit_module, Module);
    delegate!(visit_script, Script);
    delegate!(visit_program, Program);
}

macro_rules! method {
    ($name:ident, $T:ty) => {
        #[inline(always)]
        fn $name(&mut self, n: $T) -> $T {
            n.visit_with(&n, &mut self.0);
            n
        }
    };
}

impl<V> Fold for ImmutableFolder<V>
where
    V: Visit,
{
    method!(fold_ident, Ident);
    method!(fold_span, Span);

    method!(fold_expr, Expr);
    method!(fold_decl, Decl);
    method!(fold_stmt, Stmt);
    method!(fold_pat, Pat);

    method!(fold_ts_type, TsType);

    method!(fold_module, Module);
    method!(fold_script, Script);
    method!(fold_program, Program);
}
