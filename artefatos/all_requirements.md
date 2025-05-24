# Requisitos do App de Reservas

Este documento descreve os requisitos funcionais e não funcionais do App de Reservas de Quadras.

## Título e Stories

### Splash_Screen

*   **Descrição:** O app deve exibir um icone e uma tela de splash screen ao abrir o app.
*   **Persona:** app
*   **Como:** "Como app, devo exibir um icone e uma tela de splash screen ao abrir o app"

---

## Login, autenticação e sua conta

### Conta_Gmail

*   **Descrição:** O usuário pode criar e entrar na sua conta usando o Gmail.
*   **Persona:** usuário
*   **Como:** "Como usuário, eu posso criar e entrar na minha conta usando o Gmail"

---

### Conta_Visitante

*   **Descrição:** O visitante pode usar o app sem inserir nenhuma informação pessoal, ao logar na opção especial "Entrar como convidado", o usuário recebe uma conta visitante, que não precisa inserir informações como cpf e modalidade favorita.
*   **Persona:** visitante
*   **Como:** "Como visitante, eu posso usar o app sem inserir nenhuma informação pessoal"

---

### Upgrade_para_conta_pix

*   **Descrição:** O visitante converte sua conta para Pix após o primeiro pagamento ser processado.
*   **Persona:** visitante
*   **Como:** "Como visitante, eu converto minha conta para Pix ao concluir o primeiro pagamento"

---

### Gerenciar_Perfil

*   **Descrição:** O usuário pode editar seu perfil (nome, foto, telefone).
*   **Persona:** usuário
*   **Como:** "Como usuário, eu posso editar meu perfil (nome, foto, telefone)"

---

### Vincular_Gmail_a_conta_Pix

*   **Descrição:** O usuário pode conectar seu Gmail a uma conta já existente.
*   **Persona:** usuário
*   **Como:** "Como usuário, eu posso conectar meu Gmail a uma conta já existente"

---

### Middleware_verificacao_token

*   **Descrição:** O sistema valida o token de cada requisição e cria/retorna contas visitantes quando necessário.
*   **Persona:** sistema
*   **Como:** "Como sistema, eu valido o token de cada requisição e crio/retorno contas visitantes quando necessário"

---

### Criar_Conta_Email_Senha

*   **Descrição:** O usuário pode criar uma conta utilizando email e senha .
*   **Persona:** usuário
*   **Como:** "Como usuário, eu posso criar uma conta usando email e senha"

---

## Reservas e listas

### Ver_minhas_listas

*   **Descrição:** O usuário quer ver suas listas por ordem cronológica.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero ver minhas listas por ordem cronológica"

---

### Status_da_lista

*   **Descrição:** O Jogador quer ver seu status na lista em tempo real para saber se precisa agir.
*   **Persona:** Jogador
*   **Como:** "Como Jogador, quero ver meu status na lista em tempo real para saber se preciso agir."

---

### Informacoes_da_reserva

*   **Descrição:** O usuário quer ver o local, endereço, horario e quadra da sua reserva.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero ver o local, endereço, horario e quadra da minha reserva"

---

### Estado_da_lista

*   **Descrição:** O usuário quer ver o estado atual da lista, em qual estagio a lista está para conseguir a reserva.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero ver o estado atual da lista, em qual estagio a lista está para conseguir a reserva"

---

### Organizador_da_lista

*   **Descrição:** O usuário quer ver o organizador da lista para saber quem é o responsável pela lista, o seu time e a modalidade do jogo disputado.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero ver o organizador da lista para saber quem é o responsável pela lista, o seu time e a modalidade do jogo disputado"

---

### Integrantes_da_lista

*   **Descrição:** O usuário quer ver os integrantes da lista para saber quem está na lista.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero ver os integrantes da lista para saber quem está na lista"

---

## Ações externas

### Acessar_o_Mapa

*   **Descrição:** O usuário quer acessar o mapa para ver onde está a quadra da lista.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero acessar o mapa para ver onde está a quadra da lista"

---

### Chamar_Motorista

*   **Descrição:** O usuário quer chamar o motorista para a partida.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero chamar o motorista para a partida"

---

### Compartilhar_lista

*   **Descrição:** O usuário quer compartilhar a lista com outras pessoas.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero compartilhar a lista com outras pessoas"

---

## Pagamentos

### Escolher_metodo_de_pagamento

*   **Descrição:** O usuário quer escolher o método de pagamento para a lista.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero escolher o método de pagamento para a lista"

---

### Adicionar_novo_pix

*   **Descrição:** O usuário pode adicionar uma conta pix como método de pagamento para a sua conta, sujeito a restrições por nível: Conta full pode adicionar; Conta pix pode adicionar apenas UMA conta pix; Conta visitante não pode adicionar.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero adicionar um novo pix para a lista"

---

### Remover_pix

*   **Descrição:** O usuário pode remover uma conta pix como método de pagamento da sua conta, sujeito a restrições por nível: Conta full pode remover; Conta pix NÃO pode remover sua única conta pix; Conta visitante não pode remover.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero remover um pix da lista"

---

### Adicionar_novo_cartao

*   **Descrição:** O usuário pode adicionar um cartão como método de pagamento para a sua conta, sujeito a restrições por nível: Conta full pode adicionar; Conta pix NÃO pode adicionar; Conta visitante não pode adicionar.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero adicionar um novo cartão para a lista"

---

### Remover_cartao

*   **Descrição:** O usuário pode remover um cartão como método de pagamento da sua conta, sujeito a restrições por nível: Conta full pode remover; Conta pix NÃO pode remover; Conta visitante não pode remover.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero remover um cartão da lista"

---

### Pagar_Vaga_na_lista

*   **Descrição:** O usuário pode pagar a vaga de uma lista.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero poder pagar a vaga de uma lista"

---

### Pagar_para_outra_pessoa

*   **Descrição:** O usuário pode pagar para outra pessoa.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero poder pagar para outra pessoa"

---

### Pedir_reembolso

*   **Descrição:** O usuário quer pedir um reembolso e sair da lista.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero pedir um reembolso e sair da lista"

---

### Multa_por_bloqueio_de_reserva

*   **Descrição:** O sistema quer cobrar uma multa caso um time complete a lista e troque de horario impossibilitando outro time de ocupar aquele horário.
*   **Persona:** sistema
*   **Como:** "Como sistema, quero cobrar uma multa caso um time complete a lista e troque de horario impossibilitando outro time de ocupar aquele horário"

---

### Saque_Carteira

*   **Descrição:** O usuário pode sacar fundos de sua carteira, sujeito a regras baseadas no nível da conta (full, pix, visitante). Contas full podem sacar o valor integral para qualquer conta. Contas pix podem sacar para contas pix usadas em depósitos ou pagamentos, até o valor máximo depositado daquela conta. Contas visitante não podem sacar.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero poder sacar fundos da minha carteira, com regras definidas pelo nível da minha conta"

---

### Deposito_Carteira

*   **Descrição:** O usuário pode depositar fundos em sua carteira, sujeito a regras baseadas no nível da conta (full, pix, visitante). Contas full podem usar cartão e pix. Contas pix podem usar apenas pix. Contas visitante não podem depositar.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero poder depositar fundos na minha carteira, com métodos definidos pelo nível da minha conta"

---

### Listar_Transacoes

*   **Descrição:** O usuário logado deve poder visualizar uma lista de todas as suas transações (saques, depósitos, pagamentos, reembolsos, multas).
*   **Persona:** usuário
*   **Como:** "Como usuário logado, quero ver todas as minhas transações"

---

### Registrar_Saldo_Carteira

*   **Descrição:** O sistema deve registrar o valor do saldo da carteira do usuário após cada transação (saque, depósito, pagamento, reembolso, multa).
*   **Persona:** sistema
*   **Como:** "Como sistema, devo registrar o saldo da carteira após cada transação do usuário"

---

## Alterar lista

### Alterar_horario_da_lista

*   **Descrição:** O organizador quer alterar o horário da lista.
*   **Persona:** organizador
*   **Como:** "Como organizador, quero alterar o horário da lista"

---

### Alterar_organizador_da_lista

*   **Descrição:** O organizador quer repassar a lista para outro usuario.
*   **Persona:** organizador
*   **Como:** "Como organizador, quero repassar a lista para outro usuario"

---

### Alterar_horario_da_lista_quando_perdeu_disputa

*   **Descrição:** O organizador quer alterar o horário da lista quando perdemos a disputa.
*   **Persona:** organizador
*   **Como:** "Como organizador, quero alterar o horário da lista quando perdemos a disputa"

---

## Checkin

### Exibir_qr_code_do_check_in

*   **Descrição:** O usuário quer exibir o qr code do checkin para o estabelecimento.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero exibir o qr code do checkin para o estabelecimento"

---

## Meu perfil

### Mostrar_perfil_do_usuario

*   **Descrição:** O usuário quer ver o perfil de outro usuario para saber mais sobre ele.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero ver o perfil de outro usuario para saber mais sobre ele"

---

### Mostrar_perfil_da_arena

*   **Descrição:** O usuário quer ver o perfil da arena para saber mais sobre ela.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero ver o perfil da arena para saber mais sobre ela"

---

### Portar_os_mensalistas_atuais

*   **Descrição:** O usuário quer portar os mensalistas atuais para o novo sistema.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero portar os mensalistas atuais para o novo sistema"

---

### Lidar_com_alteracoes_de_horarios_da_lista

*   **Descrição:** O sistema precisa alterar a carteira dos usuarios de acordo com o novo horario da lista.
*   **Persona:** sistema
*   **Como:** "Como sistema, preciso alterar a carteira dos usuarios de acordo com o novo horario da lista"

---

## Clubes

### Criar_Clube

*   **Descrição:** O usuário com conta 'full' pode criar um clube se tiver pelo menos 80 pontos de reputação e não possuir outro clube, um jogador pode ser admin em apenas um clube. A imagem do clube é enviada em binário para armazenamento em blob storage. Contas 'pix' e 'visitante' não podem criar clubes.
*   **Persona:** usuário
*   **Como:** "Como usuário com conta full, reputação >= 80 e sem clube, quero criar um clube enviando sua imagem"

---

### Gerenciar_Clube

*   **Descrição:** O administrador de um clube pode gerenciar os detalhes do clube, incluindo adicionar, editar e remover links para o site do clube, grupo do WhatsApp e perfil do Instagram. Somente o administrador tem permissão para realizar estas ações.
*   **Persona:** organizador
*   **Como:** "Como administrador de clube, quero gerenciar os detalhes do meu clube e seus links externos"

---

### Deletar_Clube

*   **Descrição:** O administrador de um clube pode deletar o clube permanentemente. Somente o administrador tem permissão para realizar esta ação.
*   **Persona:** organizador
*   **Como:** "Como administrador de clube, quero poder deletar meu clube"

---

### Exibir_Clube

*   **Descrição:** O usuário deve poder visualizar os detalhes de um clube específico, como nome, descrição, links externos, imagem e membros.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero ver as informações de um clube específico."

---

### Gerenciar_Membros_Clube

*   **Descrição:** O administrador de um clube pode gerenciar os membros, incluindo adicionar, remover, banir e alterar o status de administrador de outros membros.
*   **Persona:** organizador
*   **Como:** "Como administrador de clube, quero gerenciar os membros do meu clube."

---

### Solicitar_Entrar_Sair_Clube

*   **Descrição:** O usuário pode solicitar a entrada em um clube ou pedir para sair de um clube do qual é membro.
*   **Persona:** usuário
*   **Como:** "Como usuário, quero solicitar para entrar em um clube ou pedir para sair de um clube."

---

## Personas

*   **Usuario:** "Qualquer usuario do aplicativo"
*   **Visitante:** "Usuario que não possui uma conta no aplicativo"
*   **Organizador:** "Usuario que organiza uma listas"
*   **Jogador:** "Usuario que participa das listas"
*   **App:** "Aplicativo"
*   **Sistema:** "Qualquer sistema que interage com o aplicativo"

---

## Detalhes Adicionais

### Lidar_com_alteracoes_de_horarios_da_lista

*   O novo horario escolhido pode ser mais caro ou mais barato que o horario original da lista.
*   Se for mais barato, o sistema deve calcular a diferença e devolver o valor para a carteira do usuario.
*   Se for mais caro, o sistema deve calcular a diferença e cobrar o valor do usuario, nesse caso todos os integrantes da lista são removidos e podem entrar novamente apenas pagando o excedente.
*   Se o novo horario for igual ao horario original, o sistema não deve fazer nada.

---

## Requisitos Não Funcionais

### Limite_Tamanho_Upload

*   **Descrição:** O sistema deve limitar o tamanho dos arquivos de imagem uploaded (como fotos de perfil e logos de clube) a um máximo de 100MB.
*   **Persona:** sistema
*   **Como:** "Como sistema, devo garantir que uploads de imagem não excedam 100MB"

### Processamento_Imagem_Upload

*   **Descrição:** O sistema deve processar as imagens uploaded (redimensionar, comprimir e remover fundo se necessário) antes de armazená-las no blob storage.
*   **Persona:** sistema
*   **Como:** "Como sistema, devo processar as imagens uploaded antes de armazená-las"

### Filtragem_Nomes

*   **Descrição:** O sistema deve manter uma lista de palavras proibidas e utilizá-la para filtrar nomes de times e jogadores durante a criação ou edição, rejeitando nomes que contenham palavras da lista.
*   **Persona:** sistema
*   **Como:** "Como sistema, devo filtrar nomes de times e jogadores usando uma lista de palavras proibidas"

### Rate_Limit

*   **Descrição:** O sistema deve implementar rate limiting nos endpoints da API para proteger contra abuso, garantir a estabilidade do serviço e manter a disponibilidade para todos os usuários. Diferentes limites podem ser aplicados a diferentes endpoints ou níveis de usuário.
*   **Persona:** sistema
*   **Como:** "Como sistema, devo implementar rate limiting nos endpoints da API para proteger o serviço."

---

## Notificações

### Notificacoes_Importantes

*   **Descrição:** O sistema deve enviar notificações para os usuários sobre ações importantes que os impactam. Exemplos de ações que devem gerar notificações incluem:
    *   Deletar um time: notificar os integrantes do time deletado.
    *   Criar uma lista em nome de um time: notificar os integrantes do time.
    *   Um usuário entrar ou sair de uma lista da qual você participa.
    *   Alterações no status ou estado de uma lista.
    *   Alterações no horário ou organizador de uma lista.
    *   Confirmação de pagamentos, reembolsos ou aplicação de multas relacionadas ao usuário.
    *   Ser adicionado ou removido de um time.
    *   Criação ou exclusão de um clube.
    *   Alterações importantes nos detalhes de um clube (para membros ou seguidores).
*   **Persona:** sistema
*   **Como:** "Como sistema, devo notificar os usuários sobre ações importantes que os impactam"

---
