pub mod analytics;
pub mod campaign;
pub mod company;
pub mod employee;
pub mod template;

pub use company::{Company, CreateCompany, UpdateCompany};
pub use employee::{Employee, CreateEmployee, UpdateEmployee};
pub use campaign::{Campaign, CreateCampaign, UpdateCampaign};
pub use template::{Template, CreateTemplate, UpdateTemplate};
