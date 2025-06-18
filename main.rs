use chrono::{NaiveDate, Local};

#[derive(Debug)]
struct Sarcina {
    text: String,
    categorie: String,
    prioritate: String,
    termen: Option<NaiveDate>,
}

fn main() {
    let filtru_categorie: Option<&str> = None;
    let filtru_prioritate: Option<&str> = None;
    let afiseaza_numai_viitoare = true;

    let azi = Local::now().date_naive();

    let mut lista = vec![
        Sarcina {
            text: "Finalizează raportul".to_string(),
            categorie: "Muncă".to_string(),
            prioritate: "Ridicată".to_string(),
            termen: Some(NaiveDate::from_ymd_opt(2025, 6, 30).unwrap()),
        },
        Sarcina {
            text: "Mergi la cumpărături".to_string(),
            categorie: "Personal".to_string(),
            prioritate: "Medie".to_string(),
            termen: None,
        },
        Sarcina {
            text: "Curăță garajul".to_string(),
            categorie: "Altele".to_string(),
            prioritate: "Scăzută".to_string(),
            termen: Some(NaiveDate::from_ymd_opt(2025, 7, 15).unwrap()),
        },
        Sarcina {
            text: "Rezervă bilete avion".to_string(),
            categorie: "Personal".to_string(),
            prioritate: "Ridicată".to_string(),
            termen: Some(NaiveDate::from_ymd_opt(2024, 12, 10).unwrap()),
        },
        Sarcina {
            text: "Pregătește prezentarea".to_string(),
            categorie: "Muncă".to_string(),
            prioritate: "Medie".to_string(),
            termen: None,
        },
    ];

    if let Some(cat) = filtru_categorie {
        lista = lista.into_iter().filter(|s| s.categorie == cat).collect();
    }

    if let Some(prio) = filtru_prioritate {
        lista = lista.into_iter().filter(|s| s.prioritate == prio).collect();
    }

    if afiseaza_numai_viitoare {
        lista = lista
            .into_iter()
            .filter(|s| match s.termen {
                Some(data) => data >= azi,
                None => true,
            })
            .collect();
    }

    lista.sort_by_key(|s| s.termen.unwrap_or(NaiveDate::from_ymd_opt(9999, 12, 31).unwrap()));

    println!("=== Lista To-Do ===");
    for (i, s) in lista.iter().enumerate() {
        let termen_text = s
            .termen
            .map(|d| d.format("%Y-%m-%d").to_string())
            .unwrap_or_else(|| "-".to_string());
        println!(
            "{}: [{}][{}][{}] {}",
            i + 1,
            s.categorie,
            s.prioritate,
            termen_text,
            s.text
        );
    }
}
