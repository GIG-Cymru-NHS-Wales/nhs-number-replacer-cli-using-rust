# NHS Number replacer CLI

**[documentation](https://docs.rs/nhs-number-replacer-cli/)**
•
**[source](https://github.com/GIG-Cymru-NHS-Wales/nhs-number-replacer-cli)**
•
**[llms.txt](https://raw.githubusercontent.com/GIG-Cymru-NHS-Wales/nhs-number-replacer-cli/refs/heads/main/llms.txt)**
•
**[crate](https://crates.io/crates/nhs-number-replacer-cli)**
•
**[email](mailto:joel.henderson@wales.nhs.uk)**

National Health Service (NHS) Number replacer command line interface (CLI).

## Steps

This program does these steps:

1. Read each line from standard input.

2. Match a regex pattern that is essentially "### ### ####".

3. Replace the match with a NHS Number that is a testable random sample i.e. in the NHS Number reserved range of "999 000 0000" to "999 999 9999".

4. Output each line to standard output.

## Example

Run:

```sh
echo "111 111 1111" | nhs-number-replacer-cli
```

Output:

```stdout
999 265 6328
```
