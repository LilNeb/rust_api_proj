# Comment utiliser


## Setup un cron 


`crontab -e`

rajouter une ligne commande dans le cron :

`* * * * * ./[path to binary]`

ex :

`* * * * * /Users/username/Documents/Projets/rust_api_proj/target/release/rust_api_proj`


comment avoir le fichier binaire : 

`cargo build --release`

## Run le front

Une fois suffisament de données recoltées (sinon les graphs n'auront pas de sens)

Se placer dans le fichier front_end

`trunk serve` 

Se rendre sur l'url imprimée