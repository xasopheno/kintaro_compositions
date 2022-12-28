papertrail --min-time 'Dec 21, 2022 at 1am' --max-time 'Dec 22, 2022 at 1am' --json  ecs/prod/svc-social createHome > homes.json &

papertrail --min-time 'Dec 21, 2022 at 1am' --max-time 'Dec 22, 2022 at 1am' --json  ecs/prod/svc-social createRoom > rooms.json &

papertrail --min-time 'Dec 21, 2022 at 1am' --max-time 'Dec 22, 2022 at 1am' --json  ecs/prod/svc-social acceptFriendRequest > friends.json &

papertrail --min-time 'Dec 21, 2022 at 1am' --max-time 'Dec 22, 2022 at 1am' --json  ecs/prod/svc-messaging sendMessageMutation > messages.json

