use std::env::Args;
use std::io::{Error, ErrorKind, stdin, stdout, Write};
use std::process::exit;
use std::str::FromStr;
use log::error;
use sha256::{digest};
use crate::config::dbconfig::establish_connection;
use crate::constants::constants::Role;
use crate::models::user::{User, UserWithoutPassword};
use crate::utils::time::get_current_timestamp_str;
use rpassword::read_password;
use crate::models::device::Device;
use crate::models::episode::Episode;
use crate::models::favorites::Favorite;
use crate::models::models::PodcastHistoryItem;
use crate::models::session::Session;
use crate::models::subscription::Subscription;


pub fn start_command_line(mut args: Args){
    println!("Starting from command line");
    match args.nth(1).unwrap().as_str() {

        "help"|"--help"=>{
            println!(r" The following commands are available:
            users => Handles user management
            ")
        }
        "users"=>{
            println!("User management");
            match args.nth(0).unwrap().as_str() {
                "add"=> {
                    let mut user = read_user_account();


                    println!("Should a user with the following settings be applied {:?}",user);

                    match ask_for_confirmation(){
                        Ok(..)=>{
                            user.password = Some(digest(user.password.unwrap()));
                            match User::insert_user(&mut user, &mut establish_connection()){
                                Ok(..)=>{
                                    println!("User succesfully created")
                                },
                                Err(..)=>{

                                }
                            }
                        },
                        Err(..)=> {
                        }
                    }
                }
                "remove"=> {
                    let mut username = String::new();
                    // remove user
                    let available_users = list_users();
                    retry_read("Please enter the username of the user you want to delete",
                               &mut username);
                    username = trim_string(username);
                    println!("{}", username);
                    match available_users.iter().find(|u|u.username==username){
                        Some(..)=>{
                            PodcastHistoryItem::delete_by_username(trim_string(username.clone()),
                                                                   &mut establish_connection())
                                .expect("Error deleting entries for podcast history item");
                            Device::delete_by_username(username.clone(), &mut
                                establish_connection())
                                .expect("Error deleting devices");
                            Episode::delete_by_username_and_episode(trim_string(username.clone()),
                                                                    &mut establish_connection())
                                .expect("Error deleting episodes");
                            Favorite::delete_by_username(trim_string(username.clone()),
                                                         &mut establish_connection())
                                .expect("Error deleting favorites");
                            Session::delete_by_username(&trim_string(username.clone()),
                                                        &mut establish_connection())
                                .expect("Error deleting sessions");
                            Subscription::delete_by_username(&trim_string(username.clone()),
                                                             &mut establish_connection()).expect("TODO: panic message");
                            User::delete_by_username(trim_string(username.clone()),
                                                     &mut establish_connection())
                                .expect("Error deleting user");
                        println!("User deleted")
                        },
                        None=>{
                            println!("Username not found")
                        }
                    }
                }
                "update"=>{
                    //update a user
                    list_users();
                    let mut username = String::new();

                    retry_read("Please enter the username of the user you want to delete",
                               &mut username);
                    username = trim_string(username);
                    println!(">{}<", username);
                    match User::find_by_username(username.as_str(), &mut establish_connection()){
                        Some(user)=>{
                            do_user_update(user)
                        },
                        None=>{
                            println!("Username not found")
                        }
                    }

                }
                "list"=> {
                    // list users

                    list_users();
                }
                "help"|"--help"=>{
                    println!(r" The following commands are available:
                    add => Adds a user
                    remove => Removes a user
                    update => Updates a user
                    list => Lists all users
                    ")
                }
                _ => {
                    error!("Command not found")
                }
            }
        }
        _ => {
            error!("Command not found")
        }
    }
}

fn list_users() -> Vec<UserWithoutPassword> {
    let users = User::find_all_users(&mut establish_connection());

    users.iter().for_each(|u| {
        println!("|Username|Role|Explicit Consent|Created at|", );
        println!("|{}|{}|{}|{}|", u.username, u.role, u.explicit_consent, u.created_at);
    });
    users
}


pub fn read_user_account()->User{
    let mut username = String::new();
    let password;

    let role = Role::VALUES.map(|v|{
        return v.to_string()
    }).join(", ");
    retry_read("Enter your username: ", &mut username);

    let user_exists = User::find_by_username(&username, &mut establish_connection()).is_some();
    if user_exists{
        println!("User already exists");
        exit(1);
    }
    password = retry_read_secret("Enter your password: ");
    let assigned_role = retry_read_role(&format!("Select your role {}",&role));

    let user = User{
        id: 0,
        username: trim_string(username.clone()),
        role: assigned_role.to_string(),
        password: Some(trim_string(password)),
        explicit_consent: false,
        created_at: get_current_timestamp_str(),
    };

    user
}

pub fn retry_read(prompt: &str, input: &mut String){
    println!("{}",prompt);
    stdin().read_line(input).unwrap();
    match  input.len()>0{
        true => {
            if input.trim().len()==0{
                retry_read(prompt, input);
            }
        }
        false => {
            retry_read(prompt, input);
        }
    }
}


pub fn retry_read_secret(prompt: &str)->String{
    println!("{}",prompt);
    stdout().flush().unwrap();
    let input = read_password().unwrap();
    match  input.len()>0{
        true => {
            if input.trim().len()==0{
                retry_read_secret(prompt);
            }
        }
        false => {
            retry_read_secret(prompt);
        }
    }
    input
}

pub fn retry_read_role(prompt: &str)->Role{
    let mut input = String::new();
    println!("{}",prompt);
    stdin().read_line(&mut input).unwrap();
    let res = Role::from_str(&trim_string(input));
    match res{
        Err(..)=> {
            println!("Error setting role. Please choose one of the possible roles.");
            return retry_read_role(prompt);
        }
        Ok(..)=>{
            res.unwrap()
        }
    }
}

fn ask_for_confirmation()->Result<(),Error>{
    let mut input = String::new();
    println!("Y[es]/N[o]");
    stdin().read_line(&mut input).expect("Error reading from terminal");
    match input.to_lowercase().starts_with("y") {
        true=>Ok(()),
        false=>Err(Error::new(ErrorKind::WouldBlock, "Interrupted by user."))
    }
}


fn trim_string(string_to_trim: String)->String{
    string_to_trim.trim_end_matches("\n").trim().parse().unwrap()
}


fn do_user_update(mut user:User){
    let mut input = String::new();
    println!("The following settings of a user should be updated: {:?}",user);
    println!("Enter which field of a user should be updated [role, password, \
    explicit_consent]");
    stdin().read_line(&mut input)
        .expect("Error reading from terminal");
    input = trim_string(input);
    match input.as_str() {
        "role" =>{
            user.role = Role::to_string(&retry_read_role("Enter the new role [user,admin]"));
            User::update_user(user, &mut establish_connection())
                .expect("Error updating role");
            println!("Role updated");
        },
        "password"=>{
            let mut password = retry_read_secret("Enter the new username");
            password = digest(password);
            user.password = Some(password);
            User::update_user(user, &mut establish_connection())
                .expect("Error updating username");
            println!("Password updated");
        },
        "explicit_consent"=>{
            user.explicit_consent = !user.explicit_consent;
            User::update_user(user, &mut establish_connection())
                .expect("Error updating explicit_consent");
            println!("Explicit consent updated");
        }
        _=>{
            println!("Field not found");
        }
    }

}