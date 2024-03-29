use crate::{
    testing::{get_example_preferences, setup},
    tools::{formatting::get_formatted_string, parsing::get_sections},
};

#[test]
fn case_1() {
    setup();

    let input = r#"## Heading 2

### heading 3
```ts
console.log("Hello World");
```

Heading 1
==="#;

    let sections = get_sections(input, &get_example_preferences()).unwrap();
    let output = get_formatted_string(sections, &get_example_preferences()).unwrap();
    let expected_output = r#"## Heading 2

### heading 3
```ts
console.log("Hello World");
```



Heading 1
==="#;

    assert_eq!(output, expected_output);
}

#[test]
fn case_2() {
    setup();

    let input = r#"Heading 1
==

Heading 1
===

### Heading 3"#;

    let sections = get_sections(input, &get_example_preferences()).unwrap();
    let output = get_formatted_string(sections, &get_example_preferences()).unwrap();
    let expected_output = r#"Heading 1
==



Heading 1
===

### Heading 3"#;

    assert_eq!(output, expected_output);
}
