## Goal
Detta project syftar till konstrukera en digital katalog för CHARM. Interfacet ska tillhanda hålla:
* Vy för företags katalog sidan
* Sökfunktion för företag
* Sökfunktion på taggar, för att hitta relevanta företag
* Vy för att användar att ge företag taggar
* Vy för att skapa nya taggar
* Vy för administör att modderar systemet:
  * Skapa/redisera/ta bort taggar/företag/taggar på företag

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



## Data formatting of csv files
tags.csv is structure in a tree structure, where indentions indicates a sublevel. The structure always infinitely many sublevels but the file must be valid csv. That means that every row needs to have the same number of : and can't have tailing newlines. Empty cells such med totally empty aka '' and no value such have unnecessary white. 

Note: The example below has extra whitespace to make it more readable.
```
root_tag_1 :            :
           : sub_tag1_1 :
           :            : sub_sub_tag1_1_1
           : sub_tag1_2 :
root_tag_2 :            :
.              .            .
.              .            .
.              .            .
```


data.csv is structured with the first two cols as metadata, followed by a a matrix of tags and companies. The metadata is company name and the company catalog page. The tag company matrix is structured as if a company_A has tag_A then put TRUE cell where tag_A and company_A intersects. Empty cells such med totally empty aka '' and no value such have unnecessary white. 

Note: The example below has extra whitespace to make it more readable.
```
Company : page : tag_A : tag_B : ...
comp_A  : 1    : TRUE  :       : ...
comp_B  : 2    : TRUE  : TRUE  : ...
comp_C  : 3    :       : TRUE  : ...
.         .      .       .       ...
.         .      .       .       ...
.         .      .       .       ...
```
