# Projeto Final de Laboratório de Bases de Dados (SCC0541) - 2022/1

## Integrantes
- Gabriel Fontes (10856803)
- Giovanna Fardini (10260671)
- Kalilo Gonçalves (11836754)
- Vinícius Baca (10788589)

## Como executar

Temos uma versão live executando em um servidor meu. Está disponível em
https://bd.misterio.me.

### Base de dados

A SGBD usada foi o Postgres. Na pasta `db` temos vários arquivos `.sql`:
- Providos no T1, sem modificação:
    - `01-ddl_inicial.sql` - DDL para criar a schema de fórmula 1. Esse é o provido no T1.
    - `02-dml_inicial.sql` - DML de dados providos no T1, e dumpados.
- Criados por nós:
    - `03-tabela_e_triggers.sql` - DDLs de tabelas (usuários, log), funções para
      registrar usuários, e respectivos triggers.
    - `04-criar_usuarios.sql` - Executa as funções do `03`, criando os usuários
      iniciais (admin, constructors, drivers).
    - `05-nulabilidade_mais_rigida` - Adiciona alguns constraints (pk e NOT
      NULLs) em constructors e drivers, para facilitar inserção e busca.
    - `06-funcoes_overview_relatorios_acoes.sql` - Adiciona as funções dos
      relatórios, métricas das overviews, e função para a ação da escuderia
      (buscar motorista).

### Aplicação

A aplicação foi desenvolvida na linguagem [Rust](https://rust-lang.org), com o
framework [Rocket](https://rocket.rs).

- Instale o compilador (rustc) e gerenciador de pacotes (cargo) do rust. Na
  maioria das distribuições de Linux é um pacote chamado `cargo` ou `rust`.
    - Você também pode instalar o `rustup`, que gerencia toolchains.
    - Passos disponíveis no [site do rust](https://rust-lang.org).
- Abra o arquivo `Rocket.toml`, e troque a string de conexão do postgres para a
  que você usará.
- Compile & execute com `cargo run`.
    - Você pode apenas compilar com `cargo build`.
- Pronto! Basta abrir o navegador em http://127.0.0.1:8000 (você pode mudar a
  porta no `Rocket.toml`)

#### Com o nix (opcional)

Se você tiver o gerenciador de pacotes [nix](https://nixos.org), você pode
executar com `nix run` e compilar com `nix build`.
