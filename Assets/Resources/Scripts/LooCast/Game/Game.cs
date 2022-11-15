using UnityEngine;
using System;
using System.IO;

namespace LooCast.Game
{
    public class Game
    {
        public string Name
        {
            get
            {
                return name;
            }

            private set
            {
                name = value;
            }
        }

        private string name;

        private Game(string name)
        {
            Name = name;
        }

        public static void SaveGame(Game game)
        {
            if (game == null)
            {
                throw new NullReferenceException("Game is null!");
            }

            string path = $"{Application.dataPath}/Data/{game.Name}/Game.json";
            string json = JsonUtility.ToJson(game, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter streamWriter = new StreamWriter(path);
            streamWriter.Write(json);
        }

        public static Game LoadGame(string name)
        {
            string path = $"{Application.dataPath}/Data/{name}/Game.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            return JsonUtility.FromJson<Game>(json);
        }

        public static Game CreateGame(string name)
        {
            return new Game(name);
        }
    }
}
