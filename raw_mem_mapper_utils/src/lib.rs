use const_panic::concat_panic;

#[track_caller]
pub const fn generate_padding_size_aligned(
    field_name: &str,
    offset: usize,
    prior_offset: usize,
    prior_type_size: usize,
) -> usize {
    let result = offset - (prior_offset + prior_type_size);
    if (offset - prior_offset) % 8 != 0 {
        concat_panic! {
            "\nPadding size must be a multiple of 8\n",
            "field_name: ", field_name, " at offset: ", offset, " with reminder: ", result % 8, "\n"
        }
    }
    result
}
