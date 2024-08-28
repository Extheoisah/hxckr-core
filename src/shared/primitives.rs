pub enum Difficulty {
    Easy,
    Medium,
    Hard,
}

impl Difficulty {
    pub fn from_str(difficulty: &str) -> Result<Difficulty, &'static str> {
        match difficulty {
            "easy" => Ok(Difficulty::Easy),
            "medium" => Ok(Difficulty::Medium),
            "hard" => Ok(Difficulty::Hard),
            _ => Err("Invalid difficulty"),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            Difficulty::Easy => "easy",
            Difficulty::Medium => "medium",
            Difficulty::Hard => "hard",
        }
    }
}

pub enum ChallengeMode {
    FunctionalTest,
    Project,
}

impl ChallengeMode {
    pub fn from_str(mode: &str) -> Result<ChallengeMode, &'static str> {
        match mode {
            "functional_test" => Ok(ChallengeMode::FunctionalTest),
            "project" => Ok(ChallengeMode::Project),
            _ => Err("Invalid challenge mode"),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ChallengeMode::FunctionalTest => "functional_test",
            ChallengeMode::Project => "project",
        }
    }
}

pub enum UserRole {
    Admin,
    User,
}

impl UserRole {
    pub fn from_str(role: &str) -> Result<UserRole, &'static str> {
        match role {
            "admin" => Ok(UserRole::Admin),
            "user" => Ok(UserRole::User),
            _ => Err("Invalid user role"),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            UserRole::Admin => "admin",
            UserRole::User => "user",
        }
    }
}

pub enum ProgressStatus {
    Completed,
    InProgress,
    NotStarted,
}

impl ProgressStatus {
    pub fn from_str(status: &str) -> Result<ProgressStatus, &'static str> {
        match status {
            "completed" => Ok(ProgressStatus::Completed),
            "in_progress" => Ok(ProgressStatus::InProgress),
            "not_started" => Ok(ProgressStatus::NotStarted),
            _ => Err("Invalid progress status"),
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            ProgressStatus::Completed => "completed",
            ProgressStatus::InProgress => "in_progress",
            ProgressStatus::NotStarted => "not_started",
        }
    }
}
