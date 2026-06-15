use std::f32::consts::E;
use std::fmt::Error;
use std::process::{Command, Stdio};
use std::str;
use std::path::Path;

    fn file_valid( x:&str  )-> bool{
        if x.is_empty(){
            return false;
        }else{
            let y = x.strip_prefix("0x").unwrap_or(x);
            !y.is_empty() && y.char().all(|c| c.is_ascii_hexdigit());
        }
    return true;
    }

    fn dir_valid(x:&str) -> bool{
        return Path::new(x).exists();
    }

fn read_log () -> Result<(), Box<dyn std::error::Error>>{
    
    // O grep retorna status de sucesso (0) se encontrar algo. 
    // Retorna falha (1) se a string não for encontrada.
    // Retorna erro (>1) se o arquivo não existir ou outro problema de SO ocorrer.
    
    let log_file = "switch_mac_event.log";
    let search_term = "ERROR";
    let folder = "/home/luiz/Desktop/switch logs/";
    let f = dir_valid(folder);
    

    if !dir_valid(folder){
        return Err("Caminho não encontrado".into());
    }else {
        println!("Iniciando Busca...");

        let process_output = Command::new("grep")
            .current_dir(folder)
            .arg(search_term)
            .arg(log_file)
            .stdout(Stdio::piped()) //captura saida padrão
            .stderr(Stdio::piped()) //captura possiveis erros
            .output()?; // executa e bloqueia até o processo filho terminar

            if process_output.status.success(){
                let result_string = str::from_utf8(&process_output.stdout)?;
                println!("{}",result_string);
            }else {
                let error_string = str::from_utf8(&process_output.stderr)?;

                if error_string.is_empty(){
                    println!("Nenhuma ocorrencia em {}" , search_term);
                }else {
                    eprintln!("Falha ao invocar o processo {}", error_string());
                }
            }
    }
    Ok(())
}


fn main() {
  let log = read_log();
  println!("{}",log);
}
