# Auction Application

Building the Application using Rust with Axum the things used in this application was Rest-api , websockets, Graph-ql and Postgress for Primary Storage and Redis for Speed access of the real-time data.

## Below is the Application Flow

![Activity Diagram](assets/activity.jpg)


## Back-End Routes

Authentication-Routes:

    /authentication/login - Post Url encoded - username and password
    /authentication/sign-up Post Url encoded - mail_id , username and password
    /authentication/forget-password same but only mail_id

My Next thing was to check whether all Rest API's were implemented or not
along with handlers by cross checking with application sequence diagram.

In Home Page

    -> public-rooms -> /rooms/get-public-rooms
    -> create-room -> ws://localhost:9090/ [websocket connection will be created here]
    -> join-room -> ws://localhost:9090/ [or here the websocket connection will be created]
    -> search/:username -> for live username search
    -> profile -> user/:username returns profile
    -> reset-password inside profile -> user/reset-password
In Profile Page:

        Each auction will be listed played by the user. where user can click on the auctions played by them and the list of teams played and the owners(usernames) of the teams will be appeared and each user can click on the team that he wants to see .
        -> rooms/get-teams/{room_id} return teams that participated in the auction along with owners usernames(owners are nothing but users).
        -> rooms/get-team/{room_id}/{team_name} returns team that the players bought by them (only player-id and player name and amount bought for will be returned).
        To get in-detailed player details, the below should be accessed
        -> player/get-player/:player_id (it returns everything except stats,it included stats_id)
        -> player/get-stats/:stats_id (we can get back stats from the stats_id).
        -> we can also get unsold players of that auction as well.
    As this is all we can do in Profile page.


These are all the main-routes and below are the routes that are designed for
inside room-coomunication.

Room Communication with Actual Biddings:

    -> Once every one joined room then, the room-creator can send Ready string via websocket and websocket response with the First Player.
    (with player_id,player_name and base_price).

    -> bidding will be taken place, each bid will be sent to the server via websocket and websocket checks whether user has money to buy 18 players after this bid and then adds the bid to the redis and broadcast to the remaining room.

    -> once 10 seconds waiting team is over the last bid will taken to consideration and sold to that team and broadcasted to all the room.
    if there is no bid , the player will be sent to unsold.

    -> once players has been completed, the Auction room will be ended.


Inside room apart from biddings

    (yet to write routes)
    -> get unsold players using graph ql

    -> get players list by pool  (/rooms/get-pool/{pool_id} -> returns players name, player_id, base_price)

    -> get each team bought players -> (/rooms/get-team/{room_id}/{team_name})

    -> we can add the players in to intrested players list , such that
       those will be re-visited again after the each team has completed buying 16 players. (/rooms/add-to-intrested/{player_id} so at last all players intrested players whom are unsold are not yet came will be sent to the server and stored in the redis).

    -> once each team has completed buying 16 players, the room-creator
    can send a request that continue with the following intrested players instead of the auction-list such that auction will be completed faster.
    ws://localhost:9090/ with all intrested players list in a hash-set to be sent to the websocket.


In Room Bidding

    -> yet to write logic for bidding
