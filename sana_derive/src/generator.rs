use proc_macro2::TokenStream;
use quote::{format_ident, quote};

use sana_core::ir::{Op, Ir};
use heck::ShoutySnakeCase;

use crate::SanaSpec;
use proc_macro_error::abort;
use syn::Ident;

pub(crate) fn generate(spec: SanaSpec) -> TokenStream {
    let dfa = match spec.rules.construct_dfa() {
        Ok(dfa) => dfa,
        Err(sana_core::Error::AmbiguityError(_, i)) =>
            abort!(spec.variants[i], "Ambigiuous rule"),
    };

    let ir = Ir::from_automata(dfa);

    let enum_ident = spec.enum_ident;
    let enum_const_name = enum_ident.to_string()
        .to_shouty_snake_case();
    let ir_var = format_ident!("_{}_IR", enum_const_name);
    let ir_code = generate_ir(enum_ident.clone(), ir, &spec.variants);
    let error = spec.terminal;

    quote! {
        const #ir_var: &'static [sana::ir::Op<#enum_ident>] = #ir_code;

        impl sana::Sana for #enum_ident {
            const ERROR: Self = #enum_ident::#error;

            fn ir() -> &'static [sana::ir::Op<Self>] { #ir_code }
        }
    }
}

fn generate_ir(enum_ident: Ident, ir: Ir<usize>, variants: &[Ident]) -> TokenStream {
    let code = ir.flatten();
    let mut ops = vec![];

    for op in code {
        let op = match op {
            Op::Shift => quote! {
                sana::ir::Op::Shift
            },
            Op::JumpMatches { from, to, on_success } => quote! {
                sana::ir::Op::JumpMatches {
                    from: #from,
                    to: #to,
                    on_success: #on_success,
                }
            },
            Op::JumpNotMatches { from, to, on_failure } => quote! {
                sana::ir::Op::JumpNotMatches {
                    from: #from,
                    to: #to,
                    on_failure: #on_failure,
                }
            },
            Op::LoopMatches { from, to } => quote! {
                sana::ir::Op::LoopMatches {
                    from: #from,
                    to: #to,
                }
            },
            Op::Jump(loc) => quote! {
                sana::ir::Op::Jump(#loc)
            },
            Op::Set(act) => {
                let var = &variants[act];

                quote! { sana::ir::Op::Set(#enum_ident::#var) }
            },
            Op::Halt => quote! {
                sana::ir::Op::Halt
            },
        };

        ops.push(op)
    }

    quote! {
        &[
            #(#ops),*
        ]
    }
}
