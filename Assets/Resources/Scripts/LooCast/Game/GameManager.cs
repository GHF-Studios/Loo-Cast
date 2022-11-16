using System.Collections;
using System.Collections.Generic;
using UnityEngine;

namespace LooCast.Game
{
    using LooCast.Data.Runtime;
    using LooCast.UI.Screen;
    using LooCast.Sound;
    using LooCast.Core;
    using LooCast.Statistic;
    using System;
    using LooCast.Util;

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
        private void OnApplicationQuit()
        {
            runtimeSets.Initialize();
        }
        #endregion

        #region Methods
        public void Initialize()
        {
            if (Instance != null)
            {
                throw new Exception("Cannot have multiple instances of GameManager!");
            }

            #region Initialization
            Instance = this;
            runtimeSets.Initialize();
            IsPaused = false;
            KillsStatistic.Kills = 0;
            currentGame = null;
            #endregion

            Debug.Log($"[GameManager] Initialized.");
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

        public static void InitializeGame()
        {
            if (Initialized)
            {

            }
        }

        [Serializable]
        public struct Identifier
        {
            public string DataFilePath
            {
                get
                {
                    return dataFilePath;
                }
            }

            private string dataFilePath;

            public Identifier(params string[] typeIdentifierCompositeString)
            {
                if (typeIdentifierCompositeString.Length < 2)
                {
                    throw new ArgumentException("An identifier must atleast be ");
                }
                dataFilePath = "";
                for (int i = 1; i < typeIdentifierCompositeString.Length; i++)
                {
                    dataFilePath += ""
                }
            }
        }

        public struct PersistentDataDictionaryEntry<DataObjectType> where DataObjectType : IGameDataHandler
        {
            public PersistentDataDictionaryEntry(int dataObjectInstanceID, Identifier dataIdentifier)
            {

            }

            private string GetJsonSerializedData()
            {
                
            }

            public PersistentData<IGameDataHandler> GetPersistentData()
            {
                return JSONUtil.LoadData<PersistentData<Player.Player>>(dataIdentifier.DataFilePath);
            }
        }

        public void DoShit()
        {
            
            Dictionary<Identifier, PersistentData<IGameDataHandler>> persistentDataDictionary;
        }

        [Serializable]
        public class PersistentData<ObjectType> where ObjectType : IGameDataHandler
        {
            [SerializeField] private string jsonSerializedData;
            [SerializeField] private Type objectType;
            [SerializeField] private string objectPrefabPath;

            private PersistentData(ObjectType _object)
            {
                
            }

            public PersistentData<ObjectType> CreatePersistentDataFromGameObject(ObjectType _object)
            {
                RuntimeData gameData = ((IGameDataHandler)_object).GetData();
            }

            public RuntimeData CreateGameDataFromPersistentData(PersistentData<ObjectType> persistentData)
            {
                ((IGameDataHandler)_object).SetData();
            }

            public void LoadDataIntoGameObject
            {

            }

            public void CreateGameObjectFromData(PersistentData<ObjectType> persistentData)
            {
                persistentData.LoadObject();
            }

            public GameObject LoadGameObject()
            {
                UnityEngine.Object.Instantiate(Resources.Load<GameObject>(objectPrefabPath)); 
            }

            public void SaveObject(GameObject gameObject)
            {

            }
        }
        #endregion
    }
}
