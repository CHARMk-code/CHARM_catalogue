use std::{collections::HashMap, fs, path::PathBuf};

use chrono::DateTime;
use sqlx::{Pool, Postgres};

use crate::{
    services::{
        self,
        batch::{process_batch_zip, BatchProcessError},
        company::CompanyDB,
        layout::LayoutDB,
        map::MapDB,
        prepage::PrepageDB,
        shortcut::ShortcutDB,
        tag::TagDB,
    },
    tests::{create_test_folders, TEST_STORAGE, TEST_UPLOAD},
};

#[sqlx::test()]
async fn full_parsing_of_zip_file_should_populate_db(
    db: Pool<Postgres>,
) -> Result<(), BatchProcessError> {
    let zip_path: PathBuf =
        (env!("CARGO_MANIFEST_DIR").to_owned() + "/src/tests/batch/batch_test.zip").into();

    let base_path = create_test_folders().unwrap();

    let upload_path = base_path.join(TEST_UPLOAD);
    let storage_path = base_path.join(TEST_STORAGE);

    process_batch_zip(&db, &zip_path, &upload_path, &storage_path).await?;

    // Expected results

    // Doesn't verify all data entered however checks a couple to be sure the data has been
    // properly handled
    let expected_companies_amount = 160;
    let expected_companies = vec![
            CompanyDB {
            id:  1,
            last_updated:  DateTime::parse_from_str("2023-08-07T15:11:57.824Z", "%+").unwrap().into(),
            active:  true,
            charmtalk:  false,
            name:  "AFRY".to_string(),
            description:  "AFRY is a European leader in engineering, design, and advisory services, with a global reach. We accelerate the transition towards a sustainable society. We are 17,000 devoted experts in infrastructure, industry, energy and digitalisation, creating sustainable solutions for generations to come. Making Future".to_string(),
            unique_selling_point:  "Our culture, our work with Inclusion & Diversity, our Projects & Clients, our Global reach.".to_string(),
            summer_job_description:  "".to_string(),
            summer_job_link:  "".to_string(),
            summer_job_deadline:  DateTime::parse_from_str("2023-08-07T15:11:57.824Z", "%+").unwrap().into(),
            contacts:  "Nico Hengst".to_string(),
            contact_email:  "Talent Sourcing Specialist".to_string(),
            employees_world:  9000,
            employees_sweden:  19000,
            website:  "https://www.afry.com".to_string(),
            talk_to_us_about:  "How we  accelerate the transition towards a sustainable society. What type of career opportunities we offer and our work with Inclusion & Diversity.".to_string(),
            logo:  "AFRY-Logotype-Horizontal_Explainer_Black_ac97e95afec665f301d8e16073c76b74.svg".to_string(),
            map_image:  3,
            booth_number:  225,
            tags: Some(vec![7,4,2,3,6,9,11,13,16,20,19,17,21,22,23,25,26,27,28,40])
            },
            CompanyDB {
            id:  17,
            last_updated:  DateTime::parse_from_str("2023-08-07T15:11:57.824Z", "%+").unwrap().into(),
            active:  true,
            charmtalk:  false,
            name:  "Beyond Gravity".to_string(),
            description:  "Beyond Gravity is the first startup to combine agility, speed and innovation with decades of experience and proven quality. Around 1600 employees at 12 locations in six countries develop and manufacture products for satellites and launch vehicles with the goal of advancing humankind and enabling the exploration of the world and beyond. At Beyond Gravity Sweden, about 400 people work with the development and manufacture of advanced equipment for use in space.".to_string(),
            unique_selling_point:  "Beyond Gravity is the first startup to combine agility, speed and innovation with decades of experience and proven quality.".to_string(),
            summer_job_description:  "".to_string(),
            summer_job_link:  "".to_string(),
            summer_job_deadline:  DateTime::parse_from_str("2023-08-07T15:11:57.824Z", "%+").unwrap().into(),
            contacts:  "Lena Johnsen".to_string(),
            contact_email:  "HR Manager".to_string(),
            employees_world:  350,
            employees_sweden:  1600,
            website:  "https://www.beyondgravity.com".to_string(),
            talk_to_us_about:  "The space industry is currently undergoing an exciting change where the traditional space industry is being challenged by \"New Space\".".to_string(),
            logo:  "bg_positiv_cmyk_19faf3f62c7fdae75db3f0d417a778e3.svg".to_string(),
            map_image:  1,
            booth_number:  10,
            tags: Some(vec![4,2,3,16,17,21,22,23,27,28,29,37])
            },
            CompanyDB {
            id:  44,
            last_updated:  DateTime::parse_from_str("2023-08-07T15:11:57.824Z", "%+").unwrap().into(),
            active:  true,
            charmtalk:  false,
            name:  "EasyMining".to_string(),
            description:  "EasyMining is an innovation company dedicated to closing nutrient cycles. We are owned by the Swedish environmental company Ragn-Sells. Our objective is to create new circular material flows in an efficient commercial way. We do this by inventing and implementing new technology that uses chemical solutions to recycle important materials. We are currently designing and building chemical process plants for our Aqua2N and Ash2Phos technologies.".to_string(),
            unique_selling_point:  "We are one of the world’s leading deep tech waste recovery companies where waste is our clean product's starting point.".to_string(),
            summer_job_description:  "".to_string(),
            summer_job_link:  "".to_string(),
            summer_job_deadline:  DateTime::parse_from_str("2023-08-07T15:11:57.824Z", "%+").unwrap().into(),
            contacts:  "Mimmi Lundin".to_string(),
            contact_email:  "Administrator".to_string(),
            employees_world:  38,
            employees_sweden:  40,
            website:  "https://www.easymining.se".to_string(),
            talk_to_us_about:  "Our news published on our webpage: easymining.se".to_string(),
            logo:  "EasyMining_logotyp_tagline_v2_1a76e258d0006e1784af09ef4b46c046.svg".to_string(),
            map_image:  1,
            booth_number:  8,
            tags: Some(vec![4,2,3,9,16,20,19,17,22,23,25,24,28,29,31])
            },
            CompanyDB {
            id:  53,
            last_updated:  DateTime::parse_from_str("2023-08-07T15:11:57.824Z", "%+").unwrap().into(),
            active:  true,
            charmtalk:  false,
            name:  "Eurocon Consulting AB".to_string(),
            description:  "Sustainable comprehensive solutions to create long-term benefits. Eurocon offers cutting-edge expertise in the form of qualified services and systems for the implementation of industrial projects in the processing and manufacturing industry, infrastructure and information systems of all sizes and complexities. The services span the entire life cycle of an investment, from feasibility studies to implementation projects and maintenance.".to_string(),
            unique_selling_point:  "Eurocon accelerates the transition to a sustainable society by helping clients reduce emissions, lower energy consumption and optimise resource use.".to_string(),
            summer_job_description:  "".to_string(),
            summer_job_link:  "".to_string(),
            summer_job_deadline:  DateTime::parse_from_str("2023-08-07T15:11:57.824Z", "%+").unwrap().into(),
            contacts:  "Jan Östman".to_string(),
            contact_email:  "HR Director".to_string(),
            employees_world:  285,
            employees_sweden:  285,
            website:  "https://eurocon.se".to_string(),
            talk_to_us_about:  "Sustainable projects, career paths and opportunities to develop at Eurocon, located in ten locations in Sweden.".to_string(),
            logo:  "Eurocon_st_ende_logo_RGB_48d8bf504493ca3f3c8847cb0c20a328.svg".to_string(),
            map_image:  1,
            booth_number:  41,
            tags: Some(vec![7,4,2,3,6,9,8,12,11,15,13,16,20,19,17,21,22,23,27,28,29,32])
            },
            CompanyDB {
            id:  94,
            last_updated:  DateTime::parse_from_str("2023-08-07T15:11:57.825Z", "%+").unwrap().into(),
            active:  true,
            charmtalk:  false,
            name:  "Mpya Sci & Tech".to_string(),
            description:  "Mpya Sci & Tech is created by and for people who love science and technology. We are a consulting and recruitment company, built by people with years of experience from the industry. We are not here to do the same thing as others and have shaped a progressive and edgy company in our own way. We apply niche recruitment and consulting with focus on the candidate´s perspective. At Mpya Sci & Tech, we believe in a sustainable work life and together we build a culture where you can Be excitedly curious, Be inclusive and allowing and Be who you are.".to_string(),
            unique_selling_point:  "We believe that life is much more than just work. Or as we say – the more life you live, the more passion and well-being you bring to work.".to_string(),
            summer_job_description:  "".to_string(),
            summer_job_link:  "".to_string(),
            summer_job_deadline:  DateTime::parse_from_str("2023-08-07T15:11:57.825Z", "%+").unwrap().into(),
            contacts:  "Maja Palmqvist".to_string(),
            contact_email:  "Talent Advisor".to_string(),
            employees_world:  115,
            employees_sweden:  115,
            website:  "https://mpyascitech.com/".to_string(),
            talk_to_us_about:  "At Mpya Sci & Tech you as an Mployee contribute to the company´s development by being a part of different initiatives and decisions.".to_string(),
            logo:  "logga_h_guppl_st_e530f618058a728a4b8515f7df9e6790.svg".to_string(),
            map_image:  3,
            booth_number:  251,
            tags: Some(vec![4,3,9,13,16,19,17,22,23,28,29,31])
            },           CompanyDB {
            id:  109,
            last_updated:  DateTime::parse_from_str("2023-08-07T15:11:57.825Z", "%+").unwrap().into(),
            active:  true,
            charmtalk:  false,
            name:  "Preem AB".to_string(),
            description:  "Preem is Sweden’s largest fuel company. Our vision is to lead the transition toward a sustainable society. By 2035 Preem will produce 5 million cubic meters of renewable fuels and achieve climate neutrality. Our two refineries are among the most modern and environmentally friendly in Europe with a refining capacity of over 18 million cubic meters per year. The business we are in includes production, sales and distribution as well as trading and sale of goods.".to_string(),
            unique_selling_point:  "Preem aims to be a safe, inclusive and open workplace offering excellent opportunities for all our employees. At Preem, it is only natural for us to lead the transition toward a sustainable society.".to_string(),
            summer_job_description:  "".to_string(),
            summer_job_link:  "".to_string(),
            summer_job_deadline:  DateTime::parse_from_str("2023-08-07T15:11:57.825Z", "%+").unwrap().into(),
            contacts:  "Frida Carlsson Green".to_string(),
            contact_email:  "Employer Branding Manager".to_string(),
            employees_world:  1500,
            employees_sweden:  1500,
            website:  "https://www.preem.se".to_string(),
            talk_to_us_about:  "Talk to us about Preem´s role in the sustainable transition and how our engineers contribute to our goal; to have climate-neutral operations by 2035. You can also talk to us about career and development opportunities.".to_string(),
            logo:  "Preem_logo_RGB_d93952d1ac9a9e5f4fc65feb0935bdc5.svg".to_string(),
            map_image:  2,
            booth_number:  111,
            tags: Some(vec![4,2,3,9,15,16,20,19,17,22,28,40])
            },
    ];

    let expected_layouts_amount = 6;
    let expected_layouts = vec![
        LayoutDB {
            id: 1,
            image: "karusellvanster.png".to_string(),
            active: true,
            placement: 1,
        },
        LayoutDB {
            id: 6,
            image: "4.png".to_string(),
            active: true,
            placement: 0,
        },
        LayoutDB {
            id: 5,
            image: "3.png".to_string(),
            active: true,
            placement: 0,
        },
        LayoutDB {
            id: 3,
            image: "2.png".to_string(),
            active: true,
            placement: 0,
        },
        LayoutDB {
            id: 4,
            image: "1.png".to_string(),
            active: true,
            placement: 0,
        },
        LayoutDB {
            id: 2,
            image: "bergochdalhoger.png".to_string(),
            active: true,
            placement: 2,
        },
    ];

    let expected_maps_amount = 4;
    let expected_maps = vec![
        MapDB {
            id: 1,
            name: "SB".to_string(),
            image: "SB.png".to_string(),
            reference: 0,
        },
        MapDB {
            id: 2,
            name: "SU2".to_string(),
            image: "SU2.png".to_string(),
            reference: 0,
        },
        MapDB {
            id: 3,
            name: "SU1".to_string(),
            image: "SU1.png".to_string(),
            reference: 0,
        },
        MapDB {
            id: 4,
            name: "SU3".to_string(),
            image: "SU3.png".to_string(),
            reference: 0,
        },
    ];

    let expected_prepages_amount = 49;
    let expected_prepages = vec![
        PrepageDB {
            id: 1,
            name: "P1".to_string(),
            image: "prepage1.png".to_string(),
            active: true,
            mobile: true,
            side: "left".to_string(),
            page: 6,
        },
        PrepageDB {
            id: 17,
            name: "P13".to_string(),
            image: "prepage13.png".to_string(),
            active: true,
            mobile: true,
            side: "right".to_string(),
            page: 11,
        },
        PrepageDB {
            id: 28,
            name: "P27".to_string(),
            image: "prepage27.png".to_string(),
            active: true,
            mobile: true,
            side: "left".to_string(),
            page: 18,
        },
        PrepageDB {
            id: 44,
            name: "P44".to_string(),
            image: "prepage44.png".to_string(),
            active: true,
            mobile: true,
            side: "right".to_string(),
            page: 3,
        },
        PrepageDB {
            id: 49,
            name: "P48".to_string(),
            image: "prepage48.png".to_string(),
            active: true,
            mobile: false,
            side: "right".to_string(),
            page: 5,
        },
    ];

    let expected_shortcuts_amount = 3;
    let expected_shortcuts = vec![
        ShortcutDB {
            id: 1,
            name: "Search".to_string(),
            description: "".to_string(),
            link: "/search".to_string(),
            icon: "mdi-magnify".to_string(),
        },
        ShortcutDB {
            id: 2,
            name: "Browse Companies".to_string(),
            description: " ".to_string(),
            link: "/company/Aarsleff%20Ground%20Engineering%20AB".to_string(),
            icon: "mdi-account-group".to_string(),
        },
        ShortcutDB {
            id: 3,
            name: "Favorites".to_string(),
            description: "".to_string(),
            link: "/search?favorites=true".to_string(),
            icon: "mdi-star".to_string(),
        },
    ];

    let expected_tags_amount = 41;
    let expected_tags = vec![
        TagDB {
            id: 1,
            name: "A".to_string(),
            parent_tag: 0,
            up_votes: 0,
            down_votes: 0,
            crowd_sourced: false,
            icon: "a.svg".to_string(),
            division: true,
            business_area: false,
            looking_for: false,
            offering: false,
            language: false,
            fair_area: false,
        },
        TagDB {
            id: 15,
            name: "MT".to_string(),
            parent_tag: 0,
            up_votes: 0,
            down_votes: 0,
            crowd_sourced: false,
            icon: "mt.svg".to_string(),
            division: true,
            business_area: false,
            looking_for: false,
            offering: false,
            language: false,
            fair_area: false,
        },
        TagDB {
            id: 28,
            name: "Swedish".to_string(),
            parent_tag: 0,
            up_votes: 0,
            down_votes: 0,
            crowd_sourced: false,
            icon: "".to_string(),
            division: false,
            business_area: false,
            looking_for: false,
            offering: false,
            language: true,
            fair_area: false,
        },
        TagDB {
            id: 41,
            name: "Volvo 3".to_string(),
            parent_tag: 0,
            up_votes: 0,
            down_votes: 0,
            crowd_sourced: false,
            icon: "".to_string(),
            division: false,
            business_area: false,
            looking_for: false,
            offering: false,
            language: false,
            fair_area: true,
        },
    ];

    let expected_images_amount = 257;

    // Companies
    let companies = services::company::get_all(&db)
        .await
        .expect("to have been tested elsewhere");

    assert_eq!(
        companies.len(),
        expected_companies_amount,
        "Different amount of companies than expected"
    );
    // Since the DateTimes are based on insertion time they will differ between tests hence normal
    // hashing as with the others isn't possible, this instead looks at each field individually
    let company_tuple_iter = companies
        .iter()
        .map(|c| c.name.clone())
        .zip(companies.iter());
    let companies_set: HashMap<String, &CompanyDB> = HashMap::from_iter(company_tuple_iter);

    for expected_company in expected_companies.iter() {
        let option_company = companies_set.get(&expected_company.name).copied();

        assert!(
            option_company.is_some(),
            "missing an expected company: {:?}",
            expected_company
        );

        let company = option_company.unwrap();

        let actual_expected_company = &CompanyDB {
            id: company.id,
            last_updated: company.last_updated,
            summer_job_deadline: company.summer_job_deadline,
            tags: company.tags.clone(),
            ..expected_company.clone()
        };

        assert_eq!(company, actual_expected_company)
    }

    // Tags
    let tags = services::tag::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        tags.len(),
        expected_tags_amount,
        "Different amount of tags than expected"
    );

    let tag_tuple_iter = tags.iter().map(|c| c.name.clone()).zip(tags.iter());
    let tags_set: HashMap<String, &TagDB> = HashMap::from_iter(tag_tuple_iter);

    for expected_tag in expected_tags.iter() {
        let option_tag = tags_set.get(&expected_tag.name).copied();

        assert!(
            option_tag.is_some(),
            "missing an expected tag: {:?}",
            expected_tag
        );

        let tag = option_tag.unwrap();

        let actual_expected_tag = &TagDB {
            id: tag.id,
            ..expected_tag.clone()
        };

        assert_eq!(tag, actual_expected_tag)
    }
    // Prepages
    let prepages = services::prepage::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        prepages.len(),
        expected_prepages_amount,
        "Different amount of prepages than expected"
    );

    let prepage_tuple_iter = prepages.iter().map(|c| c.name.clone()).zip(prepages.iter());
    let prepages_set: HashMap<String, &PrepageDB> = HashMap::from_iter(prepage_tuple_iter);

    for expected_prepage in expected_prepages.iter() {
        let option_prepage = prepages_set.get(&expected_prepage.name).copied();

        assert!(
            option_prepage.is_some(),
            "missing an expected prepage: {:?}",
            expected_prepage
        );

        let prepage = option_prepage.unwrap();

        let actual_expected_prepage = &PrepageDB {
            id: prepage.id,
            ..expected_prepage.clone()
        };

        assert_eq!(prepage, actual_expected_prepage)
    }

    // Shortcuts
    let shortcuts = services::shortcut::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        shortcuts.len(),
        expected_shortcuts_amount,
        "Different amount of shortcuts than expected"
    );

    let shortcut_tuple_iter = shortcuts
        .iter()
        .map(|c| c.name.clone())
        .zip(shortcuts.iter());
    let shortcuts_set: HashMap<String, &ShortcutDB> = HashMap::from_iter(shortcut_tuple_iter);

    for expected_shortcut in expected_shortcuts.iter() {
        let option_shortcut = shortcuts_set.get(&expected_shortcut.name).copied();

        assert!(
            option_shortcut.is_some(),
            "missing an expected shortcut: {:?}",
            expected_shortcut
        );

        let shortcut = option_shortcut.unwrap();

        let actual_expected_shortcut = &ShortcutDB {
            id: shortcut.id,
            ..expected_shortcut.clone()
        };

        assert_eq!(shortcut, actual_expected_shortcut)
    }

    // Layouts
    let layouts = services::layout::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        layouts.len(),
        expected_layouts_amount,
        "Different amount of layouts than expected"
    );

    let layout_tuple_iter = layouts.iter().map(|c| c.image.clone()).zip(layouts.iter());
    let layouts_set: HashMap<String, &LayoutDB> = HashMap::from_iter(layout_tuple_iter);

    for expected_layout in expected_layouts.iter() {
        let option_layout = layouts_set.get(&expected_layout.image).copied();

        assert!(
            option_layout.is_some(),
            "missing an expected layout: {:?}",
            expected_layout
        );

        let layout = option_layout.unwrap();

        let actual_expected_layout = &LayoutDB {
            id: layout.id,
            ..expected_layout.clone()
        };

        assert_eq!(layout, actual_expected_layout)
    }

    // Maps
    let maps = services::map::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        maps.len(),
        expected_maps_amount,
        "Different amount of maps than expected"
    );

    let map_tuple_iter = maps.iter().map(|c| c.name.clone()).zip(maps.iter());
    let maps_set: HashMap<String, &MapDB> = HashMap::from_iter(map_tuple_iter);

    for expected_map in expected_maps.iter() {
        let option_map = maps_set.get(&expected_map.name).copied();

        assert!(
            option_map.is_some(),
            "missing an expected map: {:?}",
            expected_map
        );

        let map = option_map.unwrap();

        let actual_expected_map = &MapDB {
            id: map.id,
            ..expected_map.clone()
        };

        assert_eq!(map, actual_expected_map)
    }

    // Images
    let images = services::image::get_all(&db)
        .await
        .expect("to have been tested elsewhere");
    assert_eq!(
        images.len(),
        expected_images_amount,
        "Different amount of images than expected"
    );

    let files_in_dir = fs::read_dir(storage_path).unwrap().fold(0, |c, _| c + 1);
    assert_eq!(files_in_dir, expected_images_amount);

    Ok(())
}
