use clap::Parser;

#[derive(Parser, Debug, Default, Clone)]
pub struct FakeUserConfig {
    /// The number of fake users to create at startup.
    ///
    /// The higher this number is the more direct user load there will be, so balance this with
    /// other types of fake activity.
    ///
    /// See also [`FakeConfig`](crate::dev::fake).
    #[arg(long, default_value_t = 3000)]
    pub fake_user_count: u16,
}
