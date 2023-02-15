using System;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Game
{
    using Data.Runtime;
    using UI.Screen;
    using Sound;
    using Statistic;
    using Universe;
    using Scene;

    public class GameManager : ModuleManager
    {
        #region Static Properties
        public static GameManager Instance
        {
            get
            {
                if (instance == null)
                {
                    GameObject instanceObject = new GameObject("[GameManager]");
                    instanceObject.layer = 31;
                    instanceObject.tag = "INTERNAL";
                    instanceObject.transform.parent = Core.CoreManager.Instance.transform;
                    return instanceObject.AddComponent<GameManager>();
                }
                else
                {
                    return instance;
                }
            }
        }
        #endregion

        #region Static Fields
        private static GameManager instance;
        #endregion

        #region Properties
        public bool IsPaused { get; private set; }
        public Game CurrentGame => currentGame;
        public Games Games => games;
        public Game GameToBeLoaded => gameToBeLoaded;    // TODO: Implement this
        #endregion

        #region Fields
        public LoadingScreen loadingScreen;
        public RuntimeSets runtimeSets;
        public GameSoundHandler gameSoundHandler;

        private Game currentGame;
        private Games games;
        private Game gameToBeLoaded;
        #endregion

        #region Static Methods
        public override void InitializeInstance()
        {
            base.InitializeInstance();

            runtimeSets.Initialize();
            IsPaused = false;
            KillsStatistic.Kills = 0;
        }
        #endregion

        #region Unity Callbacks
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
            if (Instance.CurrentGame != null)
            {
                if (Instance.CurrentGame == game)
                {
                    throw new Exception("Cannot load Game because it is already loaded!");
                }
                throw new Exception("Cannot load Game when another Game is already loaded!");
            }

            Instance.currentGame = game;

            // TODO: Load all Chunks inside range at player position into Scene
        }

        public static void SaveGame()
        {
            if (Instance.CurrentGame == null)
            {
                throw new Exception("Cannot save Game when no Game is loaded!");
            }

            Game.Save(Instance.currentGame);

            // TODO: Save all loaded Chunks
        }
        #endregion

        #region Methods
        public void CreateNewGame(string gameName)
        {
            if (games.Contains(gameName))
            {
                throw new Exception("Cannot create new Game, because another Game with the same Name already exists!");
            }

            SceneManager.Instance.LoadScene(SceneManager.SceneType.Game, () =>
            {
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(gameName);
            });
        }

        public void CreateNewGame(string gameName, Universe.GenerationSettings generationSettings)
        {
            if (games.Contains(gameName))
            {
                throw new Exception("Cannot create new Game, because another Game with the same Name already exists!");
            }

            SceneManager.Instance.LoadScene(SceneManager.SceneType.Game, () =>
            {
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(gameName, generationSettings);
            });
        }

        public void LoadGame(string gameName)
        {
            if (!games.Contains(gameName))
            {
                throw new Exception("Cannot load Game, because it does not exist!");
            }

            SceneManager.Instance.LoadScene(SceneManager.SceneType.Game, () =>
            {
                Game game = games.GetGame(gameName);
                GameManager gameManager = FindObjectOfType<GameManager>();
                gameManager.InitializeGame(game);
            });
        }

        public void DeleteGame(string gameName)
        {
            if (CurrentGame.Name == gameName)
            {
                throw new Exception("Cannot delete Game when it is loaded!");
            }

            Game game = games.GetGame(gameName);
            Game.DeleteGame(game);
        }

        public void RenameGame(string oldGameName, string newGameName)
        {
            if (CurrentGame.Name == oldGameName)
            {
                throw new Exception("Cannot rename Game when it is loaded!");
            }

            Game game = games.GetGame(oldGameName);
            Game.Rename(game, newGameName);
        }

        public void InitializeGame(string newGameName)
        {
            InitializeGame(newGameName, Universe.DefaultGenerationSettings);
        }

        public void InitializeGame(string newGameName, Universe.GenerationSettings generationSettings)
        {
            Game game = new Game(newGameName);
            game.Initialize();
            Games.AddGame(newGameName);
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

        #region Overrides
        public override void PreInitializeInstance()
        {
            base.PreInitializeInstance();

            #region Namespace/Type/Instance Registration
            NamespaceManager namespaceManager = NamespaceManager.Instance;
            TypeManager typeManager = TypeManager.Instance;
            InstanceManager instanceManager = InstanceManager.Instance;

            Namespace rootNamespace = namespaceManager.GetNamespace("LooCast");
            looCastNamespace = new Namespace("Game", rootNamespace);
            looCastType = new Type(typeof(GameManager), looCastNamespace);
            looCastInstance = new Instance(this, looCastType);

            namespaceManager.RegisterNamespace(looCastNamespace);
            typeManager.RegisterType(looCastType);
            instanceManager.RegisterInstance(looCastInstance);

            Type gameType = new Type(typeof(Game), looCastNamespace);
            Type gamesType = new Type(typeof(Games), looCastNamespace);

            typeManager.RegisterType(gameType);
            typeManager.RegisterType(gamesType);
            #endregion
        }
        #endregion
    }
}
