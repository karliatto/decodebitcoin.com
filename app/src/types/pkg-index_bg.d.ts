declare module '../../../pkg/index_bg.js' {
  // Minimal type declarations for WASM bindings
  export function __wbg_set_wasm(exports: WebAssembly.Exports): void
  export function decode(transaction_hex: string): string
  export function xor(a: string, b: string): string
  export function policy(s: string): string
}

declare module './../../../pkg/index_bg.js' {
  export function __wbg_set_wasm(exports: WebAssembly.Exports): void
  export function decode(transaction_hex: string): string
  export function xor(a: string, b: string): string
  export function policy(s: string): string
}


