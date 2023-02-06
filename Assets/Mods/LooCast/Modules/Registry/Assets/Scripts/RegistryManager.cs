using System;
using System.Collections.Generic;

namespace LooCast.Registry
{
    using Core;
    
    public class RegistryManager
    {
        #region Static Properties
        public static RegistryManager Instance
        {
            get
            {
                if (instance == null)
                {
                    instance = new RegistryManager();
                }
                return instance;
            }
        }
        #endregion

        #region Static Fields
        private static RegistryManager instance;
        #endregion

        #region Fields
        private Dictionary<string, object> registries;
        #endregion

        #region Methods
        public void RegisterRegistry<T>(IRegistry<T> registry)
        {
            if (!MainManager.IsPreInitializing)
            {
                throw new Exception("[RegistryManager] Registries can only be registered in Pre-Initialization!");
            }
            if (registries.ContainsKey(registry.ID))
            {
                throw new Exception($"[RegistryManager] Registry '{registry.ID}' already exists!");
            }
            
            registries.Add(registry.ID, registry);
        }

        public IRegistry<T> GetRegistry<T>(string registryID)
        {
            if (!MainManager.IsPreInitialized)
            {
                throw new Exception("[RegistryManager] Registries can only be accessed after Pre-Initialization!");
            }
            if (!registries.ContainsKey(registryID))
            {
                throw new Exception($"[RegistryManager] Registry '{registryID}' does not exist!");
            }
            
            return (IRegistry<T>)registries[registryID];
        }

        internal void Initialize()
        {
            registries = new Dictionary<string, object>();
        }
        #endregion
    }
}
