use fervid_core::{Node, VSlotDirective};
use lazy_static::lazy_static;
use swc_core::ecma::{
    atoms::JsWord,
    visit::{Visit, VisitWith},
};

lazy_static! {
    static ref JS_BUILTINS: [JsWord; 7] =
        ["true", "false", "null", "undefined", "Array", "Set", "Map"].map(JsWord::from);
}

#[derive(Debug)]
pub struct Scope {
    variables: Vec<JsWord>,
    parent: u32,
}

#[derive(Default, Debug)]
pub struct ScopeHelper {
    pub template_scopes: Vec<Scope>,
    setup_vars: Vec<JsWord>,
    props_vars: Vec<JsWord>,
    data_vars: Vec<JsWord>,
    options_vars: Vec<JsWord>,
    globals: Vec<JsWord>,
}

#[derive(Debug, PartialEq)]
pub enum VarScopeDescriptor {
    Builtin,
    Data,
    Global,
    Props,
    Options,
    Setup,
    Template(u32), // I know it takes 4 extra bytes, but this is more convenient
    Unknown,
}

impl VarScopeDescriptor {
    pub fn get_prefix(&self) -> &'static str {
        match self {
            Self::Builtin => "",
            Self::Data => "$data.",
            Self::Global => "_ctx.",
            Self::Props => "$props.",
            Self::Options => "$options.",
            Self::Setup => "$setup.",
            Self::Template(_) => "",
            Self::Unknown => "_ctx.",
        }
    }
}

impl ScopeHelper {
    pub fn find_scope_of_variable(
        &self,
        starting_scope: u32,
        variable: &JsWord,
    ) -> VarScopeDescriptor {
        let mut current_scope_index = starting_scope;

        // Macro to check if the variable is in the slice/Vec and conditionally return
        macro_rules! check_scope {
            ($vars: expr, $ret_descriptor: expr) => {
                if $vars.iter().any(|it| it == variable) {
                    return $ret_descriptor;
                }
            };
        }

        // Check builtins and globals
        check_scope!(JS_BUILTINS, VarScopeDescriptor::Builtin);
        check_scope!(self.globals, VarScopeDescriptor::Global);

        // Check template scope
        while let Some(current_scope) = self.template_scopes.get(current_scope_index as usize) {
            // Check variable existence in the current scope
            let found = current_scope.variables.iter().find(|it| *it == variable);
            if let Some(_) = found {
                return VarScopeDescriptor::Template(current_scope_index);
            }

            // Check if we reached the root scope, it will have itself as a parent
            if current_scope.parent == current_scope_index {
                break;
            }

            // Go to parent
            current_scope_index = current_scope.parent;
        }

        // Check setup vars, props, data and options
        check_scope!(self.setup_vars, VarScopeDescriptor::Setup);
        check_scope!(self.props_vars, VarScopeDescriptor::Props);
        check_scope!(self.data_vars, VarScopeDescriptor::Data);
        check_scope!(self.options_vars, VarScopeDescriptor::Options);

        VarScopeDescriptor::Unknown
    }

    /// Transforms an AST by assigning the scope identifiers to Nodes
    /// The variables introduced in `v-for` and `v-slot` are recorded to the ScopeHelper
    pub fn transform_and_record_ast(&mut self, ast: &mut [Node]) {
        // Pre-allocate template scopes to at least the amount of root AST nodes
        if self.template_scopes.len() == 0 && ast.len() != 0 {
            self.template_scopes.reserve(ast.len());

            // Add scope 0.
            // It may be left unused, as it's reserved for some global template vars (undecided)
            self.template_scopes.push(Scope {
                variables: vec![],
                parent: 0,
            });
        }

        for node in ast {
            self.walk_ast_node(node, 0)
        }
    }

    fn walk_ast_node(&mut self, node: &mut Node, current_scope_identifier: u32) {
        match node {
            Node::Element(element_node) => {
                // A scope to use for both the current node and its children (as a parent)
                let mut scope_to_use = current_scope_identifier;

                // Finds a `v-for` or `v-slot` directive when in ElementNode
                if let Some(ref directives) = element_node.starting_tag.directives {
                    let v_for = directives.v_for.as_ref();
                    let v_slot = directives.v_slot.as_ref();

                    // Create a new scope
                    if v_for.is_some() || v_slot.is_some() {
                        // New scope will have ID equal to length
                        scope_to_use = self.template_scopes.len() as u32;
                        self.template_scopes.push(Scope {
                            variables: vec![],
                            parent: current_scope_identifier,
                        });
                    }

                    if let Some(v_for) = v_for {
                        // We only care of the left hand side variables
                        let introduced_variables = &v_for.itervar;

                        // Get the needed scope and collect variables to it
                        let mut scope = &mut self.template_scopes[scope_to_use as usize];
                        Self::collect_variables(introduced_variables, &mut scope);
                    } else if let Some(VSlotDirective { value: Some(v_slot_value), .. }) = v_slot {
                        // Get the needed scope and collect variables to it
                        let mut scope = &mut self.template_scopes[scope_to_use as usize];
                        Self::collect_variables(v_slot_value, &mut scope);
                    }
                }

                // Update Node's scope
                element_node.template_scope = scope_to_use;

                // Walk children
                for mut child in element_node.children.iter_mut() {
                    self.walk_ast_node(&mut child, scope_to_use);
                }
            }

            // For dynamic expression, just update the scope
            Node::Interpolation(interpolation) => {
                interpolation.template_scope = current_scope_identifier;
            }

            _ => {}
        }
    }

    fn collect_variables(expr: &impl VisitWith<IdentifierVisitor>, scope: &mut Scope) {
        let mut visitor = IdentifierVisitor { collected: vec![] };

        expr.visit_with(&mut visitor);

        scope.variables.reserve(visitor.collected.len());
        for collected in visitor.collected {
            scope.variables.push(collected.sym)
        }
    }
}

struct IdentifierVisitor {
    collected: Vec<swc_core::ecma::ast::Ident>,
}

impl Visit for IdentifierVisitor {
    fn visit_ident(&mut self, n: &swc_core::ecma::ast::Ident) {
        self.collected.push(n.to_owned());
    }

    fn visit_object_lit(&mut self, n: &swc_core::ecma::ast::ObjectLit) {
        self.collected.reserve(n.props.len());

        for prop in n.props.iter() {
            let swc_core::ecma::ast::PropOrSpread::Prop(prop) = prop else {
                continue;
            };

            // This is shorthand `a` in `{ a }`
            let shorthand = prop.as_shorthand();
            if let Some(ident) = shorthand {
                self.collected.push(ident.to_owned());
                continue;
            }

            // This is key-value `a: b` in `{ a: b }`
            let Some(keyvalue) = prop.as_key_value() else { continue };

            // We only support renaming things (therefore value must be an identifier)
            let Some(value) = keyvalue.value.as_ident() else { continue };
            self.collected.push(value.to_owned());
        }
    }
}
