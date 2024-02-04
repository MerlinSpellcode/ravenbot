use winapi::shared::windef::HWND;
use crate::utils::inputs::{press_w, press_a, press_s, press_d, press_skill, press_tab};
use crate::utils::env::{Skill, BasicS};
use crate::checks::{get_aether, hp_need_restore, mana_need_restore, is_hp_full, is_mana_full, get_target, get_coord};

use tokio::time::{Duration, Instant};
use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::sync::{Arc, Mutex};
use std::{thread};

struct CooldownManagerAsync {
    last_called: Arc<Mutex<HashMap<String, Instant>>>,
}

impl CooldownManagerAsync {
    fn new() -> CooldownManagerAsync {
        CooldownManagerAsync {
            last_called: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn can_use_skill(&self, function_id: &str, _cooldown: Duration) -> bool {
        let now = Instant::now();
        let last_called = self.last_called.lock().unwrap();
    
        if let Some(&last_time) = last_called.get(function_id) {
            println!("Habilidade '{}' em cooldown. Agora: {:?}, Último: {:?}, Deve Esperar Até: {:?}", function_id, now, last_time, last_time + _cooldown);
            now >= last_time
        } else {
            true
        }
    }

    // Método assíncrono para atualizar o cooldown
    async fn update_cooldown(&self, function_id: String, cooldown: Duration) {
        let now = Instant::now();
        let next_allowed_time = now + cooldown;
        println!("Atualizando cooldown para '{}'. Agora: {:?}, Próximo: {:?}", function_id, now, next_allowed_time);
        let mut last_called = self.last_called.lock().unwrap();
        last_called.insert(function_id, next_allowed_time);
    }
}

// Definindo a variável estática com Lazy para permitir inicialização em tempo de execução
static CDR_MANAGER: Lazy<Arc<CooldownManagerAsync>> = Lazy::new(|| {
    Arc::new(CooldownManagerAsync::new())
});



pub fn path_walker(hwnd: HWND, destination: [i32; 3], hp_regen_passive: &str, mana_regen_passive: &str, hp_to_defense: &str, combat_basic: &[BasicS], combat_damage: &[Skill], combat_defense: &[Skill]) {
    let mut attempts = 0;
    let max_attempts = 1000; // Limite para tentativas de movimento para evitar loop infinito

    while attempts < max_attempts {
        let current = get_coord();
        
        // Verifica se o personagem chegou ao destino
        if current == destination {
            break;
        }

        // Tenta mover-se na direção correta
        if current[0] < destination[0] {
            press_d(hwnd); // Aumenta X
        } else if current[0] > destination[0] {
            press_a(hwnd); // Diminui X
        }

        if current[1] < destination[1] {
            press_s(hwnd); // Aumenta Y
        } else if current[1] > destination[1] {
            press_w(hwnd); // Diminui Y
        }

        // Aguarda um curto período antes de tentar novamente
        std::thread::sleep(std::time::Duration::from_millis(1));

        // Verifica se o personagem se moveu
        let new_coord = get_coord();
        if new_coord == current {
            // O personagem não se moveu, então tenta outro movimento
            combat_instance(hwnd, hp_regen_passive, mana_regen_passive, hp_to_defense, combat_basic, combat_damage, combat_defense);
            attempts += 1;
        } else {
            // Reseta as tentativas se houver movimento
            attempts = 0;
        }
    }
}


fn check_target(hwnd: HWND) -> bool {
    let first_try_get = get_target();
    if first_try_get {
        true
    } else {
        press_tab(hwnd);
        let sec_try_get = get_target();
        if sec_try_get {
            true
        } else {
            press_tab(hwnd);
            let third_try_get = get_target();
            if third_try_get {
                true
            } else {
                false
            }
        }
    }
}

fn spam_skills(
    manager: Arc<CooldownManagerAsync>,
    hwnd: HWND,
    skills_list: &[Skill]
) {
    for skill in skills_list {
        let skill_cd_name = format!("{}_skill", &skill.hotkey);
        let cooldown_duration = Duration::from_secs(skill.cooldown);
        if manager.can_use_skill(&skill_cd_name, cooldown_duration) {
        }
        if check_target(hwnd) {
            use_skill(hwnd, &skill.hotkey);
            thread::sleep(Duration::from_millis(1));
            let manager_clone = manager.clone();
            let function_id = skill_cd_name.to_string();
            tokio::spawn(async move {
                manager_clone.update_cooldown(function_id, cooldown_duration).await;
            });
            if get_aether() < 50.0 {
                break;
            }
        }
        else {
            return;
        }
    }
    return;
}

fn use_skill(hwnd: HWND, hotkey: &str) {
    if check_target(hwnd) {
        thread::sleep(Duration::from_millis(1));
        press_skill(hwnd, hotkey);
        return;
    } else {
        return;
    }
}



pub fn combat_instance(hwnd: HWND, hp_regen_passive: &str, mana_regen_passive: &str, hp_to_defense: &str, combat_basic: &[BasicS], combat_damage: &[Skill], combat_defense: &[Skill]) {
    while check_target(hwnd) {
        if hp_need_restore(hp_to_defense) {
            while get_aether() < 50.0 {
                std::thread::sleep(std::time::Duration::from_secs(1));
                if check_target(hwnd) {
                    use_skill(hwnd, &combat_basic[0].hotkey)
                } else {
                    break;
                }
            }
            spam_skills(CDR_MANAGER.clone(), hwnd, combat_defense);
        } else {
            while get_aether() < 50.0 {
                std::thread::sleep(std::time::Duration::from_secs(1));
                if check_target(hwnd) {
                    use_skill(hwnd, &combat_basic[0].hotkey)
                } else {
                    break;
                }
            }
            spam_skills(CDR_MANAGER.clone(), hwnd, combat_damage);
        }
        thread::sleep(Duration::from_millis(1));
    }

    if hp_need_restore(hp_regen_passive) {
        println!("HP needs restore");
        while is_hp_full() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if check_target(hwnd) {
                break;
            }
        }
    }
    if mana_need_restore(mana_regen_passive) {
        println!("Mana needs restore");
        while is_mana_full() {
            std::thread::sleep(std::time::Duration::from_secs(1));
            if check_target(hwnd) {
                break;
            }
        }
    }
    println!("# Saindo do Combat Stance #");
    return;
}