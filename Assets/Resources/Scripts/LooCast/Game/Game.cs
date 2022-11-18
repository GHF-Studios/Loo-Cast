using UnityEngine;
using System;
using System.IO;
using System.Collections.Generic;
using LooCast.Core;
using UnityEngine.UIElements;

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
        #endregion

        #region Fields
        [SerializeField] private string name;
        [SerializeField] private IInstanceIdentifierProvider[] objectInstances;
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

            string path = $"{Application.dataPath}/Data/{game.name}/Game.json";
            string json = JsonUtility.ToJson(game, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter streamWriter = new StreamWriter(path);
            streamWriter.Write(json);
        }

        public static Game LoadGame(string gameName)
        {
            string path = $"{Application.dataPath}/Data/{gameName}/Game.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            return JsonUtility.FromJson<Game>(json);
        }

        public static void DeleteGame(Game game)
        {
            string path = $"{Application.dataPath}/Data/{game.name}/Game.json";
            if (File.Exists(path))
            {
                File.Delete(path);
            }
            MainManager.Games.RemoveGame(game);
        }

        public static void Rename(Game game, string newName)
        {
            string oldPath = $"{Application.dataPath}/Data/{game.name}/Game.json";
            string newPath = $"{Application.dataPath}/Data/{newName}/Game.json";
            MainManager.Games.RemoveGame(game);
            game.name = newName;
            Directory.Move(oldPath, newPath);
            MainManager.Games.AddGame(game);
        }
    }
}
