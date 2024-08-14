// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Instant;

use opengm_rts::*;

// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command(async)]
fn run_tests(sample_dir: &str) -> String {
    let samples = read_dir(sample_dir);
    if samples.is_err() {
        return format!("{}", samples.err().unwrap());
    }

    let samples = samples.unwrap();

    let n = samples.len();
    let bits = samples[0].bits();
    let w = waterline(ALPHA, samples.len()) as i32;

    let testers = get_testers(&ALL_TESTS_FUNCS, bits);
    let start_time = Instant::now();
    let (presult, qresult) = randomness_test(&samples, &testers);

    let mut pfailed_tests = Vec::new();
    let mut qfailed_tests = Vec::new();
    
    let mut s = String::from("<table>");
    s.push_str("<tr>");
    s.push_str("<th></th>");
    s.push_str("<th>p_value通过率</th>");
    s.push_str("<th>q_value分布均匀性</th>");
    s.push_str("</tr>");
    for tester in testers{
        s.push_str("<tr>");
        s.push_str(format!("<td>{}</td>",tester).as_str());

        let passed  = count_pvalue_pass(&presult, tester, ALPHA);
        if passed < w{
            s.push_str(format!("<td style=\"color:#ff0000\">{}/{}</td>",passed, n).as_str());
            pfailed_tests.push(tester);
        }else{
            s.push_str(format!("<td>{}/{}</td>",passed, n).as_str());
        }

        if *qresult.get(&tester).unwrap() < SAMPLE_DISTRIBUTION_ALPHA_T{
            s.push_str(format!("<td style=\"color:#ff0000\">{:.4}</td>",qresult.get(&tester).unwrap()).as_str());
            qfailed_tests.push(tester);
        }else{
            s.push_str(format!("<td>{:.4}</td>",qresult.get(&tester).unwrap()).as_str());
        }

        s.push_str("</tr>");
    }
    s.push_str("</table>");

    // if pfailed_tests.len() > 0{
    //     s.push_str("p_value tests failed: ");
    //     for t in &pfailed_tests{
    //         s.push_str(format!("{} ", t).as_str());
    //     }
    //     s.push_str("</br>");
    // }
    // if qfailed_tests.len() > 0{
    //     s.push_str("q_value tests failed: ");
    //     for t in &qfailed_tests{
    //         s.push_str(format!("{} ", t).as_str());
    //     }
    //     s.push_str("</br>");

    // }

    if pfailed_tests.len() == 0 && pfailed_tests.len() == 0{
        s.push_str("Randomness test PASS.</br>")
    }else{
        s.push_str("Randomness test FAIL.</br>")
    }
    s.push_str(format!("Randomness test used time: {} seconds</br>",  (Instant::now() - start_time).as_secs()).as_str());
    s.push_str("</br>By the OpenGM Group</br>");
    s
}


fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_tests])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
