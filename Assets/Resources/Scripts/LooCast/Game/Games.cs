using LooCast.Core;
using LooCast.Data.Runtime;
using LooCast.Util;
using System;
using System.Collections.Generic;
using System.Linq;
using UnityEngine;

namespace LooCast.Game
{
    [Serializable]
    public class Games
    {
        #region Data
        private struct DataContainer
        {
            public string[] GamesArray => gamesArray;

            [SerializeField] private string[] gamesArray;

            public DataContainer(string[] gamesArray)
            {
                this.gamesArray = gamesArray;
            }
        }

        private DataContainer serializableRuntimeData
        {
            get
            {
                return new DataContainer(gameNamesArray);
            }

            set
            {
                gameNamesArray = value.GamesArray;
            }
        }
        #endregion

        #region Properties
        public string[] GameNamesArray
        {
            get
            {
                return gameNamesArray;
            }
        }
        #endregion

        #region Fields
        [SerializeField] private string[] gameNamesArray;
        #endregion

        #region Methods
        public static Games LoadGames()
        {
            Games games = new Games();
            games.Load();
            return games;
        }

        public Game GetGame(string gameName)
        {
            if (gameNamesArray.Contains(gameName))
            {
                return Game.LoadGame(gameName);
            }
            return null;
        }

        public void AddGame(string gameName)
        {
            List<string> gameNamesList = gameNamesArray.ToList();
            gameNamesList.Add(gameName);
            gameNamesArray = gameNamesList.ToArray();
            Save();
        }

        public void RemoveGame(string gameName)
        {
            List<string> gameNamesList = gameNamesArray.ToList();
            gameNamesList.Remove(gameName);
            gameNamesArray = gameNamesList.ToArray();
            Save();
        }

        public bool Contains(string gameName)
        {
            if (gameNamesArray == null || gameNamesArray.Count() == 0)
            {
                return false;
            }
            return gameNamesArray.Contains(gameName);
        }

        private void Save()
        {
            JSONUtil.SaveData(serializableRuntimeData, "Games/Games.json");
        }

        private void Load()
        {
            serializableRuntimeData = JSONUtil.LoadData<DataContainer>("Games/Games.json");
        }
        #endregion
    }
}
