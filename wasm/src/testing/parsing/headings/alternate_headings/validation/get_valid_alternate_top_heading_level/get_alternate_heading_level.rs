use crate::tools::parsing::headings::alternate_headings::validation::get_valid_alternate_top_heading_level::get_alternate_heading_level;

#[test]
fn heading_1() {
    let input = "===";

    let output = get_alternate_heading_level(input).unwrap();
    let expected_out: usize = 1;

    assert_eq!(output, expected_out);
}

#[test]
fn heading_2() {
    let input = "---";

    let output = get_alternate_heading_level(input).unwrap();
    let expected_out: usize = 2;

    assert_eq!(output, expected_out);
}
