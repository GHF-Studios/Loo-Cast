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
            public Game[] GamesArray => gamesArray;

            [SerializeField] private Game[] gamesArray;

            public DataContainer(Game[] gamesArray)
            {
                this.gamesArray = gamesArray;
            }
        }

        private DataContainer serializableRuntimeData
        {
            get
            {
                return new DataContainer(gamesArray);
            }

            set
            {
                gamesArray = value.GamesArray;
            }
        }
        #endregion

        #region Properties
        public Game[] GamesArray
        {
            get
            {
                return gamesArray;
            }
        }
        #endregion

        #region Fields
        [SerializeField] private Game[] gamesArray;
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
            foreach (Game game in gamesArray)
            {
                if (game.Name == gameName)
                {
                    return game;
                }
            }
            return null;
        }

        public void AddGame(Game game)
        {
            List<Game> gamesList = gamesArray.ToList();
            gamesList.Add(game);
            gamesArray = gamesList.ToArray();
            Save();
        }

        public void RemoveGame(Game game)
        {
            List<Game> gamesList = gamesArray.ToList();
            gamesList.Remove(game);
            gamesArray = gamesList.ToArray();
            Save();
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
