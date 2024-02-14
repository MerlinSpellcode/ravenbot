# Ravenbot

## Creating Hunting Coordinates

## Hunting

## [Arquivo de Configuração](./config.json)

- **hunts** -> Array de objetos com as coordenadas de cada hunt
- **combat** -> Objeto de Configuração que necessita de interação do usuário
    - **hp_regen_passive** -> limite de porcentagem de HP para que o personagem pare para curar passivamente
    - **mana_regen_passive** -> limite de porcentagem de Mana para que o personagem pare para curar passivamente
    - **hp_to_defense_light** -> limite de porcentagem de HP para que o script execute as skills da variável "defense_light"
    - **hp_to_defense_full** -> limite de porcentagem de HP para que o script execute as skills da variável "defense_light"
    - **global_cd** -> valor em milissegundos para setar o global cooldown do seu personagem (ex.: 1100 = 1,1 segundos)
    - **basic** -> Objeto de configuração da skill básica geradora de Aether
        - **hotkey** -> hotkey da skills (OBS: apenas letras, números e a família F (F1, F2, etc) foram mapeados)
        - **mana** -> mana gasto com a skill (WIP)
        - **name** -> nome da skill (WIP)
    - **start** -> Array de configuração de skills para iniciar cada combate
        - **hotkey** -> hotkey da skills (OBS: apenas letras, números e a família F (F1, F2, etc) foram mapeados)
        - **mana** -> mana gasto com a skill (WIP)
        - **aether** -> boolean (true ou false), se true, antes de castar o script vai garantir 50 de aether
        - **cooldown** -> tempo em segundos de cooldown da skill (WIP)
        - **name** -> nome da skill (WIP)
        - **is_area** -> boolean (true ou false), se true, o script vai garantir o duplo clique da hotkey
    - **combo** -> Array de configuração de skills para combo (normalmente skills de dano que usam aether)
        - **hotkey** -> hotkey da skills (OBS: apenas letras, números e a família F (F1, F2, etc) foram mapeados)
        - **mana** -> mana gasto com a skill (WIP)
        - **aether** -> boolean (true ou false), se true, antes de castar o script vai garantir 50 de aether
        - **cooldown** -> tempo em segundos de cooldown da skill (WIP)
        - **name** -> nome da skill (WIP)
        - **is_area** -> boolean (true ou false), se true, o script vai garantir o duplo clique da hotkey
    - **defense_light** -> Array de configuração de skills para usar quando "hp_to_defense_light" chegar no limite de hp definido pelo valor variável
        - **hotkey** -> hotkey da skills (OBS: apenas letras, números e a família F (F1, F2, etc) foram mapeados)
        - **mana** -> mana gasto com a skill (WIP)
        - **aether** -> boolean (true ou false), se true, antes de castar o script vai garantir 50 de aether
        - **cooldown** -> tempo em segundos de cooldown da skill (WIP)
        - **name** -> nome da skill (WIP)
        - **is_area** -> boolean (true ou false), se true, o script vai garantir o duplo clique da hotkey
    - **defense_full** -> Array de configuração de skills para usar quando "hp_to_defense_full" chegar no limite de hp definido pelo valor variável
        - **hotkey** -> hotkey da skills (OBS: apenas letras, números e a família F (F1, F2, etc) foram mapeados)
        - **mana** -> mana gasto com a skill (WIP)
        - **aether** -> boolean (true ou false), se true, antes de castar o script vai garantir 50 de aether
        - **cooldown** -> tempo em segundos de cooldown da skill (WIP)
        - **name** -> nome da skill (WIP)
        - **is_area** -> boolean (true ou false), se true, o script vai garantir o duplo clique da hotkey

