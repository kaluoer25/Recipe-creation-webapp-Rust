# Recipe-creation-webapp-Rust

This is a backend food recipe creation webapp written in Rust. 
Users can add in their food recipe. 
##
Techstack:
1) Rust
2) Actix-web
3) PostgresSQL 
4) curl
##

1. Health check result

<img width="1265" alt="Screenshot 2023-02-08 at 11 44 52 PM" src="https://user-images.githubusercontent.com/101921758/218033720-b1f08dfb-1c2f-4b81-b437-2b214f7870aa.png">

2. #1 Testing 
<img width="1262" alt="Screenshot 2023-02-08 at 11 41 04 PM" src="https://user-images.githubusercontent.com/101921758/218034423-42dd825c-3639-4119-8fc3-65b243a1d105.png">


3.Iter1 - cargo run --bin iter 1 
<img width="891" alt="Screenshot 2023-02-10 at 10 23 10 AM" src="https://user-images.githubusercontent.com/101921758/218034464-3b83b9e3-f607-42cb-97d2-0f829258bff4.png">

4.Iter2 - cargo check --bin iter 2
<img width="814" alt="Screenshot 2023-02-10 at 11 07 52 AM" src="https://user-images.githubusercontent.com/101921758/218034573-9295e958-ebcf-4f36-8d4d-45055ed5fdc9.png">

5.Iter3 - cargo check --bin iter 3
<img width="812" alt="iter3 ss" src="https://user-images.githubusercontent.com/101921758/218089615-fdc42683-6c8c-496f-95eb-cb44bd75e122.png">


6.Iter4 -  cargo run --bin iter 4 then do a curl request. See below:

###

curl -X POST localhost:3000/recipes/ -H "Content-Type: application/json" -d '{"recipe_id":3, "user_id": 1, "recipe_name":"Halibut fish and chips"}’

<img width="863" alt="Screenshot 2023-02-10 at 11 47 15 AM" src="https://user-images.githubusercontent.com/101921758/218034915-2b0468ab-0d39-4d8b-bec5-a2b769318cfc.png">

##

Database posgres: 
Added tables. There are total 3 database created to 

<img width="486" alt="Screenshot 2023-02-10 at 8 26 15 PM" src="https://user-images.githubusercontent.com/101921758/218092409-567b5207-4bc6-48b9-b43d-be97ed287b4f.png">
 



