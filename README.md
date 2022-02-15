# CHARMcatalogue

A fully fledged digital catalogue for the CHARMfair at Chalmers. The system is built to be easily usable and customizable by an administrator with minimal programming understanding. The current published version in production should be available on http://catalogue.charm.chalmers.se

## Features

- Company pages
    - Individually hideable for users
    - Company Logo
    - Description of the company
    - Show relevant tags (such as Programs, Offering, Business Areas, Looking for)
    - Summer jobs, if relevant
    - Map of where at the fair the company is located
    - Link to their website
    - Save favorite companies
    - Take notes (only saved locally)
- Pre pages (Custom png for other information)
- Searching and filtering companies
- Shortcuts 
    - To premade search and filtering query
    - To anywhere on the web
- Batch updates 
    - Upload a csv to update any information saved in database
    - Download current configuration (to save or upload later)
- Customizable layout 
    - Choose which company information to show on company pages
    - Adds customized art for all pages
- Tagging system
    - Explicitly handles:
        - Programs the company is interested in
        - Business areas they operate in
        - The level of study they're looking for (Bachelor, Masters, PhD)
        - Types of job the offer
    - Can handle generic tags as well
- Easy to use admin interface
    - Allows configuration and changing of most (if not all) information entered about companies, tags, prepages, shortcuts, etc.

## Batch uploading

When the system is setup it can easily be populated by uploading a `.zip` file containing a `.csv` file along with all the images (named correctly) as an administrator. 

An example of such a file can be found [here](https://drive.google.com/drive/folders/1ARqpngACz8koJlrudFBCM7jHow94vemY?usp=sharing)

# Setup
## Production

### Requirements

A system configured with `Docker` and `Docker-compose`

### Running 

Download and the `production.yml` file found in `.` and run it with `docker-compose up` 


## Development

### Requirements

A system configured with `Docker`, `Docker-compose` and `yarn`

### Running 
The backend is started by simple running 
```
docker-compose up
```

For the frontend go in to the `vue` directorty and run 
```
yarn install 
```
Followed by 
```
yarn serve
```


