# Projeto Final de Laboratório de Bases de Dados (SCC0541) - 2022/1

## Integrantes

TODO

## Como executar

TODO

## Sobre stack

TODO

# Lista de funcionalidades e rotas

- [x] GET `/` -> Home, redireciona pra login ou overview

## Autenticação

Apenas não-autenticado:
- [x] GET `/login` -> Tela de login
- [x] POST `/login` -> Ação de login

Apenas autenticado:
- [x] POST `/logout` -> Ação de logout

## Dashboard

Apenas autenticado:
- [x] GET `/overview` -> Tela da overview

## Ações

Apenas admin:
- [ ] GET `/actions/add-constructor` -> Tela de criação de constructor
- [ ] POST `/actions/add-constructor` -> Ação de criação de constructor
- [ ] GET `/actions/add-driver` -> Tela de criação de driver
- [ ] POST `/actions/add-driver` -> Ação de criação de driver

Apenas escuderia:
- [ ] GET `/actions/show-driver?<forename>` -> Tela de info de piloto dado nome

## Relatórios

Apenas admin:
- [x] GET `/reports/1` -> Resultados de corridas COUNT por status
- [x] GET `/reports/2` -> Coisa lá dos aeroportos

Apenas escuderia:
- [x] GET `/reports/3` -> Listagem dos pilotos e primeiras posição
- [x] GET `/reports/4` -> Resultados de corridas COUNT por status somente dos seus pilotos

Apenas piloto:
- [x] GET `/reports/5` -> Vitórias obtidas por ano e corrida c/ ROLLUP
- [x] GET `/reports/6` -> Resultados de corridas COUNT por status somente de si

