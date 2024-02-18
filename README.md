# Ravenbot

## Creating Hunting Coordinates

## Hunting

## [Arquivos de Configuração]()

### [Combat](./config/combat.json) -> Objeto de Configuração que necessita de interação do usuário

- **hp_regen_passive** -> limite de porcentagem de HP para que o personagem pare para curar passivamente
- **mana_regen_passive** -> limite de porcentagem de Mana para que o personagem pare para curar passivamente
- **hp_to_defense_light** -> limite de porcentagem de HP para que o script execute as skills da variável "defense_light"
- **hp_to_defense_full** -> limite de porcentagem de HP para que o script execute as skills da variável "defense_light"
- **global_cd** -> valor em milissegundos para setar o global cooldown do seu personagem (ex.: 1100 = 1,1 segundos)

### [Foods](./config/foods.json)

- **status** -> objeto com atributos para uso da food
    - **hotkey** -> hotkey para a food
    - **timer** -> tempo em minutos para executar as 3 hotkeys de food
    - **name** -> name da food 
- **hp_mana_regen** -> hotkey para segunda food
- **attack_power** -> hotkey para terceira food
    
### [Hunts](./config/hunts.json) -> Array de objetos com as coordenadas de cada hunt

- **name** -> Nome da rota
- **route** -> array de coordenadas da rota
- **stairs** -> boolean (true ou false), se true, vai garantir 2 segundos de sleep antes de comecar a rotaçao de skills

### [Walks](./config/walks.json) -> Array de objetos com as coordenadas de cada walk

- **name** -> Nome da rota
- **route** -> array de coordenadas da rota

### [Skills](./config/skills.json)

- **basic** -> Objeto de configuração da skill básica geradora de Aether
    - **hotkey** -> hotkey da skills (OBS: apenas letras, números e a família F (F1, F2, etc) foram mapeados)
    - **mana** -> mana gasto com a skill (WIP)
    - **name** -> nome da skill
- **start** -> Array de configuração de skills para iniciar cada combate
- **combo** -> Array de configuração de skills para combo (normalmente skills de dano que usam aether)
- **defense_light** -> Array de configuração de skills para usar quando "hp_to_defense_light" chegar no limite de hp definido pelo valor variável
- **defense_full** -> Array de configuração de skills para usar quando "hp_to_defense_full" chegar no limite de hp definido pelo valor variável
    - **hotkey** -> hotkey da skills (OBS: apenas letras, números e a família F (F1, F2, etc) foram mapeados)
    - **mana** -> mana gasto com a skill (WIP)
    - **aether** -> boolean (true ou false), se true, antes de castar o script vai garantir 50 de aether
    - **cooldown** -> tempo em segundos de cooldown da skill (WIP)
    - **name** -> nome da skill
    - **is_area** -> boolean (true ou false), se true, o script vai garantir o duplo clique da hotkey
    - **prereq** -> objeto da skill a ser castada como prerequisito para ela
        - **hotkey** -> hotkey da skills (OBS: apenas letras, números e a família F (F1, F2, etc) foram mapeados)
        - **mana** -> mana gasto com a skill (WIP)
        - **aether** -> boolean (true ou false), se true, antes de castar o script vai garantir 50 de aether
        - **cooldown** -> tempo em segundos de cooldown da skill (WIP)
        - **name** -> nome da skill
        - **is_area** -> boolean (true ou false), se true, o script vai garantir o duplo clique da hotkey
        - **has_global** -> boolean (true ou false), se false, script vai ignorar o delay do global cd entre o cast dela e a proxima skill
    - **has_global** -> boolean (true ou false), se false, script vai ignorar o delay do global cd entre o cast dela e a proxima skill
