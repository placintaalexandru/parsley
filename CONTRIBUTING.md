# Contributing

The following is a set of contribution guidelines.

## Reporting bugs

Before creating bug reports, please check the GitHub issues as you might find out that you don't need to create one.
When you create a bug report, please include as many details as possible.

    Note: If you find a closed issue that seems like it is the same thing that you're experiencing,
    open a new issue and include a link to the original issue in the body of your new one.

### How do I submit a bug report?

Explain the problem and include additional details to help maintainers reproduce the problem:

1. Use a clear and descriptive title for the issue to identify the problem.
2. Describe the exact steps which reproduce the problem in as many details as possible (provide the manifest files
   you try to parse, if possible)
3. Provide specific examples to demonstrate the steps.
4. Describe the behavior you observed after following the steps and point out what exactly is the problem with that behavior.
5. Explain which behavior you expected to see instead **and why**.

## Feature requests

**New ideas are welcomed!**

Explain the idea and include as many additional details as possible:

1. Use a clear and descriptive title for the issue.
2. Describe your motivation and how other people could benefit from this
   change.
3. Include links to the related tools, alternative implementations or any other
   information sources.

### Backwards compatibility

Please note that maintaining backwards compatibility is critically important;
any changes that require a new major version to be published will be postponed
till there will be enough changes to make a new major release.

## Pull Requests

### Breaking changes

If your change introduces any new functionality or breaks the backwards
compatibility in any matter - **do not rush to create a Pull Request at all**.

Do not waste your time on that, check
[Backwards compatibility](#backwards-compatibility) section first for
motivation, and create an issue first, explain why you want to make this change
and let the discussion happen.

### Open a Pull Request

The Code you are contributing should pass the following checks:

1. Should change only one specific thing
2. Not raising any compiler errors or warnings
    - **do not use lint annotations to mask specific error**
3. Conforms to formatting rules (use `cargo fmt` command)
4. Not raising any lint warnings (use 
   ```shell
   cargo clippy --tests -- \
   -D clippy::uninlined-format-args \
   -D warnings \
   -A dead-code \
   -A deprecated \
   -A unknown-lints \
   -W clippy::missing_docs_in_private_items
   ```
   command)
5. All tests should pass (use `cargo test` command)

Now create a GitHub Pull Request with a patch:

1. Ensure the Pull Request description clearly describes the problem and solution
2. Include the relevant issue number if applicable
3. Ensure that all the checks from above pass
