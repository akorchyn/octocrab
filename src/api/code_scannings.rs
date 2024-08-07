//! The code scanning API.
use crate::{models, params, Octocrab, Result};

mod list;

/// Handler for GitHub's code scanning API.
///
/// Created with [`Octocrab::code_scannings`].
pub struct CodeScanningHandler<'octo> {
    crab: &'octo Octocrab,
    owner: String,
    repo: Option<String>,
}

impl<'octo> CodeScanningHandler<'octo> {
    pub(crate) fn new(crab: &'octo Octocrab, owner: String, repo: Option<String>) -> Self {
        Self { crab, owner, repo }
    }

    /// Gets an code scanning from the repository.
    /// ```no_run
    /// # async fn run() -> octocrab::Result<()> {
    /// # let octocrab = octocrab::Octocrab::default();
    /// let code_scanning = octocrab.code_scannings("owner", "repo").get(3).await?;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn get(&mut self, number: u64) -> Result<models::code_scannings::CodeScanningAlert> {
        let route = format!(
            "/repos/{owner}/{repo}/code-scanning/alerts/{number}",
            owner = self.owner,
            repo = self.repo.as_mut().expect("Repository must be specified"),
            number = number,
        );

        self.crab.get(route, None::<&()>).await
    }

    /// List code scannings in the repository.
    pub fn list(&self) -> list::ListCodeScanningsBuilder<'_, '_> {
        list::ListCodeScanningsBuilder::new(self)
    }
}
