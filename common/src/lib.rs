pub mod configuration;
pub mod persistence;

use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, PartialEq, Deserialize, FromRow)]
pub struct Record {
    pub id: String,
    pub date: NaiveDate,
    pub county: String,
    pub location: String,
    pub title: String,
    pub description: String,
}

pub enum RomanianCounty {
    Alba,
    Arad,
    Arges,
    Bacau,
    Bihor,
    BistritaNasaud,
    Botosani,
    Brasov,
    Braila,
    Buzau,
    CarasSeverin,
    Calarasi,
    Cluj,
    Constanta,
    Covasna,
    Dambovita,
    Dolj,
    Galati,
    Giurgiu,
    Gorj,
    Harghita,
    Hunedoara,
    Ialomita,
    Iasi,
    Ilfov,
    Maramures,
    Mehedinti,
    Mures,
    Neamt,
    Olt,
    Prahova,
    SatuMare,
    Salaj,
    Sibiu,
    Suceava,
    Teleorman,
    Timis,
    Tulcea,
    Vaslui,
    Valcea,
    Vrancea,
    Bucuresti, // The capital city, often considered a county-level administrative unit
}
