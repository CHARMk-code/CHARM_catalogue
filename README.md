## Goal

This project aims to provide a digital catalogue for CHARM.
Strategic goal:

- Simplify for the average student to find companies that fit them.

Which should provide:

- A view for filter and search for companies
- A admin interface for creating and modifying companies
- A admin interface for batch creating companies
- A interface for authenticating admins.

It would be improved by:

- Be well adopted for mobile
- Support map functionally to display where company are
- Support for liking(I would like to come back to this) and writing notes
- Allow user to add and up/down vote tags for company, crowd sourcing want tags each company right has

## Plan

The system is likely to expenses a large number (ca 5000) of simultaneous user with limits hosting resources, as such we aim to off load computation on to the user.

## Documentation

Documentation can be created by running `doxygen doxygen_conf` in doc/

## Setup

To start the backend use

```
docker-compose up
```

This requires [docker](https://www.docker.com/).

To start the frontend running

```
yarn install && yarn serve
```

in vue directory.
which install depends and starts the host server, requires yarn.

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

## Create/modify companies

There are multiple way of doing this, singular company are easiest handle interactivity in the frontend, while a large among of companies can easily be created or modify by uploading a xlsx file.

### Interactive method

TODO: Write when functionally is available.

### Batch creation

Images and company data can be upload via `/api/mange/load` endpoint, this endpoint allows single image, just the data(in a .xlsx file), or any number of image and/or data in a zip or tar.gz file.
A example is available [here](https://drive.google.com/drive/folders/1ARqpngACz8koJlrudFBCM7jHow94vemY?usp=sharing).

#### Data formatting of xlsx file

The tags sheet is structure in a tree structure, where indentions indicates a sublevel. In template the tag 'Software' has two subtag 'Open source' and 'AI' The structure always infinitely many sublevels. The first three cols are metadata, which is used for set special properties on the tag, such as it being a student division.

The Companies sheet is structured with columns A-J being metadata metadata, followed by a list of tags. The tag list is structured as if a company_A has tag_A then put TRUE(click the checkbox) cell where tag_A and company_A intersects.

## Contribute

In order to keep the improve code quality, all contributions such be peer review via a pull request before being push the master.
