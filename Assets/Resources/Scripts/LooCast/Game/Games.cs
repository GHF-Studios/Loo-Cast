using LooCast.Core;
using LooCast.Data.Runtime;
using LooCast.Util;
using System;
using System.IO;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;
using LooCast.Util.Collections.Generic;
using JetBrains.Annotations;

namespace LooCast.Game
{
    [Serializable]
    public class Games
    {
        #region Properties
        #endregion

        #region Fields
        [SerializeField] private SerializableList<string> gameNames;
        #endregion

        #region Constructors
        public Games()
        {
            gameNames = new SerializableList<string>();
        }
        #endregion

        #region Methods
        public Game GetGame(string gameName)
        {
            if (gameNames.Contains(gameName))
            {
                return Game.LoadGame(gameName);
            }
            return null;
        }

        public void AddGame(string gameName)
        {
            gameNames.Add(gameName);
            Save(this);
        }

        public void RemoveGame(string gameName)
        {
            gameNames.Remove(gameName);
            Save(this);
        }

        public bool Contains(string gameName)
        {
            if (gameNames.Count == 0)
            {
                return false;
            }
            return gameNames.Contains(gameName);
        }

        public static void Save(Games games)
        {
            string relativeDataPath = $"Games/Games.json";
            JSONUtil.SaveData(games, relativeDataPath);
        }

        public static Games Load()
        {
            string relativeDataPath = $"Games/Games.json";
            string dataPath = $"{Application.dataPath}/Data/{relativeDataPath}";
            Games games;
            if (!File.Exists(dataPath))
            {
                games = new Games();
                Save(games);
            }
            else
            {
                games = JSONUtil.LoadData<Games>(relativeDataPath);
            }
            return games;
        }
        #endregion
    }
}
