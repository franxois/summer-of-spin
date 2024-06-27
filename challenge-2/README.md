# Challenge 2
Inspired from [Challenge 3 of Advent of Spin 2023](https://github.com/fermyon/advent-of-spin/blob/main/2023/Challenge-3/README.md
)



## Idea - 1
### Spec
It's summer time! It's all about vacations and enjoying the beautiful weather outdoors. Finally, a group of friends found time to on a trip that they have been putting off for months, Alas! they don't want to deal with planning every part of the trip. Your task is to utilize the LLM powers to help people build a perfect itinerary to enjoy the season. This must be stored in a key-value store for people to refer later as they go on this trip! 

The actual output of the model is not assessed!

The users will be able to give some input such as destination, duration, type of trip and number of people. Take the help of LLMs to plan the perfect getaway! The users will also provide a unique `tag` to get the generated result later.

The users will send a POST reques to `/plan-my-trip` in JSON format.
```json
{
    "tag": "week-in-banff"
    "destination": "Banff",
    "duration": "1 week",
    "num_people": "four",
    "type": ["hiking", "camping", "long drive", "food"]
    
}
```

the return type must be a `201` response with the itinerary as follows:
```json
{
    "itinerary" : <GENERATED ITINERARY>
}
```
The header type must contain `Content-Type: application/json`

For referece, the users will `GET` request to `\<tag>` from on the previous post reqeust mentioned to see the last generated itenrary. Consider storing the generated prompt in a KV Store to and return when requested with a `200` code in the following format
```json
{
"tag": <GENERATED ITINERARY>
}
```

Don't forget to put your spinscreen and all the best! :D


## Idea - 2
### Spec


With the summer here people are looking for ways to spend more time outdoors! Golf is a great game to play with your friends or family and have fun with the ball. A local golf club needs your coding help! They w
Golf keep track and cheer them up with a encouraging slogan using the LLM


Note: We aren't using the concept of handicaps to keep things simple :D

Your application should do three things:
1) Start the application with the following course information loaded in the Key Value Store.
This is the example template of score card of 9 hole game used in golf, basically the players ideally want to hit the ball into the respective hole in as low pars as possible.
```json
{
 "scoreCard": {
    "courseName": "Fermi Golf Club",
    "yards": [400, 350, 385, 170, 530, 215, 365, 495, 390],
    "pars": [4, 4, 4, 3, 5, 3, 4, 5, 4]
  }
}
```
2) When /start-game with a POST with player names and a game title

```json
{
  "gameTitle": "Summer Golf Challenge",
  "players": ["Scorpion", "Charmandar", "Bulbasaur", "Pikachu"]
}
```

this should create the game under the key "games"

```json
{
    "games" : {
        "Summer Golf Challenge": {
            "courseName": "Summer Greens Golf Club",
    
            "players": {
            "Scorpion": {"scores": [-1, -1, -1, -1, -1, -1, -1, -1, -1]},
            "Pikachu": {"scores": [-1, -1, -1, -1, -1, -1, -1, -1, -1]},
            },
      "currentHole": 1,
            }
        }
}
```
3) Now the players can update the scores with the following POST to `\update-game` reqeust with the pars score of each player

```json
{
    "gameName" : "Summer Golf Challenge",
    "currentHole": <NUMBER BETWEEN 1 to 9>,
    "Scorpion": 2,
    "Pikachu": 4,
}
```

4) In the end when score of a game is requested through a \GET request to `\game-status` with a `{ "gameName" : <GAME NAME>}` use the LLM to encourage and send positive words back about the game or a particular player. For example: praise on Pikachu hitting great holes (if a player hit the hole in few pars than mentioned in "scoreCard").

```json
{
    "status" : "<YOUR PRAISE>",
    "gameInfo" :  {
            "<GAME NAME>": "...<all information as shown in step 2 example with udpated score>",
            }
    
    
}
```