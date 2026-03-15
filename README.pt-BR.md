# m00wm

[![en](https://img.shields.io/badge/English-380000?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.md) [![es](https://img.shields.io/badge/Español-380000?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.es.md) [![br](https://img.shields.io/badge/Português-EB5406?style=for-the-badge&logoColor=white&logo=DocuSign)](https://github.com/m00sp/m00wm/blob/main/README.pt-BR.md)

Um gerenciador de janelas tiling de X11 escrito em Rust, construído do zero usando a excelente crate [penrose](https://github.com/sminez/penrose).

Este é um projeto educacional que constrói um gerenciador de janelas totalmente funcional de forma incremental. Certifique-se de ter um ambiente de desktop alternativo disponível caso algo falhe durante os testes.

> **Agradecimentos especiais** a [sminez](https://github.com/sminez) pela excelente crate penrose e documentação abrangente que tornaram este projeto possível!

## Visão Geral

m00wm é um gerenciador de janelas tiling minimalista que oferece simplicidade e controle ao seu desktop X11. Construído com Rust, prioriza desempenho, estabilidade e flexibilidade.

**Acompanhe o desenvolvimento:**
- Assista ao processo de criação no [YouTube](https://www.youtube.com/playlist?list=PLy2HjaQiG8lOxCKzuWKfmmXov4iEVOGOF)
- Verifique [progress-so-far.md](./progress-so-far.md) para um changelog dos recursos implementados

## Características

- **Layouts tiling** – Gerenciamento eficiente de espaços de trabalho com múltiplas opções de layout
- **Atalhos de teclado personalizáveis** – Atalhos de teclado totalmente configuráveis
- **Barra de status** – Barra de status integrada com informações do sistema
- **Suporte a múltiplos espaços de trabalho** – Organize janelas em espaços de trabalho virtuais
- **Gerenciamento de janelas flutuantes** – Suporte para janelas flutuantes quando necessário
- **Espaçamento dinâmico** – Lacunas configuráveis entre janelas

## Stack Tecnológico

- **Linguagem:** [Rust](https://rust-lang.org)
- **Framework de Gerenciador de Janelas:** [Penrose](https://github.com/sminez/penrose)
- **Protocolo X11:** x11rb
- **Logging:** tracing-subscriber

## Pré-requisitos

- [Rust](https://rustup.rs/) (última versão estável)
- Headers de desenvolvimento de X11
- Um gerenciador de display que suporte sessões de desktop personalizadas
- Um ambiente de desktop alternativo para recuperação (recomendado durante o desenvolvimento)

## Instalação

> **NOTA:** Leia o [Makefile](./Makefile) antes de instalar para entender o que será executado. Não há nada prejudicial, mas sempre revise o que está executando com `sudo`!

1. Clone o repositório:
```bash
git clone https://github.com/m00sp/m00wm.git
cd m00wm
```

2. Revise e personalize os atalhos de teclado em `src/main.rs` se necessário (padrão usa terminal `st` e `dmenu_run`):
```bash
# Substitua as referências padrão de terminal e lançador
vim src/main.rs
```

3. Compile e instale:
```bash
make build && sudo make install
```

O gerenciador de janelas estará disponível como uma sessão de desktop no seu gerenciador de display. Selecione "m00wm" ou "Plasma m00wm" ao fazer login.

## Compilação e Desenvolvimento

Compile em modo de release:
```bash
make build
```

Teste em uma sessão X aninhada:
```bash
make test
```

Desinstale:
```bash
sudo make uninstall
```

## Estrutura do Projeto

```
src/
├── main.rs           # Ponto de entrada, atalhos de teclado e configuração do gerenciador
├── lib.rs            # Exportações de biblioteca e constantes
├── bar.rs            # Implementação da barra de status
├── layouts.rs        # Definições de layouts tiling
examples/             # Configurações de exemplo
config/               # Arquivos de sessão de desktop
scripts/              # Utilitários de compilação e teste
```

## Configuração

As configurações principais são definidas em `src/main.rs`:
- **Atalhos de teclado** – Modifique o mapa de atalhos para personalizar os atalhos
- **Layouts** – Adicione ou ajuste layouts tiling em `src/layouts.rs`
- **Cores e Fontes** – Opções de tema em constantes no topo de `main.rs`
- **Barra de Status** – Personalize a exibição da barra de status em `src/bar.rs`

## Obter Ajuda

- Verifique [progress-so-far.md](./progress-so-far.md) para o status de implementação
- Revise a [documentação do penrose](https://github.com/sminez/penrose)
- Explore exemplos no diretório `examples/`

## Agradecimentos

- **sminez** pelo framework [penrose](https://github.com/sminez/penrose) de gerenciador de janelas e excelente documentação

Translated using GitHub Copilot and GPT-4o.
