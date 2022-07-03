# Projeto Final de Laboratório de Bases de Dados (SCC0541) - 2022/1

## Integrantes

TODO

## Como executar

Depois de clonar, certifique-se de executar `git submodule init && git
submodule update` para clonar os submódulos.

TODO

## Sobre stack

TODO

## Checklist funcionalidades

### Usuários
- [ ] Tabela `USERS`
    - `userid`: sequencial
    - `tipo`: `administrador`, `escuderia`, ou `piloto`
    - `idoriginal`
    - `login`:
        - admins: `admin`
        - escuderias: `<constructorref>_c`
        - pilotos: `<driverref>_d`
    - `password`
        - Hasheada com MD5 direto pelo postgres
        - admins: `admin`
        - escuderias: `<constructorref>`
        - pilotos: `<driverref>`
- [ ] Triggers para criar usuários automaticamente
- [ ] log_table para cada acesso ao sistema (userid, data, hora)

### Telas
- [ ] Tela de login
- [ ] Tela de overview
    - Apresentar nome (admin/construtora/nome do piloto)
    - Dashboard
    - Botões para ações possíveis
- [ ] Tela de relatórios

### Overviews
- [ ] admins:
    - [ ] Qtde de pilotos cadastrados
    - [ ] Qtde de escuderias cadastradas
    - [ ] Qtde de corridas cadastradas
    - [ ] Qtde de temporadas cadastradas
- [ ] escuderias:
    - [ ] Qtde de vitórias dela
    - [ ] Qtde de pilotos que correram por ela
    - [ ] Primeiro e último ano com dados dela (RESULTS)
- [ ] pilotos:
    - [ ] Qtde de vitórias
    - [ ] Primeiro e último ano com dados dele (RESULTS)

### Ações:
- [ ] admins:
    - [ ] Cadastrar escuderias (constructorref, name, nationality, url)
    - [ ] Cadastrar pilotos (driverref, number, code, forename, surname, date of birth, nationality)
- [ ] escuderias:
    - [ ] Dado `forename`, mostrar piloto (caso já tenha corrido pela escuderia)

### Relatórios
TODO: vide PDF
