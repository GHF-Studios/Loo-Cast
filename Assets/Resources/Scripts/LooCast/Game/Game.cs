using UnityEngine;
using System;
using System.Collections;
using System.Collections.Generic;
using System.IO;

namespace LooCast.Game
{
    using Universe;
    using Core;
    using Attribute;
    using Attribute.Stat;
    using Currency;
    using LooCast.Data;
    using UnityEditor;

    [Serializable]
    public class Game
    {
        #region Data

        #endregion

        #region Static Properties
        public static string CurrentRelativeDataPath
        {
            get
            {
                return GameManager.Instance.CurrentGame.RelativeDataPath;
            }
        }
        public static string CurrentAbsoluteDataPath
        {
            get
            {
                return GameManager.Instance.CurrentGame.AbsoluteDataPath;
            }
        }
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
        public string RelativeDataPath
        {
            get
            {
                return $"Games/{name}/";
            }
        }
        public string AbsoluteDataPath
        {
            get
            {
                return Path.Combine(Data.Path, RelativeDataPath);
            }
        }
        #endregion

        #region Fields
        [SerializeField] private string name;
        [SerializeField] private Universe currentUniverse;
        
        private List<DynamicGameData> dynamicDataList;
        #endregion

        #region Constructors
        public Game(string name)
        {
            this.name = name;
        }
        #endregion

        #region Methods
        public void Initialize()
        {
            dynamicDataList = new List<DynamicGameData>
            {
                Resources.Load<Attributes>("Data/Dynamic/Attribute/Attributes"),
                Resources.Load<Stats>("Data/Dynamic/Attribute/Stat/Stats"),
                Resources.Load<Coins>("Data/Dynamic/Currency/Coins"),
                Resources.Load<Tokens>("Data/Dynamic/Currency/Tokens"),
                Resources.Load<Credits>("Data/Dynamic/Currency/Credits")
            };
        }

        public void GenerateUniverse(Universe.GenerationSettings generationSettings)
        {
            if (currentUniverse != null)
            {
                throw new Exception("Universe is already generated!");
            }

            currentUniverse = Universe.GenerateUniverse(generationSettings);
            currentUniverse.Initialize();
            Save(this);
        }


        private void SaveDynamicData()
        {
            foreach (DynamicGameData dynamicData in dynamicDataList)
            {
                dynamicData.Save();
            }
        }

        private void LoadDynamicData()
        {
            foreach (DynamicGameData dynamicData in dynamicDataList)
            {
                try
                {
                    dynamicData.Load();
                }
                catch (Exception e)
                {
                    Debug.LogError($"Error when trying to load dynamic data '{dynamicData.name}': {e.Message}! Trying to load default...");
                    dynamicData.LoadDefault();
                }
                EditorUtility.SetDirty(dynamicData);
            }
        }
        #endregion

        #region Static Methods
        public static void Save(Game game)
        {
            if (game == null)
            {
                throw new NullReferenceException("Game is null!");
            }

            string path = Path.Combine($"{game.AbsoluteDataPath}", "Game.dat");
            string json = JsonUtility.ToJson(game, true);
            Directory.CreateDirectory(Path.GetDirectoryName(path));
            using StreamWriter streamWriter = new StreamWriter(path);
            streamWriter.Write(json);
            game.SaveDynamicData();
        }

        public static Game Load(string gameName)
        {
            if (!GameExists(gameName))
            {
                throw new FileNotFoundException("Game does not exist!");
            }

            string path = Path.Combine(Data.Path, $"Games/{gameName}/Game.dat");
            using StreamReader reader = new StreamReader(path);
            string json = reader.ReadToEnd();
            Game game = JsonUtility.FromJson<Game>(json);
            game.LoadDynamicData();
            return game;
        }

        public static void DeleteGame(Game game)
        {
            string path = Path.Combine($"{game.AbsoluteDataPath}", "Game.dat");
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

            string oldPath = Path.Combine(Data.Path, game.RelativeDataPath);
            string newPath = Path.Combine(Data.Path, $"Games/{newName}");
            MainManager.Games.RemoveGame(game.Name);
            game.name = newName;
            Directory.Move(oldPath, newPath);
            MainManager.Games.AddGame(game.Name);
        }

        public static bool GameExists(string gameName)
        {
            string directoryPath = Path.Combine(Data.Path, $"Games/{gameName}/");
            string filePath = Path.Combine(Data.Path, $"Games/{gameName}/Game.dat");
            return Directory.Exists(directoryPath) && File.Exists(filePath);
        }
        #endregion
    }
}
