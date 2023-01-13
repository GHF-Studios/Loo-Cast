using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Game
{
    using Data.Runtime;
    using UI.Screen;
    using Sound;
    using Core;
    using Statistic;
    using System;
    using Util;
    using Universe;
    using System.Xml.Linq;

    public class GameManager : MonoBehaviour
    {
        #region Static Properties
        public static GameManager Instance { get; private set; }
        public static bool Initialized
        {
            get
            {
                return Instance != null;
            }
        }
        #endregion

        #region Static Fields
        private static Queue<Action> postInitializationActionQueue;
        #endregion

        #region Properties
        public bool IsPaused { get; private set; }
        public Game CurrentGame
        {
            get
            {
                return currentGame;
            }

            private set
            {
                currentGame = value;
            }
        }
        #endregion

        #region Fields
        public LoadingScreen loadingScreen;
        public RuntimeSets runtimeSets;
        public GameSoundHandler gameSoundHandler;

        private Game currentGame;
        #endregion

        #region Unity Callbacks
        private void Awake()
        {
            if (Instance != null)
            {
                throw new Exception("Cannot have multiple instances of GameManager!");
            }

            #region Initialization
            
            #region Game Manager Initialization
            Instance = this;
            runtimeSets.Initialize();
            IsPaused = false;
            KillsStatistic.Kills = 0;
            Debug.Log($"[GameManager] Initialized.");
            #endregion

            #region Game Manager Post-Initialization
            if (postInitializationActionQueue != null)
            {
                while (postInitializationActionQueue.Count > 0)
                {
                    Action postInitializationAction = postInitializationActionQueue.Dequeue();
                    postInitializationAction.Invoke();
                }
            }
            Debug.Log($"[GameManager] Post-Initialized.");
            #endregion

            #endregion
        }

        private void OnApplicationQuit()
        {
            Game.Save(currentGame);
            runtimeSets.Initialize();
        }
        #endregion

        #region Static Methods
        public static void PauseGame()
        {
            if (Instance == null)
            {
                return;
            }
            if (!Instance.IsPaused)
            {
                Instance.IsPaused = true;
                foreach (ExtendedMonoBehaviour extendedMonoBehaviour in ExtendedMonoBehaviour.Instances)
                {
                    extendedMonoBehaviour.Pause();
                }
            }
        }

        public static void ResumeGame()
        {
            if (Instance == null)
            {
                return;
            }
            if (Instance.IsPaused)
            {
                Instance.IsPaused = false;
                foreach (ExtendedMonoBehaviour extendedMonoBehaviour in ExtendedMonoBehaviour.Instances)
                {
                    extendedMonoBehaviour.Resume();
                }
            }
        }

        public static void LoadGame(Game game)
        {
            if (!Initialized)
            {
                throw new Exception("Cannot load Game when GameManager is not initialized!");
            }
            if (Instance.CurrentGame != null)
            {
                if (Instance.CurrentGame == game)
                {
                    throw new Exception("Cannot load Game because it is already loaded!");
                }
                throw new Exception("Cannot load Game when another Game is already loaded!");
            }

            Instance.CurrentGame = game;

            // TODO: Load all Chunks inside range at player position into Scene
        }

        public static void SaveGame()
        {
            if (!Initialized)
            {
                throw new Exception("Cannot save Game when GameManager is not initialized!");
            }
            if (Instance.CurrentGame == null)
            {
                throw new Exception("Cannot save Game when no Game is loaded!");
            }

            Game.Save(Instance.currentGame);

            // TODO: Save all loaded Chunks
        }

        public static void AddPostInitializationAction(Action postInitializationAction)
        {
            if (postInitializationAction == null)
            {
                return;
            }

            if (postInitializationActionQueue == null)
            {
                postInitializationActionQueue = new Queue<Action>();
            }

            postInitializationActionQueue.Enqueue(postInitializationAction);
        }
        #endregion

        #region Methods

        public void InitializeGame(string newGameName)
        {
            InitializeGame(newGameName, Universe.DefaultGenerationSettings);
        }

        public void InitializeGame(string newGameName, Universe.GenerationSettings generationSettings)
        {
            Game game = new Game(newGameName);
            game.Initialize();
            MainManager.Games.AddGame(newGameName);
            LoadGame(game);
            SaveGame();
            game.GenerateUniverse(generationSettings);
            Universe.MapElementLoadingUtil.InitializeInstance();

            Debug.Log($"[GameManager] Initialized Game.");
        }

        public void InitializeGame(Game game)
        {
            game.Initialize();
            LoadGame(game);

            Debug.Log($"[GameManager] Initialized Game.");
        }
        #endregion
    }
}
