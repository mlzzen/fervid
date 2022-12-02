use super::codegen::CodegenContext;

pub static CREATE_ELEMENT_VNODE: &str = "_createElementVNode";
pub static CREATE_TEXT_VNODE: &str = "_createTextVNode";
pub static TO_DISPLAY_STRING: &str = "_toDisplayString";
pub static WITH_MODIFIERS: &str = "_withModifiers";

#[derive(Clone, Copy)]
pub enum VueImports {
  CreateElementVNode,
  CreateTextVNode,
  ToDisplayString,
  WithModifiers
}

static ALL_IMPORTS: [VueImports; 4] = [VueImports::CreateElementVNode, VueImports::CreateTextVNode, VueImports::ToDisplayString, VueImports::WithModifiers];

impl <'a> CodegenContext <'a> {
  pub fn add_to_imports(self: &mut Self, vue_import: VueImports) {
    self.used_imports |= Self::get_import_mask_bit(vue_import);
  }

  pub fn get_import_str(vue_import: VueImports) -> &'static str {
    match vue_import {
      VueImports::CreateElementVNode => CREATE_ELEMENT_VNODE,
      VueImports::CreateTextVNode => CREATE_TEXT_VNODE,
      VueImports::ToDisplayString => TO_DISPLAY_STRING,
      VueImports::WithModifiers => WITH_MODIFIERS
    }
  }

  pub fn get_and_add_import_str(self: &mut Self, vue_import: VueImports) -> &'static str {
    self.add_to_imports(vue_import);

    Self::get_import_str(vue_import)
  }

  pub fn generate_imports_string(self: &Self) -> String {
    let imports_mask = self.used_imports;
    let mut result = String::from("import { ");
    let mut has_first_import = false;

    for import in ALL_IMPORTS.iter() {
      if Self::get_import_mask_bit(*import) & imports_mask == 0 {
        continue;
      }

      if has_first_import {
        result.push_str(", ");
      }

      let import_str = Self::get_import_str(*import);
      result.push_str(&import_str[1..]);
      result.push_str(" as ");
      result.push_str(import_str);

      has_first_import = true;
    }

    result.push_str(" } from \"vue\"");

    result
  }

  fn get_import_mask_bit(vue_import: VueImports) -> u64 {
    match vue_import {
      VueImports::CreateElementVNode => 1,
      VueImports::CreateTextVNode => 2,
      VueImports::ToDisplayString => 4,
      VueImports::WithModifiers => 8
    }
  }
}
