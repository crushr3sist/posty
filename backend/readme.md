# advice

22/02/2025
when setting up sea-orm, reference this commit 1f599d6 to get example of a migration
we need to run the sea-orm-cli to write migrations, once thats done, we need to write a migration. 
once a migration is written, we need to migrate up. But since the migration has a lib.rs that has the list of migrations, 
we can import that into our our main func and run the migrations automatically. 
the models arent needed for anything aside from referencing the migration. I will check in after a bit and report the queries working. 

