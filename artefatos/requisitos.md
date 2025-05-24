Documento de Especificação de Requisitos (MVP)
Sistema de Reservas de Ambientes Esportivos "PebolMAX"

Versão: 0.1
Data: 21/05/2025

Índice:

    Introdução
    1.1. Propósito
    1.2. Escopo do Produto
    1.3. Definições, Acrônimos e Abreviações

    Visão Geral do Sistema
    2.1. Perspectiva do Produto
    2.2. Funcionalidades Principais
    2.3. Atores e Perfis de Usuário

    Arquitetura de Alto Nível (Componentes Principais)

    Requisitos Funcionais
    4.1. Épico 1: Gerenciamento de Contas e Perfis de Jogador (App do Jogador)
    4.2. Épico 2: Gerenciamento de Clubes (App do Jogador)
    4.3. Épico 3: Gerenciamento de Listas de Reserva (App do Jogador)
    4.4. Épico 4: Pagamentos e Carteira Digital (App do Jogador)
    4.5. Épico 5: Visualização e Interação com o Ambiente (App do Jogador)
    4.6. Épico 6: Notificações (App do Jogador)
    4.7. Épico 7: Autenticação e Gerenciamento de Conta (App da Arena)
    4.8. Épico 8: Gerenciamento de Listas e Reservas (App da Arena)
    4.9. Épico 9: Check-in de Jogadores (App da Arena)
    4.10. Épico 10: Relatórios (App da Arena)
    4.11. Processos e Regras de Negócio Centrais do Sistema

    Modelo de Dados (Entidades Principais)

    Requisitos Não Funcionais
    6.1. Usabilidade
    6.2. Performance
    6.3. Segurança
    6.4. Confiabilidade/Disponibilidade
    6.5. Manutenibilidade
    6.6. Integrações
    6.7. Conformidade

    Fora do Escopo (MVP)

    Suposições e Dependências

1. Introdução

    1.1. Propósito
    Este documento especifica os requisitos para a primeira versão (MVP - Minimum Viable Product) do sistema de reservas de ambientes esportivos, doravante chamado "PebolMAX". O sistema visa facilitar a organização de jogos e a reserva de quadras/arenas esportivas, conectando jogadores e estabelecimentos.

    1.2. Escopo do Produto
    O PebolMAX consistirá em dois aplicativos móveis nativos (desenvolvidos em Flutter):
        1. App do Jogador: Para jogadores encontrarem e participarem de listas de reserva, gerenciarem seus perfis, clubes e pagamentos.
        2. App da Arena: Para funcionários de estabelecimentos esportivos gerenciarem a disponibilidade de suas quadras, criarem listas de reserva, acompanharem o status das reservas e gerarem relatórios.
        O sistema incluirá funcionalidades de criação de listas, gerenciamento de pagantes, concessão de reservas baseada em quórum, um sistema de precificação dinâmica, carteira digital para jogadores e ferramentas de gestão para as arenas.

    1.3. Definições, Acrônimos e Abreviações
        Arena/Estabelecimento: Local físico que oferece quadras/espaços esportivos para aluguel.
        Lista (de Reserva): Uma tentativa de um grupo de jogadores (clube) de reservar um horário específico em uma quadra. Atingir um número mínimo de pagantes é necessário para garantir a prioridade na reserva.
        Clube: Um grupo de jogadores, geralmente com um ou mais administradores.
        Organizador (da Lista): O jogador responsável por iniciar/gerenciar uma lista específica.
        Pagante: Jogador que efetuou o pagamento para participar de uma lista.
        PIB: Preço Individual Base.
        SDHQ: Score de Demanda do Horário/Quadra.
        SU: Score do Usuário.
        PIF: Preço Individual Final.
        MVP: Minimum Viable Product (Produto Mínimo Viável).
        JWT: JSON Web Token.
        OAuth: Open Authorization.
        API: Application Programming Interface.
        UI: User Interface (Interface do Usuário).
        UX: User Experience (Experiência do Usuário).

2. Visão Geral do Sistema

    2.1. Perspectiva do Produto
    O PebolMAX é um novo produto independente que se integrará a serviços externos de pagamento e autenticação (Google OAuth). Ele fornecerá uma plataforma para facilitar a interação entre jogadores que buscam horários para praticar esportes e arenas que oferecem esses espaços.

    2.2. Funcionalidades Principais
        Criação e gerenciamento de contas de usuário (Visitante, PIX, Full para jogadores; Funcionário para arenas).
        Criação e gerenciamento de clubes por jogadores.
        Criação, participação e gerenciamento de Listas de Reserva.
        Sistema de estados para Listas (Vazia, Aberta, Em Disputa, Esperando, Reservado, Cancelada, Perdido, Concluído).
        Processamento de pagamentos individuais para participação em listas.
        Carteira digital no App do Jogador para saldo, depósitos e saques.
        Precificação dinâmica baseada em demanda e score do usuário.
        Check-in de jogadores via QR Code no App da Arena.
        Geração de relatórios básicos para o App da Arena.
        Notificações para eventos importantes.

    2.3. Atores e Perfis de Usuário (Detalhado)

    Esta seção descreve os principais atores que interagem com o sistema PebolMAX e os diferentes perfis de usuário, detalhando suas permissões, regras e responsabilidades.

    2.3.1. Jogador
    Indivíduo que utiliza o "App do Jogador" para encontrar, participar e organizar atividades esportivas.

        2.3.1.1. Perfil: Visitante
            Acesso: Obtido ao abrir o app pela primeira vez sem realizar login/cadastro ou ao acessar um link de lista sem estar logado.
            Permissões:
                Visualizar listas de reserva de forma limitada (ex: local, horário, status básico como "Aberta", "Em Disputa", "Esperando", "Reservado", sem ver detalhes de participantes ou organizador).
                Navegar em telas públicas do app (ex: tela inicial, talvez uma lista de arenas parceiras).
            Restrições:
                Não pode visualizar perfis detalhados de jogadores ou clubes.
                Não pode interagir com pagamentos (entrar em lista, depositar na carteira).
                Não pode criar ou gerenciar clubes.
                Não pode se tornar organizador de lista.
                Não pode configurar perfil pessoal.
            Evolução: Pode evoluir para Conta PIX (ao tentar pagar com PIX rápido) ou Conta Full (ao realizar cadastro completo).

        2.3.1.2. Perfil: Conta PIX
            Acesso: Obtido ao evoluir de Visitante através do fluxo de "pagamento rápido com PIX", associando um CPF e uma única conta bancária (usada no primeiro pagamento PIX).
            Permissões:
                Todas as permissões de Visitante.
                Visualizar perfis detalhados de jogadores (nome, foto, posição, pé, clubes, modalidades) e clubes.
                Realizar pagamentos para entrar em listas, utilizando a conta PIX única associada.
                Optar por "Pagar Pessoalmente".
                Depositar fundos na carteira digital (mínimo R$12), utilizando a conta PIX única associada.
                Solicitar saques da carteira digital (taxa de 2%), exclusivamente para a conta PIX única associada.
                Tornar-se Organizador de uma lista (se for o primeiro pagante).
                Gerenciar sua participação em listas (compartilhar, sair com reembolso dentro das regras, visualizar QR Code de check-in).
                Receber e visualizar notificações.
                Visualizar saldo e extrato da carteira digital.
            Restrições:
                Pode ter apenas UMA conta bancária (PIX) associada, que não pode ser alterada.
                Não pode cadastrar cartões de crédito ou outras contas PIX.
                Não pode criar clubes.
                Não pode ser administrador de clubes (a menos que evolua para Full).
                Login persistente entre dispositivos pode ser limitado (reautenticação via novo pagamento com PIX da mesma conta/CPF pode ser necessária se não evoluir para Full).
            Regras:
                O CPF informado é usado para identificação única deste tipo de conta.
            Evolução: Pode evoluir para Conta Full a qualquer momento, associando um método de login robusto (Google OAuth ou Email/Senha).

        2.3.1.3. Perfil: Conta Full
            Acesso: Obtido através de cadastro completo (Google OAuth ou Email/Senha) ou evoluindo de uma Conta PIX.
            Permissões:
                Todas as permissões de Conta PIX.
                Registrar e gerenciar múltiplos métodos de pagamento (até 20 contas bancárias/PIX, cartões de crédito).
                Criar até 1 clube, tornando-se seu primeiro administrador.
                Ser administrador de até 1 clube no total.
                Participar de até 5 clubes.
                Convidar jogadores para seus clubes (como membros ou administradores).
                Gerenciar perfil completo (incluindo edição de modalidade/posição favorita, pé preferido, foto).
                Login robusto e persistente entre dispositivos (Mobile/Web).
            Restrições:
                Limites de criação/administração de clubes conforme especificado.
            Regras:
                Se um CPF já está associado a uma Conta PIX, a criação de uma nova Conta Full com o mesmo CPF só pode ocorrer via "upgrade" da Conta PIX existente.

        2.3.1.4. Papel Adicional: Organizador da Lista
            Acesso: Qualquer jogador com Conta PIX ou Full pode se tornar Organizador ao ser o primeiro a entrar e pagar em uma lista VAZIA, ou ao receber a organização de outro jogador.
            Permissões Adicionais (sobre a lista que organiza):
                Alterar horário, duração e tamanho mínimo da lista (seguindo regras de recálculo de preço e impacto nos pagantes).
                Transferir o cargo de organizador para outro integrante pago da lista.
                Para listas no estado PERDIDO:
                    Definir preferências de busca para um novo horário.
                    Selecionar um novo horário/local/quadra/duração/tamanho dentre as opções apresentadas pelo sistema.
                    Desistir da lista (reembolsando todos os pagantes).
                Visualizar quem optou por "Pagar Pessoalmente".
                Pode ter acesso a mais detalhes sobre o andamento da sua lista.
            Responsabilidades:
                Principal ponto de contato/decisão para a lista.
                Comunicar alterações importantes aos membros da lista (embora o sistema também notifique).

    2.3.2. Funcionário da Arena
    Indivíduo associado a um estabelecimento esportivo, que utiliza o "App da Arena" para gerenciar operações relacionadas a reservas.
        Acesso: Obtido através de cadastro e login (Google OAuth ou Email/Senha) no App da Arena. A associação com um estabelecimento específico é gerenciada (no MVP, manualmente no backend após cadastro do funcionário).
        Permissões:
            Criar novas listas de reserva para as quadras do(s) seu(s) local(is) associado(s), especificando quadra, horário de início, duração e tamanho mínimo.
            Copiar/compartilhar o link de listas criadas.
            Visualizar o calendário de horários de todas as quadras do(s) seu(s) local(is), incluindo o status das listas e reservas.
            Cancelar uma reserva confirmada (estado RESERVADO), informando um motivo, o que aciona o reembolso aos jogadores.
            Escanear QR Codes de jogadores para realizar o check-in de acesso à quadra.
            Visualizar relatórios de métricas (receitas, ticket médio, % ocupação, qtde agendamentos, faltas) para o(s) seu(s) local(is) associado(s), com drill-down por quadra.
        Restrições:
            Ações limitadas ao(s) estabelecimento(s)/local(is) aos quais está vinculado.
            Não interage com o sistema de carteira digital ou perfis de jogador (além do necessário para check-in).
            No MVP, não gerencia o cadastro de informações da arena (locais, quadras, preços base) diretamente pelo app (isso é feito via formulário externo).
        Regras:
            Para o MVP, assume-se um perfil de permissão único para todos os funcionários da arena. Não há diferentes níveis de acesso dentro do App da Arena.

    2.3.3. Clube (Entidade Gerenciada)
    Representa um grupo de jogadores. Não é um ator que "loga", mas uma entidade central para a organização de listas.
        Criação: Por um jogador com Conta Full.
        Administração:
            Até 3 administradores por clube.
            Um jogador pode ser administrador de no máximo 1 clube.
        Participação:
            Um jogador pode ser membro de até 5 clubes.
        Associação:
            Toda lista de reserva deve ser associada a um clube.
        Informações: Nome, logo (opcional), lista de membros, lista de administradores.

3. Arquitetura de Alto Nível (Componentes Principais)
    O sistema será composto por:
        App do Jogador (Flutter)
        App da Arena (Flutter)
        Backend API (Servidor): Lógica de negócios, gerenciamento de dados.
        Banco de Dados: Persistência dos dados do sistema.
        Módulo de Integração com API de Pagamentos: Interação com provedor de pagamentos para PIX e Cartão.
        Webhook Listener: Recebimento de callbacks da API de Pagamentos.
        Blob Storage: Armazenamento de imagens (logos, fotos de perfil).

    (Diagrama de arquitetura simplificado pode ser inserido aqui posteriormente)

4. Requisitos Funcionais

    4.1. Épico 1: Gerenciamento de Contas e Perfis de Jogador (App do Jogador)
    * US-JOG-ACC-001: Como um novo jogador, eu quero poder me registrar usando meu email/senha ou Google OAuth, informando minha modalidade e posição favoritas e pé preferido, para ter uma Conta Full.
    * US-JOG-ACC-002: Como um jogador, eu quero poder fazer login na minha Conta Full usando email/senha ou Google OAuth.
    * US-JOG-ACC-003: Como um visitante, ao tentar pagar, eu quero poder optar por um cadastro rápido via pagamento PIX (Conta PIX), informando meu CPF e banco.
    * US-JOG-ACC-004: Como um jogador com Conta PIX, eu quero poder evoluir minha conta para uma Conta Full associando Google OAuth ou definindo email/senha.
    * US-JOG-ACC-005: Como um jogador, eu quero poder visualizar e editar meu perfil (nome, foto, modalidade/posição preferida, pé preferido).
    * US-JOG-ACC-006: Como um jogador com Conta PIX/Full, eu quero poder visualizar perfis de outros jogadores (nome, foto, posição, pé, clubes, modalidades).
    * US-JOG-ACC-007: Como um jogador, eu quero que o sistema crie automaticamente um perfil para mim em uma nova modalidade quando eu participar de uma reserva dessa modalidade pela primeira vez.
    * US-JOG-ACC-008: Como um jogador, eu quero poder fazer logout da minha conta.
    * US-JOG-ACC-009: Como um visitante, eu quero poder acessar o app sem conta para visualizar listas (limitadamente).

    4.2. Épico 2: Gerenciamento de Clubes (App do Jogador)
    * US-JOG-CLB-001: Como um jogador com Conta Full, eu quero poder criar um clube, tornando-me seu primeiro administrador.
    * US-JOG-CLB-002: Como um administrador de clube, eu quero poder convidar outros jogadores para serem membros ou administradores do meu clube (limite de 3 admins/clube, jogador pode ser admin de 1 clube).
    * US-JOG-CLB-003: Como um jogador, eu quero poder aceitar/recusar convites para clubes e participar de até 5 clubes.
    * US-JOG-CLB-004: Como um jogador, eu quero poder visualizar informações de clubes (nome, logo, membros, administradores).
    * US-JOG-CLB-005: Como um administrador de clube, eu quero poder editar informações do clube (nome, logo) e remover membros/administradores.

    4.3. Épico 3: Gerenciamento de Listas de Reserva (App do Jogador)
    * US-JOG-LST-001: Como um jogador, eu quero poder acessar uma lista em estado VAZIA através de um link compartilhado.
    * US-JOG-LST-002: Como um jogador, ao acessar uma lista VAZIA, eu quero poder me tornar o Organizador, associar um dos meus clubes e optar por ser o primeiro pagante (com 20% de desconto).
    * US-JOG-LST-003: Como um participante de lista, eu quero poder compartilhar o link da lista com outros jogadores.
    * US-JOG-LST-004: Como um jogador, eu quero poder entrar em uma lista pagando o valor individual (antecipado).
    * US-JOG-LST-005: Como um jogador, eu quero poder indicar que vou "Pagar Pessoalmente" em uma lista, entendendo que não conto para o quórum até pagar e não tenho desconto.
    * US-JOG-LST-006: Como um jogador que optou por "Pagar Pessoalmente", eu quero poder mudar e "Pagar Antecipado" a qualquer momento antes da lista ser ESPERANDO ou RESERVADO.
    * US-JOG-LST-007: Como um jogador, eu quero poder visualizar os detalhes de uma lista (local, quadra, horário, status, número de pagantes/mínimo, integrantes, preço individual).
    * US-JOG-LST-008: Como um organizador de lista, eu quero poder alterar o horário, duração e tamanho mínimo da lista, ciente das regras de recálculo de preço e potencial remoção/reembolso de integrantes.
    * US-JOG-LST-009: Como um organizador de lista, eu quero poder transferir o cargo de organizador para outro integrante pago da lista.
    * US-JOG-LST-010: Como um organizador de lista no estado PERDIDO, eu quero poder definir preferências (dia, duração, hora, tamanho) para que o sistema me mostre novas opções de horário disponíveis (priorizando o local original), e então escolher uma nova opção, acionando o recálculo de preços e status dos pagantes.
    * US-JOG-LST-011: Como um organizador de lista no estado PERDIDO, eu quero poder desistir da lista, reembolsando todos os pagantes para suas carteiras.
    * US-JOG-LST-012: Como um jogador pagante de uma lista (antes de <24h da reserva), eu quero poder solicitar reembolso, saindo da lista e recebendo o valor na minha carteira in-app.
    * US-JOG-LST-013: Como um jogador de uma lista RESERVADO, eu quero ter acesso a um QR Code para check-in.

    4.4. Épico 4: Pagamentos e Carteira Digital (App do Jogador)
    * US-JOG-PAY-001: Como um jogador com Conta Full, eu quero poder cadastrar e gerenciar meus métodos de pagamento (PIX via Open Finance, Cartão de Crédito - até 20 contas).
    * US-JOG-PAY-002: Como um jogador com Conta PIX, eu quero que minha única conta PIX associada seja usada para pagamentos e saques.
    * US-JOG-PAY-003: Como um jogador, eu quero poder realizar pagamentos para entrar em listas usando meus métodos cadastrados ou PIX rápido.
    * US-JOG-PAY-004: Como um jogador, eu quero ter uma carteira digital no app para visualizar meu saldo e extrato de transações.
    * US-JOG-PAY-005: Como um jogador, eu quero poder depositar fundos na minha carteira via PIX (Open Finance) ou Cartão de Crédito (mínimo R$12).
    * US-JOG-PAY-006: Como um jogador, eu quero poder solicitar o saque de fundos da minha carteira para uma conta bancária associada (taxa de 2%).

    4.5. Épico 5: Visualização e Interação com o Ambiente (App do Jogador)
    * US-JOG-ENV-001: Como um jogador, ao visualizar uma lista, eu quero ver informações do local (logo, nome, endereço) e um botão para abrir o local no Google Maps.
    * US-JOG-ENV-002: Como um jogador, ao visualizar uma lista, eu quero ver um botão para chamar um motorista (ex: Uber), direcionando para um local de estacionamento da arena.

    4.6. Épico 6: Notificações (App do Jogador)
    * US-JOG-NOT-001: Como um jogador, eu quero receber notificações sobre eventos importantes (nova lista no meu clube, mudança de horário/valor da minha lista, reserva concedida, lista perdida, reembolso processado, chargeback, cancelamento pela arena, etc.).

    4.7. Épico 7: Autenticação e Gerenciamento de Conta (App da Arena)
    * US-ARN-ACC-001: Como um funcionário da arena, eu quero poder me registrar/logar no App da Arena usando email/senha ou Google OAuth.

    4.8. Épico 8: Gerenciamento de Listas e Reservas (App da Arena)
    * US-ARN-LST-001: Como um funcionário da arena, eu quero poder criar uma nova lista informando quadra, horário de início, duração da partida e tamanho mínimo da lista.
    * US-ARN-LST-002: Como um funcionário da arena, após criar uma lista, eu quero poder copiar/compartilhar o link da lista (para envio externo, ex: WhatsApp).
    * US-ARN-LST-003: Como um funcionário da arena, eu quero poder visualizar todas as listas e reservas de um local específico para um determinado dia, incluindo seus status e horários.
    * US-ARN-LST-004: Como um funcionário da arena, eu quero poder cancelar uma reserva confirmada (estado RESERVADO), informando um motivo, o que resultará no reembolso integral para os jogadores.

    4.9. Épico 9: Check-in de Jogadores (App da Arena)
    * US-ARN-CHK-001: Como um funcionário da arena, eu quero poder escanear o QR Code apresentado pelo jogador (no App do Jogador) para realizar o check-in e liberar o acesso à quadra.

    4.10. Épico 10: Relatórios (App da Arena)
    * US-ARN-REP-001: Como um funcionário da arena, eu quero poder visualizar relatórios com métricas (receitas, ticket médio, % ocupação, qtde agendamentos, faltas) por intervalo (dia, semana, mês, ano) para um local.
    * US-ARN-REP-002: Como um funcionário da arena, ao visualizar um gráfico de métrica, eu quero poder clicar nele para ver a mesma métrica detalhada por quadra.

    4.11. Processos e Regras de Negócio Centrais do Sistema
    * SYS-RULE-LST-STATES: O sistema deve gerenciar os estados das listas (VAZIA, ABERTA, EM DISPUTA, ESPERANDO, RESERVADO, CANCELADA, PERDIDO, CONCLUIDO) conforme as regras detalhadas (Ver Apêndice A - Fluxo de Estados da Lista - a ser criado).
    * SYS-RULE-PRICING: O sistema deve implementar a lógica de precificação dinâmica individual, considerando Preço Base da Reserva (arena), Tamanho Mínimo da Lista, Margem de Segurança (2 jogadores no MVP), Score de Demanda do Horário/Quadra (SDHQ) e Score do Usuário (SU -2% a +5% no MVP). (Ver Apêndice B - Lógica de Precificação - a ser criado).
    * SYS-RULE-PRIORITY: Uma lista que atinge o estado ESPERANDO ganha prioridade, movendo listas concorrentes para PERDIDO e bloqueando o horário.
    * SYS-RULE-REFUND: Reembolsos para a carteira in-app são permitidos se solicitados >24h antes da reserva ou em casos de cancelamento/alteração pela arena ou organizador (conforme regras).
    * SYS-RULE-CHARGEBACK: O primeiro chargeback de um usuário resultará no bloqueio da conta para análise manual.
    * SYS-RULE-ARENA-INFO: Informações da arena (locais, quadras, preços base, links) são cadastradas manualmente via formulário (processo externo ao app no MVP).
    * SYS-RULE-CHECKIN-EXPIRY: QR Codes de check-in expiram após o término do horário da reserva.
    * SYS-RULE-FAILURES: Faltas são registradas se <50% dos pagantes mínimos fizerem check-in até 10 min após o início da reserva.

    4.12. Requisitos de Autenticação OAuth (Padronização Backend)

    * SYS-AUTH-OAUTH-001: O backend API deve prover endpoints para processar a autenticação via Google OAuth.
    * SYS-AUTH-OAUTH-002: Os clientes Flutter (Mobile e Web) devem enviar um token de acesso (access token) ou token de ID (ID token) obtido do provedor Google para o backend.
    * SYS-AUTH-OAUTH-003: O backend deve validar o token recebido junto ao provedor Google para confirmar sua autenticidade e obter as informações do usuário (email, nome, ID do Google).
    * SYS-AUTH-OAUTH-004: Se o usuário for novo (baseado no ID do Google ou email), o backend deve criar uma nova Conta Full, solicitando informações adicionais (modalidade favorita, posição, pé preferido) se não vierem no primeiro fluxo OAuth.
    * SYS-AUTH-OAUTH-005: Se o usuário já existir, o backend deve autenticá-lo e retornar um JWT da aplicação para sessões subsequentes.
    * SYS-AUTH-OAUTH-006: O fluxo deve ser consistente para clientes Flutter Mobile e Web, garantindo que o backend receba a mesma natureza de token (preferencialmente ID Token por ser mais seguro para este fim) para validação.

    5. Modelo de Dados (Entidades Principais)
        __EM PROGRESSO___

    6. Requisitos Não Funcionais

    6.1. Usabilidade
    * NFR-USAB-001: A interface do usuário deve ser intuitiva e fácil de usar para todos os perfis de usuário.
    * NFR-USAB-002: O sistema deve fornecer feedback claro ao usuário sobre suas ações e o status do sistema (ex: mensagens de sucesso, erro, carregamento).
    * NFR-USAB-003: A navegação principal nos aplicativos deve ser consistente e previsível.

    6.2. Performance
    * NFR-PERF-001: A consulta de preço individual e disponibilidade de horários deve ser respondida em até 3 segundos sob carga normal.
    * NFR-PERF-002: O processamento de pagamentos (PIX) deve refletir o status na carteira em tempo próximo ao real, dependendo da confirmação da API de pagamento.
    * NFR-PERF-003: As telas de listagem (listas, reservas, relatórios) devem carregar em até 5 segundos com até 100 itens.

    6.3. Segurança
    * NFR-SEC-001: Todos os dados sensíveis do usuário (CPF, dados de pagamento parciais, senhas) devem ser armazenados e transmitidos de forma segura (criptografia).
    * NFR-SEC-002: O sistema deve usar autenticação baseada em JWT para proteger as APIs.
    * NFR-SEC-003: O sistema deve informar claramente ao usuário o motivo da coleta de dados pessoais, especialmente o CPF.
    * NFR-SEC-004: O sistema deve ter mecanismos para prevenir e detectar atividades fraudulentas básicas (ex: múltiplos chargebacks).

    6.4. Confiabilidade/Disponibilidade
    * NFR-REL-001: O sistema deve ter uma disponibilidade de 99.5% para as funcionalidades críticas (login, criação/visualização de listas, pagamento).
    * NFR-REL-002: Links públicos de listas devem permanecer acessíveis.

    6.5. Manutenibilidade
    * NFR-MAIN-001: O código deve seguir padrões de desenvolvimento e ser bem documentado para facilitar futuras manutenções e evoluções.

    6.6. Integrações
    * NFR-INT-001: Integração com API de Pagamentos (PIX Open Finance, Cartões).
    * NFR-INT-002: Integração com Google OAuth para autenticação.
    * NFR-INT-003: Integração com Google Maps (abertura de link externo).
    * NFR-INT-004: Integração com apps de transporte (ex: Uber, abertura de link externo).

    6.7. Conformidade
    * NFR-COMP-001: O sistema deve estar em conformidade com a Lei Geral de Proteção de Dados (LGPD) referente à coleta, armazenamento e processamento de dados pessoais de usuários brasileiros.


Apêndice A: Fluxo de Estados da Lista de Reserva
    Okay, vamos detalhar cada estado possível de uma lista, explicando as regras de negócio associadas. Isso é fundamental para o entendimento do ciclo de vida de uma reserva.

    **Estados Possíveis da Lista e Suas Regras de Negócio:**

    1.  **`VAZIA`**
        *   **O que é?** É o estado inicial de uma lista recém-criada pelo funcionário da arena e compartilhada com um potencial organizador (geralmente via link). A lista ainda não tem um "dono" (organizador) nem um clube associado, e, consequentemente, nenhum pagante.
        *   **Principais Regras e Comportamentos:**
            *   **Criação:** Só pode ser criada pelo App da Arena.
            *   **Visibilidade:** Acessível via link público.
            *   **Ações do Jogador:** Um jogador (com conta PIX ou Full) que acessa o link pode:
                *   Optar por se tornar o **Organizador**.
                *   Ao se tornar organizador, deve associar um dos seus **Clubes** à lista.
                *   Deve ser o **primeiro a pagar** (ou comprometer-se a "Pagar Pessoalmente" e depois pagar) para ativar a lista. Se for o primeiro pagante antecipado, recebe 20% de desconto.
            *   **Conflito na Criação:** Se, no momento da criação pelo funcionário, o horário e quadra já estiverem em conflito com uma lista `ABERTA` ou `EM DISPUTA` existente, a lista pode ser criada já no estado `EM DISPUTA` assim que o primeiro organizador/pagante surgir (em vez de passar por `VAZIA` e depois `ABERTA` para então ir para `EM DISPUTA`). *Isso simplifica: `VAZIA` é sem dono; assim que tem dono e pagante, vai para `ABERTA` (sem conflito) ou `EM DISPUTA` (com conflito).*
        *   **Possíveis Transições de Saída:**
            *   Para `ABERTA`: Após um jogador se tornar organizador, associar um clube e efetuar o primeiro pagamento, E o horário/quadra da lista não conflitar com nenhuma outra lista `ABERTA` ou `EM DISPUTA`.
            *   Para `EM DISPUTA`: Após um jogador se tornar organizador, associar um clube e efetuar o primeiro pagamento, E o horário/quadra da lista conflitar com outra(s) lista(s) `ABERTA`(s) ou `EM DISPUTA`(s).

    2.  **`ABERTA`**
        *   **O que é?** A lista possui um organizador, um clube associado e pelo menos um jogador pagante (o organizador). Crucialmente, neste estado, a lista **não está concorrendo diretamente** pelo horário com nenhuma outra lista ativa.
        *   **Principais Regras e Comportamentos:**
            *   **Aceitando Pagantes:** Outros jogadores podem entrar na lista efetuando o pagamento.
            *   **Sem Conflito Imediato:** O horário/quadra está livre de concorrência direta no momento.
            *   **Gerenciamento pelo Organizador:** O organizador pode alterar dados da lista (horário, tamanho, etc.). Se uma alteração de horário criar um conflito, a lista mudará para `EM DISPUTA`.
            *   **Saída de Jogadores:** Jogadores pagantes podem solicitar reembolso e sair da lista (se >24h da reserva), o valor retorna para a carteira in-app. Se o último pagante sair, a lista pode voltar a um estado similar ao `VAZIA` internamente ou ser `CANCELADA`.
        *   **Possíveis Transições de Saída:**
            *   Para `EM DISPUTA`: Se uma nova lista é criada para o mesmo horário/quadra, ou se esta lista ou outra é alterada para um horário conflitante.
            *   Para `ESPERANDO`: Se a lista atinge o número mínimo de pagantes configurado, e ainda faltam mais de 24 horas para o horário da reserva. (Normalmente passaria por `EM DISPUTA` se o horário for concorrido, mas pode ir direto se for a única interessada no horário).
            *   Para `CANCELADA`: Se todos os pagantes saírem da lista.

    3.  **`EM DISPUTA`**
        *   **O que é?** A lista está ativa (tem organizador, clube, e pode ou não ter pagantes além do organizador), mas seu horário/quadra desejado **está sendo disputado** por pelo menos uma outra lista também `ABERTA` ou `EM DISPUTA`. É uma corrida para ver qual lista atinge o quórum primeiro.
        *   **Principais Regras e Comportamentos:**
            *   **Concorrência Ativa:** Múltiplas listas competem pelo mesmo slot de tempo, total ou parcialmente.
            *   **Objetivo:** Atingir o número mínimo de pagantes antes das concorrentes.
            *   **Demais Comportamentos:** Similar à `ABERTA` (aceita pagantes, organizador gerencia, etc.), mas com a pressão da disputa.
        *   **Possíveis Transições de Saída:**
            *   Para `ESPERANDO`: Se a lista atinge o número mínimo de pagantes, faltam >24h para a reserva, e ela "vence" a disputa. As listas concorrentes vão para `PERDIDO`.
            *   Para `PERDIDO`: Se uma lista concorrente pelo mesmo horário/quadra atinge o estado `ESPERANDO` primeiro.
            *   Para `ABERTA`: Se todas as listas concorrentes são canceladas ou mudam de horário, eliminando o conflito direto.
            *   Para `CANCELADA`: Se todos os pagantes saírem.

    4.  **`ESPERANDO`**
        *   **O que é?** A lista atingiu o número mínimo de pagantes e faltam **mais de 24 horas** para o horário da reserva. Esta lista **ganhou prioridade** sobre as concorrentes para aquele horário/quadra.
        *   **Principais Regras e Comportamentos:**
            *   **Prioridade Garantida:** Nenhuma outra lista pode mais "roubar" este horário. Todas as outras listas que estavam `EM DISPUTA` por este horário passam para o estado `PERDIDO`.
            *   **Bloqueio de Horário:** O sistema não permite a criação de novas listas que sobreponham este horário/quadra enquanto esta lista estiver em `ESPERANDO` ou `RESERVADO`.
            *   **Reembolsos Ainda Possíveis:** Jogadores ainda podem solicitar reembolso e sair da lista (valor para a carteira in-app).
            *   **Risco de Perder Prioridade:** Se, devido a saídas, a lista cair abaixo do número mínimo de pagantes, ela perde o estado `ESPERANDO` e a prioridade. O horário volta a ficar disponível para disputa.
            *   **Alterações pelo Organizador:** O organizador ainda pode alterar dados da lista. Se tais alterações (ex: mudança de horário, aumento drástico do mínimo) fizerem a lista perder o quórum ou a viabilidade, ela pode regredir de estado.
        *   **Possíveis Transições de Saída:**
            *   Para `RESERVADO`: Quando o tempo restante para o horário da reserva se torna inferior a 24 horas, e a lista continua completa (com o mínimo de pagantes).
            *   Para `ABERTA`: Se perder pagantes e cair abaixo do mínimo, e o horário (agora liberado) não tiver outras listas ativas concorrendo.
            *   Para `EM DISPUTA`: Se perder pagantes e cair abaixo do mínimo, e o horário (agora liberado) tiver outras listas ativas concorrendo.
            *   Para `CANCELADA`: (Raro neste ponto, mas possível) Se todos os pagantes decidirem sair.

    5.  **`RESERVADO`**
        *   **O que é?** A lista está completa (atingiu o mínimo de pagantes) e faltam **menos de 24 horas** para o horário da reserva. A reserva está efetivamente garantida e confirmada.
        *   **Principais Regras e Comportamentos:**
            *   **Reserva Confirmada:** O horário é da lista.
            *   **Sem Reembolso In-App:** Jogadores não podem mais solicitar reembolso automático via app. Desistências a partir daqui não geram crédito na carteira (a vaga está "perdida" para o jogador, a menos que a arena tenha políticas externas).
            *   **QR Code de Check-in:** Todos os integrantes pagantes da lista têm acesso ao QR Code para o check-in na arena.
            *   **Bloqueio de Horário Mantido:** O horário/quadra continua indisponível para outras listas.
            *   **Cancelamento pela Arena:** Em casos excepcionais (problema na quadra, etc.), o funcionário da arena pode cancelar uma reserva neste estado, o que deve gerar reembolso integral para todos os pagantes na carteira in-app e notificação.
        *   **Possíveis Transições de Saída:**
            *   Para `CONCLUIDO`: Após o horário da reserva ter passado.
            *   Para `CANCELADA_PELA_ARENA` (ou um tipo específico de `CANCELADA`): Se o funcionário da arena cancelar a reserva.

    6.  **`PERDIDO`**
        *   **O que é?** A lista estava `EM DISPUTA` por um horário, mas outra lista concorrente conseguiu atingir o estado `ESPERANDO` primeiro. A lista atual perdeu a chance de reservar aquele horário específico.
        *   **Principais Regras e Comportamentos:**
            *   **Sem Horário Associado:** A lista mantém seus integrantes (organizador, clube, pagantes), mas não tem mais um horário/quadra vinculado.
            *   **Ação do Organizador Necessária:** O organizador deve tomar uma de duas ações:
                1.  **Encontrar Novo Horário:** O organizador define novas preferências (dia, duração, hora, tamanho) e o sistema busca e apresenta opções de horários/quadras disponíveis. Ao selecionar uma nova opção, os preços são recalculados e os pagantes podem precisar complementar, ser reembolsados parcialmente, ou ser removidos com reembolso total se o valor pago não cobrir o novo preço.
                2.  **Desistir da Lista:** O organizador cancela a tentativa, e todos os pagantes recebem reembolso integral na carteira in-app.
        *   **Possíveis Transições de Saída:**
            *   Para `ABERTA`: Se o organizador escolhe um novo horário/configuração que não tem conflito e a lista (com os pagantes remanescentes após recálculo) ainda precisa de mais gente.
            *   Para `EM DISPUTA`: Se o organizador escolhe um novo horário/configuração que já tem conflito.
            *   Para `ESPERANDO`: (Menos comum, mas possível) Se o organizador escolhe um novo horário/configuração e, com os pagantes atuais e o novo preço, a lista já atinge o mínimo e as condições para `ESPERANDO`.
            *   Para `CANCELADA`: Se o organizador optar por desistir da lista.

    7.  **`CANCELADA`**
        *   **O que é?** A lista foi descontinuada. Isso pode ocorrer por várias razões: todos os pagantes saíram, o organizador desistiu de uma lista `PERDIDO`, ou uma alteração de regras pelo organizador resultou no esvaziamento da lista.
        *   **Principais Regras e Comportamentos:**
            *   **Inativa:** A lista não participa mais de disputas e não ocupa nenhum horário.
            *   **Reembolsos:** Quaisquer valores pagos por ex-integrantes devem ter sido processados e devolvidos para suas carteiras in-app conforme as regras de cada transição que levou ao cancelamento.
            *   **Histórico:** Pode ser mantida no sistema para fins de histórico, mas sem ações possíveis sobre ela.
        *   **Possíveis Transições de Saída:** Nenhuma (é um estado final para aquela instância de lista).

    8.  **`CANCELADA_PELA_ARENA`** (Subtipo de `CANCELADA` ou estado distinto para clareza)
        *   **O que é?** Uma lista que estava no estado `RESERVADO` foi cancelada por uma ação do funcionário da arena.
        *   **Principais Regras e Comportamentos:**
            *   **Motivo:** O funcionário da arena deve fornecer um motivo.
            *   **Reembolso Obrigatório:** Reembolso integral do valor pago é automaticamente creditado na carteira in-app de todos os pagantes.
            *   **Notificação:** Todos os pagantes são notificados.
            *   **Horário Liberado:** O horário/quadra volta a ficar disponível.
        *   **Possíveis Transições de Saída:** Nenhuma (é um estado final).

    9.  **`CONCLUIDO`**
        *   **O que é?** O horário da reserva já passou. A partida teoricamente aconteceu (ou deveria ter acontecido).
        *   **Principais Regras e Comportamentos:**
            *   **Pós-Reserva:** A lista serve para registro histórico.
            *   **Dados para Relatórios:** Informações de check-in (ou a ausência deles, resultando em "falta") são importantes para os relatórios da arena.
            *   **Sem Ações:** Nenhuma ação adicional pode ser tomada na lista.
        *   **Possíveis Transições de Saída:** Nenhuma (é um estado final).


Apêndice B: Lógica de Precificação Dinâmica Individual (PIF)
    Este apêndice descreve os componentes e o cálculo do Preço Individual Final (PIF) para um jogador em uma lista.
    Fórmula Geral:  PIF (Jogador J, Lista L) = (PIB_Lista_L * Fator_Demanda_Horario_L * Fator_Score_Jogador_J) - Desconto_Organizador (se aplicável)
    Componentes:
        Preço Base da Reserva da Arena (PBRA):
            Definido pela administração da arena.
            Varia por: [Quadra] + [Dia da Semana] + [Faixa Horária Geral].
            Exemplo: Quadra de Futebol Society, Sábado à tarde, 1 hora = R$180.
        Tamanho Mínimo da Lista (TML):
            Definido pelo criador da lista (Funcionário da Arena ou Organizador em lista PERDIDO).
            Exemplo: 18 jogadores.
        Margem de Segurança (MS):
            Valor fixo para o MVP: 2 jogadores.
            Usado para diluir o preço base e absorver flutuações.
        Divisor Efetivo (DE):
            DE = TML - MS
            Exemplo: 18 - 2 = 16.
        Preço Individual Base da Lista (PIB_Lista_L):
            PIB_Lista_L = PBRA / DE
            Exemplo: R$180 / 16 = R$11,25. Este é o custo "bruto" por vaga efetiva antes dos fatores dinâmicos.
        Score de Demanda do Horário/Quadra (SDHQ):
            Um valor numérico que representa o interesse atual para um [Horário Específico + Quadra Específica].
            Eventos que Influenciam POSITIVAMENTE o SDHQ (com pesos e decaimento temporal):
                Criação de novas listas para o mesmo [horário+quadra].
                Número de listas ABERTAS ou EM DISPUTA concorrendo pelo mesmo [horário+quadra].
                Chamadas ao endpoint de consulta de preço para o [horário+quadra].
                Compartilhamento de links de listas para o [horário+quadra].
                Conversões de "Pagar Pessoalmente" para "Pagar Antecipado" em listas no [horário+quadra].
                Pesquisas de disponibilidade no App da Arena que incluem aquele [horário+quadra].
            Algoritmo do SDHQ: (A ser detalhado, mas envolve soma ponderada de eventos recentes).
        Fator de Demanda (Fator_Demanda_Horario_L):
            Um multiplicador derivado do SDHQ.
            Exemplo de faixas (a serem calibradas):
                SDHQ < Threshold1: Fator = 1.0 (preço base)
                Threshold1 <= SDHQ < Threshold2: Fator = 1.1 (10% de acréscimo)
                SDHQ >= Threshold2: Fator = 1.2 (20% de acréscimo)
            Se Fator > 1.0, o app exibe "Preço acima do normal devido à alta demanda".
        Score do Usuário (SU_Jogador_J):
            Um valor numérico que representa a "reputação" ou histórico do jogador J.
            Eventos que Influenciam (exemplos, pesos a definir):
                Positivo: Frequência como organizador de listas bem-sucedidas, pagamentos pontuais, baixo índice de desistência, ausência de chargebacks.
                Negativo: Alta frequência de desistência de listas, chargebacks, histórico de faltas em reservas.
            Algoritmo do SU: (A ser detalhado, mas envolve histórico de ações).
        Fator de Score do Usuário (Fator_Score_Jogador_J):
            Um multiplicador derivado do SU do Jogador J.
            Exemplo para MVP:
                SU "Bom": Fator = 0.98 (-2% de desconto).
                SU "Neutro": Fator = 1.0.
                SU "Ruim": Fator = 1.05 (+5% de acréscimo).
        Desconto de Organizador:
            Se o Jogador J for o primeiro pagante da Lista L (tornando-se Organizador), um desconto de 20% é aplicado sobre o valor (PIB_Lista_L * Fator_Demanda_Horario_L * Fator_Score_Jogador_J).
    Processo de Cálculo ao Jogador Consultar Preço/Entrar na Lista:
        Identificar Lista L e Jogador J.
        Obter PBRA para o [horário+quadra] da Lista L.
        Obter TML da Lista L. Calcular DE e PIB_Lista_L.
        Calcular/Obter o SDHQ atual para o [horário+quadra] da Lista L e derivar o Fator_Demanda_Horario_L.
        Obter o SU_Jogador_J e derivar o Fator_Score_Jogador_J.
        Calcular o preço antes do desconto: Preco_Intermediario = PIB_Lista_L * Fator_Demanda_Horario_L * Fator_Score_Jogador_J.
        Se Jogador J for elegível para desconto de organizador (primeiro pagante da lista VAZIA):       PIF = Preco_Intermediario * 0.80.
        Senão:      PIF = Preco_Intermediario.
        Arredondar o PIF para 2 casas decimais.
    Este é o PIF que será exibido e cobrado do jogador.