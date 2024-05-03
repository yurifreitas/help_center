use tauri_plugin_shell::ShellExt;
use tauri::command;

#[command]
async fn check_disk_usage(app: tauri::AppHandle) -> Result<String, String> {
    let shell = app.shell();
    println!("Obtendo referência ao shell");
  
    // Executando o comando de forma assíncrona e tratando os possíveis erros
    let output = shell.command("ls")
        .arg("/sdcard")
        .output()
        .await;
    
    // let output = shell.command("du")
    //     .arg("-sh")
    //     .arg("/*")
    //     .output()
    //     .await;
    println!("recbendo output referência ao shell");
    match output {
        Ok(output) => {
            if output.status.success() {
                println!("resultado");
                let result = String::from_utf8(output.stdout)
                    .unwrap_or_else(|_| "Erro ao decodificar a saída.".to_string());
                println!("Result: {:?}", result);
                Ok(result)
            } else {
                let error_message = format!("Exit with code: {}", output.status.code().unwrap_or_default());
                println!("{}", error_message);
                Err(error_message)
            }
        },
        Err(error) => {
            let error_message = format!("Erro ao executar comando: {}", error);
            println!("{}", error_message);
            Err(error_message)
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .invoke_handler(tauri::generate_handler![check_disk_usage])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
