use quote::quote;

/// Behaves like the `?` operator, unwrapping a [Result] or [Option] but will break out of a
/// scope labeled with the `'block` label if a [None] or [Err] is encountered.  This can be used
/// to achieve something that resembles try-catch.
///
/// ```
/// use break_block_macro::bb;
///
/// let result = 'block: {
///     let one = bb!(Ok("one"));
///     assert_eq!(one, "one");
///
///     let _two = bb!(Err("two"));
///     Ok("three")
/// };
/// assert_eq!(result, Err("two"));
/// ```
//
// NOTE: This is a proc macro to get around scoping hygene for declarative macros that doesn't
//  allow labels to cross scope bounds
#[proc_macro]
pub fn bb(arg: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let arg: proc_macro2::TokenStream = arg.into();
    quote! {
        {
            let result = #arg;
            if someok::IsSuccess::is_success(&result) {
                result.unwrap()
            } else {
                break 'block result
            }
        }
    }.into()
}
