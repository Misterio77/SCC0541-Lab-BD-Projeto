# Projeto Final de Laboratório de Bases de Dados (SCC0541) - 2022/1

## Integrantes

TODO

## Como executar

TODO

## Sobre stack

TODO

# Lista de funcionalidades e rotas

- GET `/` -> Home

## Autenticação

Apenas não-autenticado:
- GET `/login` -> Tela de login
- POST `/login` -> Ação de login

Apenas autenticado:
- POST `/logout` -> Ação de logout

## Dashboard

Apenas autenticado:
- GET `/overview` -> Tela da overview

## Ações

Apenas admin:
- GET `/actions/add-constructor` -> Tela de criação de constructor
- POST `/actions/add-constructor` -> Ação de criação de constructor
- GET `/actions/add-driver` -> Tela de criação de driver
- POST `/actions/add-driver` -> Ação de criação de driver

Apenas escuderia:
- GET `/actions/add-driver?<forename>` -> Tela de info de piloto dado nome

## Relatórios

Apenas admin:
- GET `/reports/1` -> Resultados de corridas COUNT por status
- GET `/reports/2` -> Coisa lá dos aeroportos

Apenas escuderia:
- GET `/reports/3` -> Listagem dos pilotos e primeiras posição
- GET `/reports/4` -> Resultados de corridas COUNT por status somente dos seus pilotos

Apenas piloto:
- GET `/reports/5` -> Vitórias obtidas por ano e corrida c/ ROLLUP
- GET `/reports/6` -> Resultados de corridas COUNT por status somente de si

