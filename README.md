
# Controle Script

Se você está aqui é por que precisa configurar seu controle genérico no seu linux para jogar na xcloud, então vou lhe ajudar.

### você ira precisa dessas coisas no seu Linux.

- xboxdrv
- rust (a biblioteca cargo também)
- evtest (já vem padrão na sua maquína, se não, instala também)

faz um sudo para os acima:
``` 
sudo apt install ----

``` 

### e agora? 

Quando fizer tudo isso, o meu repositorio irá funcionar para você, então... Na sua maquina você executa o "evtest" para conseguir mapear seu controle (ta com duvida? vê esse video ----) 

obs: futuramente irei deixar automatico.

Depois de mapear e salvar o nome do "eventX" do seu controle você vai no codigo main do repositorio (src/main.rs) e modificar o paramentro da função na main. 

### Mapeação e o Paramentro

- \"/dev/input/eventX\" - Muda X por o numero do event do seu controle (evtest).
- Na linha onde tem "--evdev-keymap" altere cada "N--" pelo o nome do botão que você anotou com o evtest.
 Ex: "N-- = x" ficaria --> "BTN_TRIGGER=x"

- Na linha onde tem "--evdev-absmap" será seu analogico, então altere cada "N--" pelo o nome do seu analogico. 
obs: "dpad_x" são suas setas

- na linha "--axismap" não altere nada.

Quando fizer tudo isso, você terá seu scprit completo, então abra seu terminal e execute os comandos seguintes dentro do repositorio. 

- cargo build --release

ele ira buildar e atualizar o bin que tem dentro da pasta "target/release", então agora é so executar sempre esse bin para jogar. 
