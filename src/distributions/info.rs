pub struct OneDist {
    pub discrete: bool,
}

pub struct Info {
    pub binomial: OneDist,
    pub chi_squared: OneDist,
    pub exponential: OneDist,
    pub geometric: OneDist,
    pub negative_binomial: OneDist,
    pub normal: OneDist,
    pub poisson: OneDist,
    pub uniform: OneDist,
    pub fisher_snedecor: OneDist,
    pub students_t: OneDist,
}

pub const INFO: Info = Info {
    // discrete
    binomial: OneDist { discrete: true },
    negative_binomial: OneDist { discrete: true },
    poisson: OneDist { discrete: true },
    geometric: OneDist { discrete: true },
    // continuous
    chi_squared: OneDist { discrete: false },
    exponential: OneDist { discrete: false },
    normal: OneDist { discrete: false },
    uniform: OneDist { discrete: false },
    fisher_snedecor: OneDist { discrete: false },
    students_t: OneDist { discrete: false },
};
