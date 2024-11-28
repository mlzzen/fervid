/* tslint:disable */
/* eslint-disable */

/* auto-generated by NAPI-RS */

/** Raw options passed from the Node.js side */
export interface FervidJsCompilerOptions {
  /** Apply production optimizations. Default: false */
  isProduction?: boolean
  /**
   * TODO Support SSR
   * Enable SSR. Default: false
   */
  ssr?: boolean
  /**
   * TODO Find a performant solution to source-maps
   * TODO Implement source-maps
   * Enable source maps
   */
  sourceMap?: boolean
  /** Script compilation options */
  script?: FervidJsCompilerOptionsScript
  /** Template compilation options */
  template?: FervidJsCompilerOptionsTemplate
  /** Style compilation options */
  style?: FervidJsCompilerOptionsStyle
  /**
   * TODO Regex handling logic is needed (plus sanitation)
   * TODO Implement custom element mode (low priority)
   * Transform Vue SFCs into custom elements.
   *  - `true`: all `*.vue` imports are converted into custom elements
   *  - `string | RegExp`: matched files are converted into custom elements
   * Default: files ending with `.ce.vue`
   */
  customElement?: undefined
}
export interface FervidJsCompilerOptionsTemplate {}
export interface FervidJsCompilerOptionsScript {
  /**
   * Ignored
   * Hoist <script setup> static constants.
   * - Only enabled when one `<script setup>` exists.
   * Default: true
   */
  hoistStatic?: boolean
  /** Produce source maps */
  sourceMap?: boolean
}
export interface FervidJsCompilerOptionsStyle {
  /** Ignored */
  trim?: boolean
}
export interface FervidCompileOptions {
  /** Scope ID for prefixing injected CSS variables */
  id: string
  /** Filename is used for automatic component name inference and self-referential imports */
  filename: string
  /**
   * Is the currently compiled file a custom element.
   * To give more flexibility, this option only accepts a boolean, allowing to compute the value on the JS side,
   * instead of relying on a hacky RegEx/JS function calls from the Fervid side.
   */
  isCustomElement?: boolean
  /** Generate a const instead of default export */
  genDefaultAs?: string
  /** Enable, disable or error on props destructure */
  propsDestructure?: boolean | 'error'
  /** Whether setup bindings need to be serialized */
  outputSetupBindings?: boolean
}
export interface CompileResult {
  code: string
  styles: Array<Style>
  errors: Array<SerializedError>
  customBlocks: Array<CustomBlock>
  sourceMap?: string
  setupBindings?: Record<string, BindingTypes> | undefined
}
export interface Style {
  code: string
  isCompiled: boolean
  lang: string
  isScoped: boolean
}
export interface CustomBlock {
  content: string
  lo: number
  hi: number
  tagName: string
}
export interface SerializedError {
  lo: number
  hi: number
  message: string
}
/**
 * This is a copied enum from `fervid_core` with `napi` implementation to avoid littering the core crate.
 *
 * The type of a binding (or identifier) which is used to show where this binding came from,
 * e.g. `Data` is for Options API `data()`, `SetupRef` if for `ref`s and `computed`s in Composition API.
 *
 * <https://github.com/vuejs/core/blob/020851e57d9a9f727c6ea07e9c1575430af02b73/packages/compiler-core/src/options.ts#L76>
 */
export const enum BindingTypes {
  /** returned from data() */
  DATA = 0,
  /** declared as a prop */
  PROPS = 1,
  /**
   * a local alias of a `<script setup>` destructured prop.
   * the original is stored in __propsAliases of the bindingMetadata object.
   */
  PROPS_ALIASED = 2,
  /** a let binding (may or may not be a ref) */
  SETUP_LET = 3,
  /**
   * a const binding that can never be a ref.
   * these bindings don't need `unref()` calls when processed in inlined
   * template expressions.
   */
  SETUP_CONST = 4,
  /** a const binding that does not need `unref()`, but may be mutated. */
  SETUP_REACTIVE_CONST = 5,
  /** a const binding that may be a ref */
  SETUP_MAYBE_REF = 6,
  /** bindings that are guaranteed to be refs */
  SETUP_REF = 7,
  /** declared by other options, e.g. computed, inject */
  OPTIONS = 8,
  /** a literal constant, e.g. 'foo', 1, true */
  LITERAL_CONST = 9,
  /** a `.vue` import or `defineComponent` call */
  COMPONENT = 10,
  /** an import which is not a `.vue` or `from 'vue'` */
  IMPORTED = 11,
  /** a variable from the template */
  TEMPLATE_LOCAL = 12,
  /** a variable in the global Javascript context, e.g. `Array` or `undefined` */
  JS_GLOBAL = 13,
  /** a non-resolved variable, presumably from the global Vue context */
  UNRESOLVED = 14,
}
export type FervidJsCompiler = Compiler
/** Fervid: a compiler for Vue.js written in Rust */
export declare class Compiler {
  options: FervidJsCompilerOptions
  constructor(options?: FervidJsCompilerOptions | undefined | null)
  compileSync(source: string, options: FervidCompileOptions): CompileResult
  compileAsync(source: string, options: FervidCompileOptions, signal?: AbortSignal | undefined | null): Promise<unknown>
}
