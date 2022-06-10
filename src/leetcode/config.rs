use const_format::formatcp;

pub struct ConstConfig<'a> {
    pub url: Urls<'a>,
}

pub struct Urls<'a> {
    pub leetcode: LeetcodeUrls<'a>,
    pub github: GithubUrls<'a>,
}

pub struct LeetcodeUrls<'a> {
    pub base: &'a str,
    pub api: &'a str,
    pub graphql: &'a str,
    pub problems: &'a str,
    pub problems_all: &'a str,
    pub test: &'a str,
    pub submit: &'a str,
    pub submission: &'a str,
    pub submissions: &'a str,
    pub veriry: &'a str,
    pub github_login: &'a str,
}

pub struct GithubUrls<'a> {
    pub login: &'a str,
    pub session: &'a str,
}

pub const BASE: &'static str = "https://leetcode.cn";

pub const CONST_CONFIG: ConstConfig<'static> = ConstConfig {
    url: Urls {
        leetcode: LeetcodeUrls {
            base: BASE,
            api: formatcp!("{}/api", BASE),
            graphql: formatcp!("{}/graphql", BASE),
            problems: formatcp!("{}/problems/", BASE),
            problems_all: formatcp!("{}/api/problems/all", BASE),
            test: formatcp!("{}/problems/$slug/interpret_solution/", BASE),
            submit: formatcp!("{}/problems/$slug/submit/", BASE),
            submission: formatcp!("{}/api/submissions/detail/$id", BASE),
            submissions: formatcp!("{}/submissions/detail/$id", BASE),
            veriry: formatcp!("{}/submissions/detail/$id/check", BASE),
            github_login: formatcp!("{}/accounts/github/login/?next=%2F", BASE),
        },
        github: GithubUrls {
            login: "https://github.com/login",
            session: "https://github.com/session",
        },
    }
};
