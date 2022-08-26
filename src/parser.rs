pub(crate) struct Parser<'input> {
    input: &'input str,
}

impl<'input> Parser<'input> {
    pub(crate) fn new(input: &'input str) -> Self {
        Self { input }
    }
}
