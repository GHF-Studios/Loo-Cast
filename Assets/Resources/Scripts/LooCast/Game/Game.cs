using UnityEngine;
using System;
using System.IO;
using System.Collections.Generic;

namespace LooCast.Game
{
    [Serializable]
    public class Game
    {
        #region Data

        #endregion

        #region Properties
        public string Name
        {
            get
            {
                return name;
            }
        }
        public bool Initialized
        {
            get
            {
                return initialized;
            }
        }
        #endregion

        #region Fields
        [SerializeField] private string name;
        #endregion

        public Game(string name)
        {
            this.name = name;
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
    }
}
