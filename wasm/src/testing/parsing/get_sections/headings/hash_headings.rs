use crate::{
    testing::{get_example_preferences, setup},
    tools::{
        parsing::get_sections,
        tokens::{HeadingLevel, MarkdownSection},
    },
};

// Only one level of headings.
#[test]
fn case_1() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_2() {
    setup();

    let input = r#"## Heading 2
## Heading 2
## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_3() {
    setup();

    let input = r#"## Heading 2
### Heading 3
#### Heading 4"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_4() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::Sub("### Heading 3".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

/// Random line breaks.
#[test]
fn case_5() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.

### Heading 3

Lorem Ipsum is simply dummy text of the printing and typesetting industry.

#### Heading 4
## Heading 2"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("#### Heading 4".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

// Two levels of hash headings.
#[test]
fn case_6() {
    setup();

    let input = r#"## Heading 2
Lorem Ipsum is simply dummy text of the printing and typesetting industry.
### Heading 3
Lorem Ipsum is simply dummy text of the printing and typesetting industry."#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Content(
            "Lorem Ipsum is simply dummy text of the printing and typesetting industry."
                .to_string(),
        ),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_7() {
    setup();

    let input = r#"## Heading 2
- "#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top(r#"## Heading 2"#.to_string())),
        MarkdownSection::Content("-".to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn case_8() {
    setup();

    let input = r#"### Heading 3
## Heading 2
- "#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::FirstSub("### Heading 3".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("## Heading 2".to_string())),
        MarkdownSection::Content(r#"-"#.to_string()),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn hash_headings_without_title_names() {
    setup();

    let input = r#"#
##
##
##
##
###
###
####
####
##
#
"#;

    let expected_output = vec![
        MarkdownSection::Heading(HeadingLevel::Top("#".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("###".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("###".to_string())),
        MarkdownSection::Heading(HeadingLevel::FirstSub("####".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("####".to_string())),
        MarkdownSection::Heading(HeadingLevel::Sub("##".to_string())),
        MarkdownSection::Heading(HeadingLevel::Top("#".to_string())),
    ];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}

#[test]
fn invalid_input_1() {
    setup();

    let input = r#"##Heading 2
###Heading 3
####Heading 4"#;

    let expected_output = vec![MarkdownSection::Content(
        r#"##Heading 2
###Heading 3
####Heading 4"#
            .to_string(),
    )];

    assert_eq!(
        get_sections(input, &get_example_preferences()).unwrap(),
        expected_output
    );
}
