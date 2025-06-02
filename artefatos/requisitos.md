Documento de Especificação de Requisitos (MVP)
Sistema de Reservas de Ambientes Esportivos "PebolMAX"

Versão: 0.2
Data: 21/05/2025

Índice:

    1. Introdução
    1.1. Propósito
    1.2. Escopo do Produto
    1.3. Definições, Acrônimos e Abreviações

    2. Visão Geral do Sistema
    2.1. Perspectiva do Produto
    2.2. Funcionalidades Principais
    2.3. Atores e Perfis de Usuário

    3. Arquitetura de Alto Nível (Componentes Principais)

    4. Requisitos Funcionais
    4.1. Gerenciamento de Contas e Perfis de Jogador
    4.2. Gerenciamento de Clubes
    4.3. Gerenciamento de Listas de Reserva
    4.4. Pagamentos e Carteira Digital
    4.5. Visualização e Interação com o Ambiente
    4.6. Notificações
    4.7. Autenticação e Gerenciamento de Conta (Arena)
    4.8. Gerenciamento de Listas e Reservas (Arena)
    4.9. Check-in de Jogadores
    4.10. Relatórios
    4.11. Processos e Regras de Negócio Centrais

    5. Modelo de Dados (Entidades Principais)

    6. Requisitos Não Funcionais
    6.1. Usabilidade
    6.2. Performance
    6.3. Segurança
    6.4. Confiabilidade/Disponibilidade
    6.5. Manutenibilidade
    6.6. Integrações
    6.7. Conformidade

    7. Fora do Escopo (MVP)

    8. Suposições e Dependências

1.  Introdução

    1.1. Propósito
    Este documento especifica os requisitos para a primeira versão (MVP - Minimum Viable Product) do sistema de reservas de ambientes esportivos, doravante chamado "PebolMAX". O sistema visa facilitar a organização de jogos e a reserva de quadras/arenas esportivas, conectando jogadores e estabelecimentos.

    1.2. Escopo do Produto
    O PebolMAX consistirá em dois aplicativos móveis nativos (desenvolvidos em Flutter): 1. App do Jogador: Para jogadores encontrarem e participarem de listas de reserva, gerenciarem seus perfis, clubes e pagamentos. 2. App da Arena: Para funcionários de estabelecimentos esportivos gerenciarem a disponibilidade de suas quadras, criarem listas de reserva, acompanharem o status das reservas e gerarem relatórios.
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

2.  Visão Geral do Sistema

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

3.  Arquitetura de Alto Nível (Componentes Principais)
    O sistema será composto por:
    App do Jogador (Flutter)
    App da Arena (Flutter)
    Backend API (Servidor): Lógica de negócios, gerenciamento de dados.
    Banco de Dados: Persistência dos dados do sistema.
    Módulo de Integração com API de Pagamentos: Interação com provedor de pagamentos para PIX e Cartão.
    Webhook Listener: Recebimento de callbacks da API de Pagamentos.
    Blob Storage: Armazenamento de imagens (logos, fotos de perfil).

4.  Requisitos Funcionais

    4.1. Gerenciamento de Contas e Perfis de Jogador (App do Jogador)

    RF-001: Registro de Usuário
    O sistema deve permitir o registro de novos jogadores através de:
    Email/senha com confirmação de email
    Google OAuth
    Durante o registro, o sistema deve coletar:
    Nome completo
    Modalidade esportiva favorita
    Posição preferida
    Pé preferido (destro/canhoto/ambidestro)
    CPF (para verificação de unicidade)
    O sistema deve criar automaticamente uma Conta Full para usuários que se registrem por estes métodos.

    RF-002: Autenticação de Usuário
    O sistema deve permitir login através de:
    Email/senha
    Google OAuth
    O sistema deve manter sessões ativas através de JWT tokens.
    O sistema deve permitir logout em qualquer momento.

    RF-003: Cadastro Rápido via PIX
    O sistema deve permitir cadastro rápido quando um visitante tentar efetuar pagamento.
    O cadastro rápido deve coletar apenas:
    CPF
    Dados da conta bancária PIX utilizada no pagamento
    O sistema deve criar uma Conta PIX com funcionalidades limitadas.

    RF-004: Evolução de Conta PIX para Full
    O sistema deve permitir que usuários com Conta PIX evoluam para Conta Full.
    A evolução deve manter todos os dados e histórico da Conta PIX.
    O sistema deve associar um método de autenticação robusto (Google OAuth ou email/senha).

    RF-005: Gerenciamento de Perfil
    O sistema deve permitir visualização e edição de perfil próprio.
    Campos editáveis: nome, foto, modalidade/posição favorita, pé preferido.
    O sistema deve validar uploads de imagem (formato, tamanho).

    RF-006: Visualização de Perfis de Outros Usuários
    O sistema deve permitir visualização de perfis de outros jogadores para usuários com Conta PIX ou Full.
    Informações visíveis: nome, foto, posição, pé preferido, clubes participantes, modalidades.

    RF-007: Criação Automática de Perfil por Modalidade
    O sistema deve criar automaticamente um perfil em nova modalidade quando o jogador participar de reserva dessa modalidade pela primeira vez.

    RF-008: Acesso de Visitante
    O sistema deve permitir acesso limitado sem cadastro.
    Visitantes podem visualizar listas de forma limitada (sem detalhes de participantes).

    4.2. Gerenciamento de Clubes

    RF-009: Criação de Clube
    O sistema deve permitir que jogadores com Conta Full criem até 1 clube.
    O criador torna-se automaticamente o primeiro administrador.
    Dados obrigatórios: nome do clube.
    Dados opcionais: logo do clube.

    RF-010: Administração de Clube
    O sistema deve permitir até 3 administradores por clube.
    Um jogador pode ser administrador de no máximo 1 clube.
    Administradores podem:
    Convidar novos membros
    Promover membros a administradores
    Remover membros
    Editar informações do clube (nome, logo)

    RF-011: Participação em Clubes
    O sistema deve permitir que jogadores participem de até 5 clubes.
    O sistema deve gerenciar convites para clubes.
    Jogadores podem aceitar/recusar convites.

    RF-012: Visualização de Informações de Clube
    O sistema deve exibir informações detalhadas dos clubes:
    Nome e logo
    Lista de membros
    Lista de administradores
    Histórico de atividades (opcional para MVP)

    4.3. Gerenciamento de Listas de Reserva

    RF-013: Acesso a Listas via Link
    O sistema deve permitir acesso a listas através de links públicos.
    Links devem ser válidos independente do estado de autenticação do usuário.

    RF-014: Organização de Lista Vazia
    O sistema deve permitir que jogadores com Conta PIX ou Full se tornem organizadores de listas vazias.
    O organizador deve associar um dos seus clubes à lista.
    O primeiro pagante recebe 20% de desconto.

    RF-015: Compartilhamento de Lista
    O sistema deve gerar links únicos para cada lista.
    Participantes devem poder compartilhar links da lista.

    RF-016: Entrada em Lista com Pagamento
    O sistema deve permitir entrada em lista através de pagamento antecipado.
    O sistema deve calcular preço individual dinamicamente (PIF).
    O sistema deve processar pagamentos via métodos cadastrados.

    RF-017: Opção "Pagar Pessoalmente"
    O sistema deve permitir indicação de "Pagar Pessoalmente".
    Jogadores que optarem por esta modalidade:
    Não contam para o quórum até efetuar pagamento
    Não recebem desconto de pagamento antecipado
    Podem converter para pagamento antecipado a qualquer momento

    RF-018: Visualização de Detalhes da Lista
    O sistema deve exibir informações completas da lista:
    Local e quadra
    Horário e duração
    Status atual
    Número de pagantes vs mínimo necessário
    Lista de participantes
    Preço individual atual

    RF-019: Alteração de Lista pelo Organizador
    O sistema deve permitir que organizadores alterem:
    Horário
    Duração
    Tamanho mínimo
    O sistema deve recalcular preços após alterações.
    O sistema deve notificar participantes sobre mudanças.

    RF-020: Transferência de Organização
    O sistema deve permitir transferência do cargo de organizador.
    Apenas integrantes pagos da lista podem receber a organização.

    RF-021: Gerenciamento de Lista Perdida
    Para listas no estado PERDIDO, o organizador deve poder:
    Definir novas preferências de horário
    Selecionar nova opção entre as apresentadas pelo sistema
    Desistir da lista (com reembolso para todos)

    RF-022: Solicitação de Reembolso
    O sistema deve permitir reembolso para jogadores pagantes.
    Condições para reembolso:
    Mais de 24h antes da reserva
    Lista cancelada pela arena
    Alterações significativas pelo organizador
    Reembolsos devem ser creditados na carteira in-app.

    RF-023: QR Code para Check-in
    O sistema deve gerar QR codes únicos para check-in.
    QR codes devem estar disponíveis apenas para listas no estado RESERVADO.
    QR codes devem expirar após o término do horário da reserva.

    4.4. Pagamentos e Carteira Digital

    RF-024: Gerenciamento de Métodos de Pagamento
    O sistema deve permitir cadastro de métodos de pagamento para Contas Full:
    Até 20 contas PIX
    Cartões de crédito
    Contas PIX devem manter apenas o método original cadastrado.

    RF-025: Processamento de Pagamentos
    O sistema deve processar pagamentos através dos métodos cadastrados.
    O sistema deve integrar com APIs de pagamento (PIX Open Finance, Cartão).
    O sistema deve confirmar pagamentos via webhook.

    RF-026: Carteira Digital
    O sistema deve manter carteira digital para cada jogador.
    A carteira deve exibir:
    Saldo atual
    Extrato de transações
    Histórico de depósitos e saques

    RF-027: Depósitos na Carteira
    O sistema deve permitir depósitos via PIX ou cartão de crédito.
    Valor mínimo para depósito: R$ 12,00.
    Depósitos devem ser confirmados via webhook do provedor de pagamento.

    RF-028: Saques da Carteira
    O sistema deve permitir saques para contas bancárias associadas.
    Taxa de saque: 2% do valor.
    Saques devem ser processados em até 2 dias úteis.

    4.5. Visualização e Interação com o Ambiente

    RF-029: Informações do Local
    O sistema deve exibir informações detalhadas do local:
    Nome e logo
    Endereço completo
    Botão para abrir no Google Maps

    RF-030: Integração com Transporte
    O sistema deve oferecer botão para chamar motorista (Uber).
    O destino deve ser configurado para área de estacionamento da arena.

    4.6. Notificações

    RF-031: Sistema de Notificações
    O sistema deve enviar notificações para eventos importantes:
    Nova lista no clube do usuário
    Mudança de horário/valor em lista participante
    Reserva concedida
    Lista perdida
    Reembolso processado
    Chargeback detectado
    Cancelamento pela arena

    4.7. Autenticação e Gerenciamento de Conta (Arena)

    RF-032: Registro e Login de Funcionário
    O sistema deve permitir registro/login de funcionários da arena.
    Métodos aceitos: email/senha ou Google OAuth.
    Associação com estabelecimento deve ser gerenciada manualmente no MVP.

    4.8. Gerenciamento de Listas e Reservas (Arena)

    RF-033: Criação de Lista pela Arena
    O sistema deve permitir criação de novas listas pelo funcionário da arena.
    Parâmetros obrigatórios:
    Quadra
    Horário de início
    Duração da partida
    Tamanho mínimo da lista

    RF-034: Compartilhamento de Link
    O sistema deve gerar link único após criação da lista.
    Funcionário deve poder copiar/compartilhar o link.

    RF-035: Visualização de Calendário
    O sistema deve exibir calendário com todas as listas e reservas do local.
    Visualização deve incluir status e horários.
    Filtros por data e quadra devem estar disponíveis.

    RF-036: Cancelamento de Reserva
    O sistema deve permitir cancelamento de reservas confirmadas.
    Funcionário deve informar motivo do cancelamento.
    Sistema deve processar reembolso integral automaticamente.

    4.9. Check-in de Jogadores

    RF-037: Scanner de QR Code
    O sistema deve permitir escaneamento de QR codes apresentados pelos jogadores.
    Scanner deve validar autenticidade e validade do código.
    Sistema deve registrar check-in com timestamp.

    4.10. Relatórios

    RF-038: Relatórios de Métricas
    O sistema deve gerar relatórios com métricas:
    Receitas por período
    Ticket médio
    Percentual de ocupação
    Quantidade de agendamentos
    Índice de faltas
    Períodos disponíveis: dia, semana, mês, ano.

    RF-039: Drill-down por Quadra
    O sistema deve permitir detalhamento de métricas por quadra específica.
    Drill-down deve estar disponível através de clique nos gráficos principais.

    4.11. Processos e Regras de Negócio Centrais

    RF-040: Gerenciamento de Estados de Lista
    O sistema deve implementar máquina de estados para listas:
    VAZIA → ABERTA → EM DISPUTA → ESPERANDO → RESERVADO → CONCLUÍDO
    Estados alternativos: PERDIDO, CANCELADA, CANCELADA_PELA_ARENA
    Transições devem seguir regras específicas detalhadas no Apêndice A.

    RF-041: Sistema de Precificação Dinâmica
    O sistema deve calcular Preço Individual Final (PIF) baseado em:
    Preço Base da Reserva da Arena (PBRA)
    Tamanho Mínimo da Lista (TML)
    Score de Demanda do Horário/Quadra (SDHQ)
    Score do Usuário (SU)
    Desconto de organizador (20% para primeiro pagante)
    Fórmula detalhada no Apêndice B.

    RF-041.1: Expiração do Preço Dinâmico
    O preço dinâmico calculado para um usuário para um serviço específico deve expirar após 1 minuto e meio (90 segundos) da sua apresentação inicial ao usuário. Após a expiração, uma nova consulta ou tentativa de pagamento deve recalcular o preço com base nas condições atuais.

    RF-042: Sistema de Prioridade
    O sistema deve garantir prioridade para listas que atingem estado ESPERANDO.
    Listas concorrentes devem ser movidas para PERDIDO automaticamente.
    Horário deve ser bloqueado para novas listas.

    RF-043: Regras de Reembolso
    O sistema deve aplicar regras específicas para reembolso:
    Permitido se >24h antes da reserva
    Permitido em caso de cancelamento/alteração pela arena
    Permitido em caso de alteração significativa pelo organizador
    Processado automaticamente para carteira in-app

    RF-044: Detecção de Chargeback
    O sistema deve detectar chargebacks.
    Primeiro chargeback deve resultar em bloqueio da conta para análise manual.

    RF-045: Expiração de QR Code
    O sistema deve fazer QR codes expirarem após término do horário da reserva.

    RF-046: Registro de Faltas
    O sistema deve registrar falta se <50% dos pagantes mínimos fizerem check-in até 10 min após início da reserva.

    4.12. Avaliação de Estabelecimento e Atendimento

    RF-047: Avaliação da Experiência
    O sistema deve permitir que jogadores avaliem o estabelecimento (Arena) e o atendimento recebido após a conclusão de uma reserva (lista no estado CONCLUIDO).
    A avaliação deve incluir:
    Nota geral (ex: escala de 1 a 5 estrelas)
    Comentário opcional
    O sistema deve associar a avaliação à reserva específica e ao estabelecimento.
    Jogadores com Conta PIX ou Full devem ter permissão para avaliar. Visitantes não podem avaliar.

    RF-048: Visualização de Avaliações
    O sistema deve permitir que jogadores visualizem a nota média e comentários das avaliações de um estabelecimento (Arena).
    Esta informação deve ser exibida na tela de detalhes do local no App do Jogador.

5.  Modelo de Dados (Entidades Principais)
    **_EM PROGRESSO_**

6.  Requisitos Não Funcionais

    6.1. Usabilidade
    RNF-001: Interface Intuitiva
    A interface do usuário deve ser intuitiva e fácil de usar para todos os perfis de usuário.
    Tempo de aprendizado para funcionalidades básicas deve ser inferior a 15 minutos.

    RNF-002: Feedback do Sistema
    O sistema deve fornecer feedback claro ao usuário sobre suas ações e o status do sistema.
    Mensagens de sucesso, erro e carregamento devem ser exibidas consistentemente.

    RNF-003: Navegação Consistente
    A navegação principal nos aplicativos deve ser consistente e previsível.
    Máximo de 3 toques para acessar qualquer funcionalidade principal.

    6.2. Performance
    RNF-004: Tempo de Resposta para Preços
    A consulta de preço individual e disponibilidade de horários deve ser respondida em até 3 segundos sob carga normal.

    RNF-005: Processamento de Pagamentos
    O processamento de pagamentos (PIX) deve refletir o status na carteira em tempo próximo ao real.
    Máximo de 30 segundos para confirmação via webhook.

    RNF-006: Carregamento de Listagens
    As telas de listagem (listas, reservas, relatórios) devem carregar em até 5 segundos com até 100 itens.

    6.3. Segurança
    RNF-007: Criptografia de Dados Sensíveis
    Todos os dados sensíveis do usuário (CPF, dados de pagamento parciais, senhas) devem ser armazenados e transmitidos de forma segura.
    Uso obrigatório de HTTPS para todas as comunicações.

    RNF-008: Autenticação JWT
    O sistema deve usar autenticação baseada em JWT para proteger as APIs.
    Tokens devem ter tempo de expiração apropriado (24h para mobile, 1h para web).

    RNF-009: Transparência de Dados
    O sistema deve informar claramente ao usuário o motivo da coleta de dados pessoais, especialmente o CPF.

    RNF-010: Prevenção de Fraude
    O sistema deve ter mecanismos para prevenir e detectar atividades fraudulentas básicas.
    Limite de tentativas de pagamento falhado: 3 por dia.

    6.4. Confiabilidade/Disponibilidade
    RNF-011: Disponibilidade do Sistema
    O sistema deve ter uma disponibilidade de 99.5% para as funcionalidades críticas.
    Funcionalidades críticas: login, criação/visualização de listas, pagamento.

    RNF-012: Persistência de Links
    Links públicos de listas devem permanecer acessíveis.
    Links não devem expirar mesmo após conclusão da lista.

    6.5. Manutenibilidade
    RNF-013: Qualidade do Código
    O código deve seguir padrões de desenvolvimento e ser bem documentado.
    Cobertura de testes automatizados deve ser superior a 80%.

    6.6. Integrações
    RNF-014: Integração com Pagamentos
    Integração com API de Pagamentos (PIX Open Finance, Cartões).
    Suporte a webhooks para confirmação em tempo real.

    RNF-015: Integração com Google OAuth
    Integração com Google OAuth para autenticação.
    Suporte a refresh tokens para manter sessões ativas.

    RNF-016: Integração com Google Maps
    Integração com Google Maps através de abertura de link externo.

    RNF-017: Integração com Apps de Transporte
    Integração com apps de transporte (ex: Uber) através de abertura de link externo.

    6.7. Conformidade
    RNF-018: Conformidade com LGPD
    O sistema deve estar em conformidade com a Lei Geral de Proteção de Dados (LGPD).
    Implementação de consentimento explícito para coleta de dados.
    Funcionalidade para exclusão de dados pessoais mediante solicitação.

7.  Fora do Escopo (MVP)
    Funcionalidades que não serão implementadas na primeira versão:
    Chat/mensagens entre jogadores
    Sistema de avaliação/rating de jogadores
    Integração com redes sociais
    Notificações push
    Múltiplos idiomas
    Programa de fidelidade/pontos
    Reservas recorrentes
    Sistema de disputa/arbitragem
    API pública para terceiros

8.  Suposições e Dependências
    Suposições:
    Usuários possuem smartphones com acesso à internet
    Arenas possuem funcionários capazes de operar o app
    Provedores de pagamento mantêm APIs estáveis
    Google OAuth permanece disponível e gratuito

    Dependências:
    API de pagamentos (PIX Open Finance)
    Google OAuth
    Serviços de blob storage para imagens
    Provedor de hospedagem cloud
    Certificados SSL/TLS

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
Fórmula Geral: PIF (Jogador J, Lista L) = (PIB*Lista_L * Fator*Demanda_Horario_L * Fator*Score_Jogador_J) - Desconto_Organizador (se aplicável)
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
Se o Jogador J for o primeiro pagante da Lista L (tornando-se Organizador), um desconto de 20% é aplicado sobre o valor (PIB_Lista_L * Fator*Demanda_Horario_L * Fator*Score_Jogador_J).
Processo de Cálculo ao Jogador Consultar Preço/Entrar na Lista:
Identificar Lista L e Jogador J.
Obter PBRA para o [horário+quadra] da Lista L.
Obter TML da Lista L. Calcular DE e PIB_Lista_L.
Calcular/Obter o SDHQ atual para o [horário+quadra] da Lista L e derivar o Fator_Demanda_Horario_L.
Obter o SU_Jogador_J e derivar o Fator_Score_Jogador_J.
Calcular o preço antes do desconto: Preco_Intermediario = PIB_Lista_L * Fator*Demanda_Horario_L * Fator_Score_Jogador_J.
Se Jogador J for elegível para desconto de organizador (primeiro pagante da lista VAZIA): PIF = Preco_Intermediario \* 0.80.
Senão: PIF = Preco_Intermediario.
Arredondar o PIF para 2 casas decimais.
Este é o PIF que será exibido e cobrado do jogador.
