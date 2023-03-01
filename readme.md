# Memocfg
In migrating my server (which I give access to some people) a need for a tool that would do basic user management (ports, and services) was needed. Since NIH runs through my veins, instead of looking online for an existing solution. I decided to write one myself!


## High level overview
The idea behind this is that, each user should have their own "contained" env. While also not going into things like nspawn / docker. While also allowing for seperation of concerns.

This is tackled in 2 ways (currently, there might be more in the future!) 
1) Port range restrictions - Restrict a given user to a specified port range
2) User per project - Each project on the machine will have it's own dedicated user

## Rough implementation details

> #### Port range restrictions
> This is implemented by setting iptable rules for a group created for each user, the group follows the pattern of `memocfg-<username>`, these groups are also reused for user per project (more details below)

> #### User per project
> When a user creates a new project, a new user is created with the name being `<username>-<project-name>` (Note: In the project listing, the `<username>` portion is omitted for brevity). This new user is assigned the same group as the original user account, ensuring that the same port restrictions are placed.

> #### File storage
> There is a single file used by memocfg, it's stored at /var/lib/memocfg/data.json

## Usage examples (No sudo)

### Check registered users
```
$ memocfg users [list]
[Registered users]
User1
User2
User3
```

### Check ports
```
$ memocfg ports [list]
[Public ports]
21 - FTP
22 - SSH
25 - SMTP
80 - HTTP
443 - HTTPS

[Internal port ranges]
2001:2500 - @User1
2501:3000 - @User2
3001:3500 - @User3
```

### Check projects
```
$ memocfg projects [list]
[@User1]
basic-backend
website-frontend
game-server

[@User2]
discord-bot
voxel-server

[@User3]
personal-website
```

## Usage examples (sudo)
### Change port range for a given user 
```
$ memocfg ports set User1 --range 2001:3000
Updated the port restrictions for [User1]
```

### Register a new user 
```
$ memocfg users register User4 --range 3501:4000
```
