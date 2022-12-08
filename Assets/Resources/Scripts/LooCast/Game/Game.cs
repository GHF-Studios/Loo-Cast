using UnityEngine;
using System;
using System.IO;

namespace LooCast.Game
{
    using Universe;
    using Core;

    [Serializable]
    public class Game
    {
        #region Data

        #endregion

        #region Static Properties
        private string applicationDataPath = Application.dataPath;
        #endregion

        #region Properties
        public string Name
        {
            get
            {
                return name;
            }
        }
        public Universe CurrentUniverse
        {
            get
            {
                return currentUniverse;
            }
        }
        public string DataPath
        {
            get
            {
                return $"{applicationDataPath}/Data/Games/{name}";
            }
        }
        #endregion

        #region Fields
        [SerializeField] private string name;
        [SerializeField] private Universe currentUniverse;
        #endregion

        public Game(string name)
        {
            this.name = name;
        }

        public void GenerateUniverse(Universe.GenerationSettings generationSettings)
        {
            if (currentUniverse != null)
            {
                throw new Exception("Universe is already generated!");
            }

            currentUniverse = Universe.GenerateUniverse(generationSettings);
            currentUniverse.Initialize();
            SaveGame(this);
        }

        public static void SaveGame(Game game)
        {
            if (game == null)
            {
                throw new NullReferenceException("Game is null!");
            }

            string path = $"{game.DataPath}/Game.json";
            string json = JsonUtility.ToJson(game, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter streamWriter = new StreamWriter(path);
            streamWriter.Write(json);
        }

        public static Game LoadGame(string gameName)
        {
            if (!GameExists(gameName))
            {
                throw new FileNotFoundException("Game does not exist!");
            }

            string path = $"{Application.dataPath}/Data/{gameName}/Game.json";
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            return JsonUtility.FromJson<Game>(json);
        }

        public static void DeleteGame(Game game)
        {
            string path = $"{game.DataPath}/Game.json";
            if (File.Exists(path))
            {
                File.Delete(path);
            }
            MainManager.Games.RemoveGame(game.Name);
        }

        public static void Rename(Game game, string newName)
        {
            if (!GameExists(game.Name))
            {
                throw new FileNotFoundException("Game does not exist!");
            }

            string oldPath = $"{game.DataPath}";
            string newPath = $"{Application.dataPath}/Data/{newName}";
            MainManager.Games.RemoveGame(game.Name);
            game.name = newName;
            Directory.Move(oldPath, newPath);
            MainManager.Games.AddGame(game.Name);
        }

        public static bool GameExists(string gameName)
        {
            string directoryPath = $"{Application.dataPath}/Data/{gameName}";
            string filePath = $"{Application.dataPath}/Data/{gameName}/Game.json";
            return Directory.Exists(directoryPath) && File.Exists(filePath);
        }
    }
}
