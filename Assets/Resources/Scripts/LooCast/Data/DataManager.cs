using System;
using UnityEngine;

namespace LooCast.Data
{
    using Registry;
    
    public class DataManager
    {
        #region Static Properties
        public static DataManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new DataManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static DataManager instance;
        #endregion

        #region Fields
        private DataFolder rootDataFolder;
        #endregion

        #region Methods
        public DataFolder GetDataFolder(string dataFolderID)
        {

        }
        
        public DynamicData GetDynamicData(string dataID)
        {
            
        }

        public StaticData GetStaticData(string dataID)
        {

        }

        internal void Initialize()
        {
            
        }

        internal void OnPreInitialize()
        {
            Debug.Log($"[DataManager] Starting Pre-Initialization.");

            SerializerRegistry serializerRegistry = new SerializerRegistry();
            RegistryManager.Instance.RegisterRegistry(serializerRegistry);
            serializerRegistry.Register(new BinarySerializer());
            serializerRegistry.Register(new JSONSerializer());
            serializerRegistry.Register(new PNGSerializer());
            serializerRegistry.Register(new XMLSerializer());

            Debug.Log($"[DataManager] Finished Pre-Initialization.");
        }

        internal void OnInitialize()
        {
            Debug.Log($"[DataManager] Starting Initialization.");
            Debug.Log($"[DataManager] Finished Initialization.");
        }

        internal void OnPostInitialize()
        {
            Debug.Log($"[DataManager] Starting Post-Initialization.");
            Debug.Log($"[DataManager] Finished Post-Initialization.");
        }
        #endregion
    }
}
