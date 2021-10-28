## Goal

Detta project syftar till konstrukera en digital katalog för CHARM. Interfacet ska tillhanda hålla:

- Vy för företags katalog sidan
- Sökfunktion för företag
- Sökfunktion på taggar, för att hitta relevanta företag
- Vy för att användar att ge företag taggar
- Vy för att skapa nya taggar
- Vy för administör att modderar systemet:
  - Skapa/redisera/ta bort taggar/företag/taggar på företag

## Plan

Vi räknar med 5000 simultan användare, därför lägger syftar vi till att lägga så mycket beräkningare på användare. Vi tillhanda håller en database ochen lightweight api. För att möljigöra skaling så bygger vi projektet som microserivces.

If extra performance is need the backend might be rewriten in a higher performance lang such as rust.

## Documation

Dokumation kan generas genom att kör doxygen doxygen_conf i doc/

## Setup

Systemet använder docker och för att starta det skriver du

```
docker-compose up
```

NOTE: Detta kräver att du har [docker](https://www.docker.com/) installerat på din dator allt annat löser den själv

## Api endpoints

Below is a map of the endpoints provide by the api.
Keys:
A - Requires user to be logged in, all endpoints of this type is placed under directory admin.

```
api-|-tag-----|-create
    |         |-add
    |         |-match
    |         |-get
    |         |-update(A)
    |         |-comapany-update(A)
    |
    |-company-|-get
    |         |-update(A)
    |
    |-manage--|-load(A)
    |
    |-auth----|-login
              |-logout(A)
              |-signout(A)
              |-change_password(A)
```

## Data formatting of xlsx file

The xlsx file can be send to `/api/manage/load` loading the the tags and companies into the datebase. A template is avalible [here](https://docs.google.com/spreadsheets/d/14_-CgmI3QuasI0pq-A9_Za_QoODaMty4KRgX8zlVPyc/edit?usp=sharing).

The tags sheet is structure in a tree structure, where indentions indicates a sublevel. In template the tag 'Software' has two subtag 'Open source' and 'AI' The structure always infinitely many sublevels.

The Companies sheet is structured with columns A-J being metadata metadata, followed by a list of tags. The tag list is structured as if a company_A has tag_A then put TRUE(click the checkbox) cell where tag_A and company_A intersects.
